use vector::Vector;
use particle::Particle;

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

pub trait SpatialPartition {
    fn insert(&mut self, index: usize, position: Vector);
    fn clear(&mut self);
    fn collision_check(&self) -> Vec<Collision>;
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
