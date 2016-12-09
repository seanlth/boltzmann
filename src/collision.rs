
use vector::Vector;

pub struct Collision {
    pub p1: usize,
    pub p2: usize,
    pub penetration: f64,
    pub normal: Vector
}

impl Collision {
    pub fn new(p1: usize, p2: usize, penetration: f64, normal: Vector) -> Collision {
        Collision {
            p1: p1,
            p2: p2,
            penetration: penetration,
            normal: normal
        }
    }
}
