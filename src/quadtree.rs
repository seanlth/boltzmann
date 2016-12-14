
use vector::Vector;

pub struct Quadtree {
    pub level: usize,
    pub radius: f64,
    pub position: Vector,
    pub width: f64,
    pub height: f64,
    pub objects: Vec<(usize, Vector)>,
    pub children: Option<(Box<Quadtree>, Box<Quadtree>, Box<Quadtree>, Box<Quadtree>)>
}

impl Quadtree {
    pub fn new(level: usize, radius: f64, position: Vector, width: f64, height: f64) -> Quadtree {
        Quadtree {
            level: level,
            radius: radius,
            position: position,
            width: width,
            height: height,
            objects: vec![],
            children: None
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();

        if let Some((ref mut c1, ref mut c2, ref mut c3, ref mut c4)) = self.children {
            c1.clear();
            c2.clear();
            c3.clear();
            c4.clear();
        }

        self.children = None;
    }

    fn within(&self, p: Vector) -> bool {
        let b1 = p.x-self.radius > self.position.x - self.width/2.0;
        let b2 = p.x+self.radius < self.position.x + self.width/2.0;
        let b3 = p.y-self.radius > self.position.y - self.width/2.0;
        let b4 = p.y+self.radius < self.position.y + self.width/2.0;

        b1 && b2 && b3 && b4
    }

    // add object to quadtee at current level
    // will get added to children if valid
    pub fn add_object(&mut self, index: usize, p: Vector) {
        if let Some((ref mut c1, ref mut c2, ref mut c3, ref mut c4)) = self.children {
            if c1.within(p) { c1.add_object(index, p); }
            else if c2.within(p) { c2.add_object(index, p); }
            else if c3.within(p) { c3.add_object(index, p); }
            else if c4.within(p) { c4.add_object(index, p); }
            else { self.objects.push((index, p)); }
        }
        else {
            self.objects.push((index, p));
            if self.objects.len() > 10 {
                self.divide();
            }
        }
    }

    //
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
        println!("----------");
        println!("This: {}", self.level);
        for &(_, p) in &self.objects { p.print(); };
        if let Some((ref c1, ref c2, ref c3, ref c4)) = self.children {
            println!("c1: ");
            c1.print();
            println!("c1: ");
            c2.print();
            println!("c1: ");
            c3.print();
            println!("c4: ");
            c4.print();
        }
    }
}
