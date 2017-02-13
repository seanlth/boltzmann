use collision::*;
use vector::Vector;

use scoped_pool::{Pool, Scope};


pub struct Quadtree {
    pub empty: bool,
    pub level: usize,
    pub radius: f64,
    pub position: Vector,
    pub width: f64,
    pub height: f64,
    pub objects: Vec<(usize, Vector)>,
    pub children: Option<(Box<Quadtree>, Box<Quadtree>, Box<Quadtree>, Box<Quadtree>)>,
    collisions: Vec<Collision>,
    pool: Option<Pool>
}

#[allow(dead_code)]
impl Quadtree {
    pub fn new(width: f64, height: f64, radius: f64) -> Quadtree {
        Quadtree {
            empty: true,
            level: 0,
            radius: radius,
            position: Vector::new(width/2.0, height/2.0),
            width: width,
            height: height,
            objects: vec![],
            children: None,
            collisions: Vec::with_capacity(10000),
            pool: Some(Pool::new(4))
        }
    }
    fn child(level: usize, radius: f64, position: Vector, width: f64, height: f64) -> Quadtree {
        Quadtree {
            empty: true,
            level: level,
            radius: radius,
            position: position,
            width: width,
            height: height,
            objects: vec![],
            children: None,
            collisions: Vec::new(),
            pool: None
        }
    }

    // completely resets the tree
    pub fn reset(&mut self) {
        self.children = None;
        self.objects.clear();
        self.empty == true;
    }


    fn within(&self, p: Vector) -> bool {
        let b1 = p.x+self.radius >= self.position.x - self.width/2.0;
        let b2 = p.x-self.radius <= self.position.x + self.width/2.0;
        let b3 = p.y+self.radius >= self.position.y - self.height/2.0;
        let b4 = p.y-self.radius <= self.position.y + self.height/2.0;

        b1 && b2 && b3 && b4
    }

    // creates children nodes and inserts objects from this node to children
    pub fn divide(&mut self) {
        let p1 = Vector::new(self.position.x - self.width/4.0, self.position.y + self.height/4.0);
        let c1 = Box::new( Quadtree::child(self.level+1, self.radius, p1, self.width/2.0, self.height/2.0) );

        let p2 = Vector::new(self.position.x + self.width/4.0, self.position.y + self.height/4.0);
        let c2 = Box::new( Quadtree::child(self.level+1, self.radius, p2, self.width/2.0, self.height/2.0) );

        let p3 = Vector::new(self.position.x - self.width/4.0, self.position.y - self.height/4.0);
        let c3 = Box::new( Quadtree::child(self.level+1, self.radius, p3, self.width/2.0, self.height/2.0) );

        let p4 = Vector::new(self.position.x + self.width/4.0, self.position.y - self.height/4.0);
        let c4 = Box::new( Quadtree::child(self.level+1, self.radius, p4, self.width/2.0, self.height/2.0) );

        self.children = Some((c1, c2, c3, c4));

        let temp = self.objects.clone();
        self.objects.clear();
        for (i, p) in temp { self.insert(i, p); }
    }

    pub fn print(&self) {
        println!("Level: {}", self.level);
        print!("[ ");
        for &(_, p) in &self.objects { p.print(); print!(" "); };
        if let Some((ref c1, ref c2, ref c3, ref c4)) = self.children {
            c1.print();
            c2.print();
            c3.print();
            c4.print();
        }
        println!(" ]");
    }
    
    fn walk_tree(&self) -> Vec<Collision> {
        let mut collisions = Vec::new();
    
        if let Some((ref c1, ref c2, ref c3, ref c4)) = self.children {
            collisions.append( &mut c1.walk_tree() );
            collisions.append( &mut c2.walk_tree() );
            collisions.append( &mut c3.walk_tree() );
            collisions.append( &mut c4.walk_tree() );
        }
        
        for i in 0..self.objects.len() {
            let (index1, p_position) = self.objects[i];
            // let p_position = particles[index1].get_position();
        
            for j in (i+1)..self.objects.len() {
                let (index2, q_position) =  self.objects[j];
                // let q_position = particles[index2].get_position();
        
                let normal = (q_position - p_position).normalise();
                let penetration = 2.0*self.radius - p_position.distance( q_position );
        
                // if circles are overlapping
                if penetration > 0.0 {
                    // add collision
                    collisions.push( Collision::new(index1, index2, penetration, normal) );
                }
            }
        }
        
        // collisions.sort();
        // collisions.dedup();
    
    
        collisions
    }
    
    fn collect_data<'a>(scoped: &Scope<'a>, cs1: &'a mut Vec<Collision>, c1: &'a Box<Quadtree>, 
            cs2: &'a mut Vec<Collision>, c2: &'a Box<Quadtree>, 
            cs3: &'a mut Vec<Collision>, c3: &'a Box<Quadtree>, 
            cs4: &'a mut Vec<Collision>, c4: &'a Box<Quadtree> ) {
        scoped.execute(move || { *cs1 = c1.walk_tree() });
        scoped.execute(move || { *cs2 = c2.walk_tree() });
        scoped.execute(move || { *cs3 = c3.walk_tree() });
        scoped.execute(move || { *cs4 = c4.walk_tree() });
    }
    
    
    
}
impl SpatialPartition for Quadtree {

    // add object to quadtee at current level
    // will get added to children if valid
    fn insert(&mut self, index: usize, p: Vector) {
        self.empty = false;
        if let Some((ref mut c1, ref mut c2, ref mut c3, ref mut c4)) = self.children {
            if c1.within(p) { c1.insert(index, p); }
            if c2.within(p) { c2.insert(index, p); }
            if c3.within(p) { c3.insert(index, p); }
            if c4.within(p) { c4.insert(index, p); }
        }
        else {
            self.objects.push((index, p));

            // number of circles that can possibly fit within with volume
            let objects_per_volume = ( self.width*self.height ) / (4.0*self.radius*self.radius );

            // the volume must be able to contain more than 16
            let maximum_objects_per_volume = 16.0;

            // number of objects within the node
            let object_limit = 16;

            if self.objects.len() > object_limit && objects_per_volume > maximum_objects_per_volume {
                // println!("divide");
                self.divide();
            }
        }
    }

    // delete objects from the tree leaves non empty children alive
    fn clear(&mut self) {
        if self.empty == true { self.children = None; }
        if let Some((ref mut c1, ref mut c2, ref mut c3, ref mut c4)) = self.children {
            c1.clear();
            c2.clear();
            c3.clear();
            c4.clear();
        }

        self.objects.clear();
        self.empty = true;
    }
    
    
    
    // fn collision_check(&self) -> Vec<Collision> {
    //     let mut collisions = Vec::new();
    // 
    // 
    //     if let Some((ref c1, ref c2, ref c3, ref c4)) = self.children {
    // 
    //         let mut cs1 = Vec::new(); 
    //         let mut cs2 = Vec::new(); 
    //         let mut cs3 = Vec::new(); 
    //         let mut cs4 = Vec::new(); 
    //         
    //         if let Some(ref p) = self.pool {
    //             p.scoped(|scoped| {
    //                 Self::collect_data(&scoped, &mut cs1, &c1, &mut cs2, &c2, &mut cs3, &c3, &mut cs4, &c4);
    //             });
    //             // println!("{}", cs1.len());
    //             collisions.append( &mut cs1 );
    //             collisions.append( &mut cs2 );
    //             collisions.append( &mut cs3 );
    //             collisions.append( &mut cs4 );
    //         }  
    //     }
    //     
    //     for i in 0..self.objects.len() {
    //         let (index1, p_position) = self.objects[i];
    //         // let p_position = particles[index1].get_position();
    //     
    //         for j in (i+1)..self.objects.len() {
    //             let (index2, q_position) =  self.objects[j];
    //             // let q_position = particles[index2].get_position();
    //     
    //             let normal = (q_position - p_position).normalise();
    //             let penetration = 2.0*self.radius - p_position.distance( q_position );
    //     
    //             // if circles are overlapping
    //             if penetration > 0.0 {
    //                 // add collision
    //                 collisions.push( Collision::new(index1, index2, penetration, normal) );
    //             }
    //         }
    //     }
    //     
    //     collisions.sort();
    //     collisions.dedup();
    // 
    //     collisions
    // }

    fn collision_check(&mut self) -> &Vec<Collision> {
    
        if let Some((ref c1, ref c2, ref c3, ref c4)) = self.children {
            self.collisions.append( &mut c1.walk_tree() );
            self.collisions.append( &mut c2.walk_tree() );
            self.collisions.append( &mut c3.walk_tree() );
            self.collisions.append( &mut c4.walk_tree() );
        }
        
        for i in 0..self.objects.len() {
            let (index1, p_position) = self.objects[i];
            // let p_position = particles[index1].get_position();
        
            for j in (i+1)..self.objects.len() {
                let (index2, q_position) =  self.objects[j];
                // let q_position = particles[index2].get_position();
        
                let normal = (q_position - p_position).normalise();
                let penetration = 2.0*self.radius - p_position.distance( q_position );
        
                // if circles are overlapping
                if penetration > 0.0 {
                    // add collision
                    self.collisions.push( Collision::new(index1, index2, penetration, normal) );
                }
            }
        }
        
        self.collisions.sort();
        self.collisions.dedup();
    
        &self.collisions
    }
    
    fn collision_check_parallel(&mut self) -> &Vec<Collision> {
    
        if let Some((ref c1, ref c2, ref c3, ref c4)) = self.children {
    
            let mut cs1 = Vec::new(); 
            let mut cs2 = Vec::new(); 
            let mut cs3 = Vec::new(); 
            let mut cs4 = Vec::new(); 
            
            if let Some(ref p) = self.pool {
                
                
                p.scoped(|scoped| {
                    scoped.execute(|| { cs1 = c1.walk_tree() });
                    scoped.execute(|| { cs2 = c2.walk_tree() });
                    scoped.execute(|| { cs3 = c3.walk_tree() });
                    scoped.execute(|| { cs4 = c4.walk_tree() });
                    // Self::collect_data(&scoped, &mut cs1, &c1, &mut cs2, &c2, &mut cs3, &c3, &mut cs4, &c4);
                });
                // println!("{}", cs1.len());
                self.collisions.append( &mut cs1 );
                self.collisions.append( &mut cs2 );
                self.collisions.append( &mut cs3 );
                self.collisions.append( &mut cs4 );
            }  
        }
        
        for i in 0..self.objects.len() {
            let (index1, p_position) = self.objects[i];
            // let p_position = particles[index1].get_position();
        
            for j in (i+1)..self.objects.len() {
                let (index2, q_position) =  self.objects[j];
                // let q_position = particles[index2].get_position();
        
                let normal = (q_position - p_position).normalise();
                let penetration = 2.0*self.radius - p_position.distance( q_position );
        
                // if circles are overlapping
                if penetration > 0.0 {
                    // add collision
                    self.collisions.push( Collision::new(index1, index2, penetration, normal) );
                }
            }
        }
        
        self.collisions.sort();
        self.collisions.dedup();
    
        &self.collisions
    }
    
    fn collision_check_with_comparisons(&mut self) -> (&Vec<Collision>, Vec<(usize, usize)>) {
        let mut comparisons = Vec::new();
    
        if let Some((ref mut c1, ref mut c2, ref mut c3, ref mut c4)) = self.children {
            let (mut collisions1, mut comparisons1) = c1.collision_check_with_comparisons();
            let (mut collisions2, mut comparisons2) = c2.collision_check_with_comparisons();
            let (mut collisions3, mut comparisons3) = c3.collision_check_with_comparisons();
            let (mut collisions4, mut comparisons4) = c4.collision_check_with_comparisons();
                        
            self.collisions.append(&mut collisions1.clone());
            self.collisions.append(&mut collisions2.clone());
            self.collisions.append(&mut collisions3.clone());
            self.collisions.append(&mut collisions4.clone());

            comparisons.append(&mut comparisons1);
            comparisons.append(&mut comparisons2);
            comparisons.append(&mut comparisons3);
            comparisons.append(&mut comparisons4);
        }
        
        for i in 0..self.objects.len() {
            let (index1, p_position) = self.objects[i];
            // let p_position = particles[index1].get_position();
        
            for j in (i+1)..self.objects.len() {
                let (index2, q_position) =  self.objects[j];
                // let q_position = particles[index2].get_position();
        
                let normal = (q_position - p_position).normalise();
                let penetration = 2.0*self.radius - p_position.distance( q_position );
        
                comparisons.push((i, j));
        
                // if circles are overlapping
                if penetration > 0.0 {
                    // add collision
                    self.collisions.push( Collision::new(index1, index2, penetration, normal) );
                }
            }
        }
        
        self.collisions.sort();
        self.collisions.dedup();
    
        (&self.collisions, comparisons)
    }
}
