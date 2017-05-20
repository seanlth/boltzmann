//! Simulator. 

use std::f64;

use vector::*;
use particle::Particle;
use collision::*;
use attribute::Attribute;
use scale;


/// Probability function. 

pub type Probability = Fn() -> f64;

/// Struct representing the simulation. 

pub struct Simulator<T: SpatialPartition> {
    /// The spatial partitioning scheme used for
    /// accelerating collision detection.
    spatial_partition: T,
    /// Number of particles.
    number_of_particles: usize,
    /// The list of particles.
    particles: Vec<Particle>,
    /// Acceleration due to gravity. 
    gravity: f64,
    /// The restitution in a particle-particle collision.
    particle_restitution: f64,
    /// The restitution in a wall-particle collision.
    wall_restitution: f64,
    /// Simulation domain width.
    width: f64,
    /// Simulation domain height.
    height: f64,
    /// Maximum allowed speed of a particle.
    max_speed: f64,
    /// Timestep.
    dt: f64,
    attributes: Vec<Box<Attribute>>,
    normal_attributes: Vec<usize>,
    collision_attributes: Vec<usize>,
    attribute_id_count: usize,
}

#[allow(dead_code)]
impl<T: SpatialPartition> Simulator<T> {

    /// Make a simulator. 
    pub fn new(spatial_partition: T, number_of_particles: usize, 
        gravity: f64, particle_restitution: f64, wall_restitution: f64, 
        width: f64, height: f64, max_speed: f64, dt: f64) -> Simulator<T> {
        
        Simulator {
            spatial_partition: spatial_partition,
            number_of_particles: number_of_particles,
            particles: vec![],
            gravity: gravity,
            particle_restitution: particle_restitution,
            wall_restitution: wall_restitution,
            width: width,
            height: height,
            max_speed: max_speed,
            dt: dt,
            attributes: vec![],
            normal_attributes: vec![],
            collision_attributes: vec![],
            attribute_id_count: 0
        }
    }
    
    /// Get a reference to the spatial partition.
    pub fn spatial_partition(&self) -> &T { &self.spatial_partition }

    /// Get the number of particles.
    pub fn number_of_particles(&self) -> usize { self.number_of_particles }

    /// Get the current gravity.
    pub fn gravity(&self) -> f64 { self.gravity }

    /// Get the restitution in particle collision.
    pub fn particle_restitution(&self) -> f64 { self.particle_restitution }

    /// The restitution in a wall collision.
    pub fn wall_restitution(&self) -> f64 { self.wall_restitution }

    /// Get the domain width.
    pub fn width(&self) -> f64 { self.width }

    /// Get the domain height.
    pub fn height(&self) -> f64 { self.height }

    /// Get the max_speed.
    pub fn max_speed(&self) -> f64 { self.max_speed }

    /// Get the timestep.
    pub fn timestep(&self) -> f64 { self.dt }


    /// Specify the initial conditions manually. 
    pub fn initial_conditions(&mut self, 
                              positions: Vec<Vector>,
                              velocities: Vec<Vector>,
                              radii: Vec<f64>) {
        
        for i in 0..self.number_of_particles {

            let r = radii[i];

            let p_x = f64::max(0.0, f64::min(self.width - r, positions[i].x) );
            let p_y = f64::max(0.0, f64::min(self.height - r, positions[i].y) );

            let dir = velocities[i].normalise();
            let speed = f64::min(self.max_speed, velocities[i].magnitude());

            let v_x = dir.x * speed;
            let v_y = dir.y * speed;

            let position = Vector::new( p_x, self.height as f64 - p_y );
            let velocity = Vector::new( v_x, v_y );

            let p = Particle::new(position, velocity, r, self.dt);

            self.particles.push( p );

            self.spatial_partition.insert(i, position, p.radius);
        }

    }

    /// Specify the initial conditions probabilistically.
    pub fn probabilistic_initial_conditions(&mut self, 
                          posiition_distribution: (&Probability, &Probability), 
                          velocity_distribution: (&Probability, &Probability), 
                          radii_distribution: &Probability) {

        for i in 0..self.number_of_particles {

            // random positions and velocities
            let r = radii_distribution();
            
            let p_x = scale(posiition_distribution.0(), [0.0, 1.0], [r, self.width as f64 - r]); 
            let p_y = scale(posiition_distribution.1(), [0.0, 1.0], [r, self.height as f64 - r]); 
            
            let angle = scale(velocity_distribution.0(), [0.0, 1.0], [0.0, 2.0*f64::consts::PI]);
            let dir = Vector::new( f64::cos(angle), f64::sin(angle) ).normalise();
            let speed = self.max_speed * velocity_distribution.1();

            let position = Vector::new( p_x, self.height as f64 - p_y );
            let velocity = speed * dir;

            let p = Particle::new(position, velocity, r, self.dt);

            self.particles.push( p );

            self.spatial_partition.insert(i, position, p.radius);
        }
    }
    
    /// Bind an attribute.
    pub fn bind_attribute<A: 'static + Attribute>(&mut self) -> usize {
        let mut attribute = Box::new( A::new() );
        attribute.initialise( vec![0.0; self.particles.len()] );
        let id = self.attribute_id_count;
        match attribute.collision_listener() {
            true => self.collision_attributes.push(id),
            false => self.normal_attributes.push(id)
        }
        self.attributes.push(attribute);
        self.attribute_id_count += 1;
        id
    }
    
    /// Get an an attribute.
    pub fn attribute(&self, id: usize) -> &Box<Attribute> {
        &self.attributes[id]
    }
    
    /// Set a value for the attribute.
    pub fn set_attribute(&mut self, id: usize, f: f64, i: f64) {
        self.attributes[id].set(f, i);
    }

    /// Insert a particle. 
    pub fn insert_particle(&mut self, position: Vector, velocity: Vector, radius: f64) {
        self.particles.push( Particle::new(position, velocity, radius, self.dt) );
        self.number_of_particles += 1;
    }

    /// Get a list of all the particle velocities;
    pub fn velocities(&self) -> Vec<f64> {
        let mut vs = Vec::new();
        for p in &self.particles {
            let v = p.get_velocity();
            vs.push(v.magnitude());
        }
        vs
    }
    
    /// Get a list of all the particle positions;
    pub fn positions(&self) -> Vec<Vector> {
        let mut ps = Vec::new();
        for p in &self.particles {
            ps.push( p.get_position() );
        }
        ps
    }

    /// Get the total engery in system
    pub fn total_energy(&self) -> f64 {
        let mut energy = 0.0;
        for p in &self.particles {
            let v = p.get_velocity();
            energy += 0.5 * v.dot(v);
        }
        energy
    }

    // solves collisions by applying impulse and adjusting particle locations
    fn solve_collisions(&mut self) {
        let collisions = self.spatial_partition.collision_check_parallel();

        for c in collisions {
            
            // update attributes 
            for a in &self.collision_attributes { 
                self.attributes[*a].collision_update(c.p1, c.p2, 
                                                     &self.particles[c.p1], &self.particles[c.p2]);
            }
                    
            let p = self.particles[c.p1];
            let q = self.particles[c.p2];
            let normal = c.normal;
            let penetration = c.penetration;

            // adjust particle positions
            let scale = 0.8;
            let slop = 0.0001;
            let correction = f64::max( penetration - slop, 0.0 ) * scale * normal;
            
            let p1 = p.radius / ( p.radius + q.radius );
            let p2 = q.radius / ( p.radius + q.radius );

            self.particles[c.p1].set_position( p.get_position() - correction*p2 );
            self.particles[c.p2].set_position( q.get_position() + correction*p1 );

            // applying impulse
            let relative_velocity = q.get_velocity() - p.get_velocity();
            if relative_velocity.dot(normal) < 0.0 {
                let m1 = p.radius * 0.1;
                let m2 = q.radius * 0.1;

                let j = (-(1.0 + self.particle_restitution) * relative_velocity.dot(normal) ) / (m1+m2);
                
                self.particles[c.p1].set_velocity( p.get_velocity() - (j * normal) * m2 );
                self.particles[c.p2].set_velocity( q.get_velocity() + (j * normal) * m1 );
                
            }
        }
    }
    
    fn boundary_check(p: &mut Particle, radius: f64, restitution: f64, width: f64, height: f64) {

        // for p in &mut self.particles {
        let position = p.get_position();
        let velocity = p.get_velocity();
        if position.x - radius < 0.0 {
            p.set_position( Vector::new( radius, position.y ) );
            p.set_velocity( Vector::new( velocity.x.abs()*restitution, velocity.y ) );
        }
        let position = p.get_position();
        let velocity = p.get_velocity();
        if position.x + radius > width as f64 {
            p.set_position( Vector::new( width as f64 - radius, position.y ) );
            p.set_velocity( Vector::new( - velocity.x.abs()*restitution, velocity.y ) );
        }
        let position = p.get_position();
        let velocity = p.get_velocity();
        if position.y - radius < 0.0 {
            p.set_position( Vector::new( position.x, radius ) );
            p.set_velocity( Vector::new( velocity.x, velocity.y.abs()*restitution ) );
        }
        let position = p.get_position();
        let velocity = p.get_velocity();
        if position.y + radius > height as f64 {
            p.set_position( Vector::new( position.x, height as f64 - radius ) );
            p.set_velocity( Vector::new( velocity.x, - velocity.y.abs()*restitution ) );
        }
    }


    /// Do a simulation step. 
    pub fn update(&mut self) {
        self.solve_collisions();

        self.spatial_partition.clear();

        // apply gravity
        for (i, mut p) in &mut self.particles.iter_mut().enumerate() {
            p.verlet( Vector::new(0.0, self.gravity) );
            let r = p.radius;
            Self::boundary_check(p, r, self.wall_restitution, self.width, self.height);
            self.spatial_partition.insert(i, p.get_position(), r);
            
            for a in &self.normal_attributes {
                self.attributes[*a].update(i, &p);
            }
        }
    }

}
