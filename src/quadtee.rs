

struct Quadtree {
    radius: f64,
    objects: Vec<usize>,
    child1: Option<Box<Quadtree>>,
    child2: Option<Box<Quadtree>>,
    child3: Option<Box<Quadtree>>,
    child4: Option<Box<Quadtree>>
}

impl Quadtree {
    pub fn new(radius: f64) -> Quadtree {
        Quadtree {
            radius: radius,
            objects: vec![],
            child1: None,
            child2: None,
            child3: None,
            child4: None
        }
    }

    // add object to quadtee at current level
    // will get added to children if valid
    pub fn add_object(&mut self, object: usize) {
        
    }

    //
    pub fn divide(&mut self) {

    }
}
