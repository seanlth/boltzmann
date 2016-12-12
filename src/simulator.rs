
use vector::*;
use particle::Particle;
use collision::Collision;

use rand;

pub struct Simulator {
    pub particles: Vec<Particle>,
    pub radius: f64,
    pub gravity: f64,
    pub width: u32,
    pub height: u32,
}

impl Simulator {

    pub fn new(number_of_particles: usize, radius: f64, gravity: f64, width: u32, height: u32, dt: f64) -> Simulator {
        let mut particles = Vec::new();

        for _ in 0..number_of_particles {

            // random positions and velocities
            let p_x = (rand::random::<u32>() % width) as f64;
            let p_y = (rand::random::<u32>() % height) as f64;
            let v_x = (rand::random::<u32>() % 50) as f64;
            let v_y = (rand::random::<u32>() % 50) as f64;

            particles.push( Particle::new( Vector::new(p_x, height as f64 - p_y ), Vector::new(v_x, v_y), dt) )
        }

        Simulator {
            particles: particles,
            radius: radius,
            width: width,
            gravity: gravity,
            height: height,
        }
    }


    pub fn velociies(&self) -> Vec<f64> {
        let mut vs = Vec::new();
        for p in &self.particles {
            let v = p.get_velocity();
            vs.push(v.magnitude());
        }
        vs
    }

    // total engery in system
    pub fn total_energy(&self) -> f64 {
        let mut energy = 0.0;
        for p in &self.particles {
            let v = p.get_velocity();
            energy += 0.5 * v.dot(v);
        }
        energy
    }

    // check for collisions (naive)
    fn collision_check(&self) -> Vec<Collision> {
        let mut collisions = Vec::new();

        for i in 0..self.particles.len() {
            let p = self.particles[i];
            let p_position = p.get_position();

            for j in (i+1)..self.particles.len() {
                let q = self.particles[j];
                let q_position = q.get_position();

                let normal = (q_position - p_position).normalise();
                let penetration = 2.0*self.radius - p_position.distance( q_position );

                // if circles are overlapping
                if penetration > 0.0 {

                    // add collision
                    collisions.push( Collision::new(i, j, penetration, normal) );
                }
            }
        }
        return collisions;
    }

    // solves collisions by applying impulse and adjusting particle locations
    fn solve_collisions(&mut self) {
        let collisions = self.collision_check();

        for c in collisions {
            let p = self.particles[c.p1];
            let q = self.particles[c.p2];
            let normal = c.normal;
            let penetration = c.penetration;

            // adjust particle positions
            let scale = 0.8;
            let slop = 0.0001;
            let correction = (f64::max( penetration - slop, 0.0 ) / 2.0) * scale * normal;
            self.particles[c.p1].set_position( p.get_position() - correction );
            self.particles[c.p2].set_position( q.get_position() + correction );

            // applying impulse
            let relative_velocity = q.get_velocity() - p.get_velocity();
            if relative_velocity.dot(normal) < 0.0 {
                let j = -( relative_velocity ).dot( normal );
                self.particles[c.p1].set_velocity( p.get_velocity() - j * normal );
                self.particles[c.p2].set_velocity( q.get_velocity() + j * normal );
            }
        }
    }

    // check particles are within the boundaries
    fn boundary_check(&mut self) {

        for p in &mut self.particles {
            let position = p.get_position();
            let velocity = p.get_velocity();
            if position.x - self.radius < 0.0 {
                p.set_position( Vector::new( self.radius, position.y ) );
                p.set_velocity( Vector::new( -velocity.x, velocity.y ) );
            }
            if position.x + self.radius > self.width as f64 {
                p.set_position( Vector::new( self.width as f64 - self.radius, position.y ) );
                p.set_velocity( Vector::new( -velocity.x, velocity.y ) );
            }
            if position.y - self.radius < 0.0 {
                p.set_position( Vector::new( position.x, self.radius ) );
                p.set_velocity( Vector::new( velocity.x, -velocity.y ) );
            }
            if position.y + self.radius > self.height as f64 {
                p.set_position( Vector::new( position.x, self.height as f64 - self.radius ) );
                p.set_velocity( Vector::new( velocity.x, -velocity.y ) );
            }
        }
    }

    // call from main loop
    pub fn update(&mut self) {
        self.solve_collisions();
        self.boundary_check();

        // apply gravity
        for p in &mut self.particles {
            p.verlet( Vector::new(0.0, self.gravity) );
        }

    }

}
