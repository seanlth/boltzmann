use collision::*;

use vector::Vector;
use particle::Particle;

pub struct Quadtree {
    pub empty: bool,
    pub level: usize,
    pub radius: f64,
    pub position: Vector,
    pub width: f64,
    pub height: f64,
    pub objects: Vec<(usize, Vector)>,
    pub children: Option<(Box<Quadtree>, Box<Quadtree>, Box<Quadtree>, Box<Quadtree>)>,
}

#[allow(dead_code)]
impl Quadtree {
    pub fn new(level: usize, radius: f64, position: Vector, width: f64, height: f64) -> Quadtree {
        Quadtree {
            empty: true,
            level: level,
            radius: radius,
            position: position,
            width: width,
            height: height,
            objects: vec![],
            children: None
        }
    }

    // completely resets the tree
    pub fn reset(&mut self) {
        self.children = None;
        self.objects.clear();
        self.empty == true;
    }

    // delete objects from the tree leaves non empty children alive
    pub fn clear(&mut self) {
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

    fn within(&self, p: Vector) -> bool {
        let b1 = p.x+self.radius >= self.position.x - self.width/2.0;
        let b2 = p.x-self.radius <= self.position.x + self.width/2.0;
        let b3 = p.y+self.radius >= self.position.y - self.height/2.0;
        let b4 = p.y-self.radius <= self.position.y + self.height/2.0;

        b1 && b2 && b3 && b4
    }

    // add object to quadtee at current level
    // will get added to children if valid
    pub fn add_object(&mut self, index: usize, p: Vector) {
        self.empty = false;
        if let Some((ref mut c1, ref mut c2, ref mut c3, ref mut c4)) = self.children {
            if c1.within(p) { c1.add_object(index, p); }
            if c2.within(p) { c2.add_object(index, p); }
            if c3.within(p) { c3.add_object(index, p); }
            if c4.within(p) { c4.add_object(index, p); }
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

    // creates children nodes and inserts objects from this node to children
    pub fn divide(&mut self) {
        let p1 = Vector::new(self.position.x - self.width/4.0, self.position.y + self.height/4.0);
        let c1 = Box::new( Quadtree::new(self.level+1, self.radius, p1, self.width/2.0, self.height/2.0) );

        let p2 = Vector::new(self.position.x + self.width/4.0, self.position.y + self.height/4.0);
        let c2 = Box::new( Quadtree::new(self.level+1, self.radius, p2, self.width/2.0, self.height/2.0) );

        let p3 = Vector::new(self.position.x - self.width/4.0, self.position.y - self.height/4.0);
        let c3 = Box::new( Quadtree::new(self.level+1, self.radius, p3, self.width/2.0, self.height/2.0) );

        let p4 = Vector::new(self.position.x + self.width/4.0, self.position.y - self.height/4.0);
        let c4 = Box::new( Quadtree::new(self.level+1, self.radius, p4, self.width/2.0, self.height/2.0) );

        self.children = Some((c1, c2, c3, c4));

        let temp = self.objects.clone();
        self.objects.clear();
        for (i, p) in temp { self.add_object(i, p); }
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
}

impl SpatialPartition for Quadtree {
    fn insert(&mut self, index: usize, p: Vector) {
        self.add_object(index, p);
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn collision_check(&self, particles: &Vec<Particle>) -> Vec<Collision> {
        let mut collisions = Vec::new();

        if let Some((ref c1, ref c2, ref c3, ref c4)) = self.children {
            collisions.append( &mut c1.collision_check(particles) );
            collisions.append( &mut c2.collision_check(particles) );
            collisions.append( &mut c3.collision_check(particles) );
            collisions.append( &mut c4.collision_check(particles) );
        }

        for i in 0..self.objects.len() {
            let (index1, _) = self.objects[i];
            let p_position = particles[index1].get_position();

            for j in (i+1)..self.objects.len() {
                let (index2, _) =  self.objects[j];
                let q_position = particles[index2].get_position();

                let normal = (q_position - p_position).normalise();
                let penetration = 2.0*self.radius - p_position.distance( q_position );

                // if circles are overlapping
                if penetration > 0.0 {
                    // add collision
                    collisions.push( Collision::new(index1, index2, penetration, normal) );
                }
            }
        }

        collisions
    }
}
