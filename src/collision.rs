//! Collision data structure and spatial partitioning. 

use vector::Vector;
use particle::Particle;
use std::cmp::Ordering;

/// <img src="https://seanlth.github.io/boltzmann/images/collision.svg" width="300px"> <br>
/// Represents a collision between 2 particles. 
/// The id of p1 is always less than or equal to p2.

#[derive(Copy, Clone)]
pub struct Collision {
    /// id of particle 1.
    pub p1: usize,
    /// id of particle 2.
    pub p2: usize,
    /// Amount by which the particles are overlapping.
    pub penetration: f64,
    /// Vector of the collision normal.
    pub normal: Vector
}

impl Collision {
    /// Make a new collision.
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
    
    // (p1, p2) ~ (p1, p2)
    fn cmp(&self, other: &Self) -> Ordering {
        if self.p1 < other.p1 { Ordering::Less } 
        else if self.p1 > other.p1 { Ordering::Greater }
        else if self.p2 < other.p2 { Ordering::Less }
        else if self.p2 > other.p2 { Ordering::Greater }
        else { Ordering::Equal }
    }
}

impl PartialOrd for Collision {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Represents a spatial partitioning data structure
/// for accelerating collision checks.

pub trait SpatialPartition {
    /// Add a particle to the structure.
    fn insert(&mut self, index: usize, position: Vector, radius: f64);
    /// Clear all particles from the structure.
    fn clear(&mut self);
    /// Check for collisions.
    fn collision_check(&mut self) -> &Vec<Collision>;
    /// Check for collisions in a multithreaded fashion if possible.
    fn collision_check_parallel(&mut self) -> &Vec<Collision>;
    /// Check for collisions and also what pairs of particles were 
    /// compared in the process. 
    fn collision_check_with_comparisons(&mut self) -> (&Vec<Collision>, Vec<(usize, usize)>);
}


/// Check for collisions in a naive O(n^2 ) fashion.

pub fn naive_collision_check(radius: f64, particles: &[Particle]) -> Vec<Collision> {
    let mut collisions = Vec::new();

    for (i, p) in particles.iter().enumerate() {
        let p_position = p.get_position();

        for (j, q) in particles.iter().enumerate().skip((i+1)) {
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
