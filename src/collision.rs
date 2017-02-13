use vector::Vector;
use particle::Particle;

use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub struct Collision {
    pub p1: usize,
    pub p2: usize,
    pub penetration: f64,
    pub normal: Vector
}

unsafe impl Sync for Collision {}
unsafe impl Send for Collision {}

impl Collision {
    pub fn new(p1: usize, p2: usize, penetration: f64, normal: Vector) -> Collision {
        
        let (i, j) = if p2 < p1 { (p2, p1) }
                     else { (p1, p2) };
        
        Collision {
            p1: i,
            p2: j,
            penetration: penetration,
            normal: normal
        }
    }
}

impl PartialEq for Collision {
    fn eq(&self, other: &Self) -> bool {
        self.p1 == other.p1 && self.p2 == other.p2 ||
        self.p2 == other.p1 && self.p1 == other.p2
    }
}
impl Eq for Collision {} 

impl Ord for Collision {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.p1 < other.p1 { Ordering::Less }
        else if self.p1 > other.p1 { Ordering::Greater }
        else {
            if self.p2 < other.p2 { Ordering::Less }
            else if self.p2 > other.p2 { Ordering::Greater }
            else { Ordering::Equal }
        }
    }
}

impl PartialOrd for Collision {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub trait SpatialPartition {
    fn insert(&mut self, index: usize, position: Vector);
    fn clear(&mut self);
    fn collision_check(&mut self) -> &Vec<Collision>;
    fn collision_check_parallel(&mut self) -> &Vec<Collision>;
    fn collision_check_with_comparisons(&mut self) -> (&Vec<Collision>, Vec<(usize, usize)>);
}


fn naive_collision_check(radius: f64, particles: &Vec<Particle>) -> Vec<Collision> {
    let mut collisions = Vec::new();

    for i in 0..particles.len() {
        let p = particles[i];
        let p_position = p.get_position();

        for j in (i+1)..particles.len() {
            let q = particles[j];
            let q_position = q.get_position();

            let normal = (q_position - p_position).normalise();
            let penetration = 2.0*radius - p_position.distance( q_position );

            // if circles are overlapping
            if penetration > 0.0 {

                // add collision
                collisions.push( Collision::new(i, j, penetration, normal) );
            }
        }
    }

    collisions
}
