
use vector::*;
use particle::Particle;
use collision::*;
use attribute::Attribute;

pub type Probability = Fn() -> f64;

pub struct Simulator<T: SpatialPartition> {
    pub spatial_partition: T,
    pub particles: Vec<Particle>,
    pub radius: f64,
    pub gravity: f64,
    pub restitution: f64,
    pub width: f64,
    pub height: f64,
    pub dt: f64,
    attributes: Vec<Box<Attribute>>,
    normal_attributes: Vec<usize>,
    collision_attributes: Vec<usize>,
    attribute_id_count: usize,
}

#[allow(dead_code)]
impl<T: SpatialPartition> Simulator<T> {
    pub fn new(spatial_partition: T, 
        initial_posiiton: (&Probability, &Probability), initial_velocity: (&Probability, &Probability), 
        number_of_particles: usize, radius: f64, gravity: f64, restitution: f64, 
        width: f64, height: f64, dt: f64) -> Simulator<T> {
        
        let mut s = Simulator {
            spatial_partition: spatial_partition,
            particles: vec![],
            radius: radius,
            gravity: gravity,
            restitution: restitution,
            width: width,
            height: height,
            dt: dt,
            attributes: vec![],
            normal_attributes: vec![],
            collision_attributes: vec![],
            attribute_id_count: 0
        };
        s.initial_conditions(initial_posiiton, initial_velocity, number_of_particles, width, height, dt);
        s
    }

    fn initial_conditions(&mut self, initial_posiiton: (&Probability, &Probability), initial_velocity: (&Probability, &Probability), 
        number_of_particles: usize, width: f64, height: f64, dt: f64) {

        for i in 0..number_of_particles {

            // random positions and velocities
            let p_x = (initial_posiiton.0() * (width - 2.0*self.radius ) ) - self.radius;
            let p_y = (initial_posiiton.1() * (height - 2.0*self.radius ) ) - self.radius;
            let v_x = (initial_velocity.0() * 500.0) - 250.0;
            let v_y = (initial_velocity.1() * 500.0) - 250.0;

            let position = Vector::new( p_x, height as f64 - p_y );
            let velocity = Vector::new( v_x, v_y );

            self.particles.push( Particle::new(position, velocity, dt) );

            self.spatial_partition.insert(i, position);
        }
    }
    
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
    
    pub fn attribute(&self, id: usize) -> &Box<Attribute> {
        &self.attributes[id]
    }
    
    pub fn set_attribute(&mut self, id: usize, f: f64, i: f64) {
        self.attributes[id].set(f, i);
    }

    pub fn insert_particle(&mut self, p: Vector, v: Vector) {
        self.particles.push( Particle::new(p, v, self.dt) );
    }

    pub fn velocities(&self) -> Vec<f64> {
        let mut vs = Vec::new();
        for p in &self.particles {
            let v = p.get_velocity();
            vs.push(v.magnitude());
        }
        vs
    }
    
    pub fn positions(&self) -> Vec<Vector> {
        let mut ps = Vec::new();
        for p in &self.particles {
            ps.push( p.get_position() );
        }
        ps
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

    fn naive_collision_check(&self) -> Vec<Collision> {
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

        collisions
    }


    // solves collisions by applying impulse and adjusting particle locations
    fn solve_collisions(&mut self) {
        let collisions = self.spatial_partition.collision_check_parallel();

        for c in collisions {
            
            // update attributes 
            for a in &self.collision_attributes { 
                self.attributes[*a].collision_update(c.p1, c.p2, &self.particles[c.p1], &self.particles[c.p2]);
            }
                    
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


    // call from main loop
    pub fn update(&mut self) {
        self.solve_collisions();

        self.spatial_partition.clear();

        // apply gravity
        for (i, mut p) in &mut self.particles.iter_mut().enumerate() {
            p.verlet( Vector::new(0.0, self.gravity) );
            Self::boundary_check(p, self.radius, self.restitution, self.width, self.height);
            self.spatial_partition.insert(i, p.get_position());
            
            for a in &self.normal_attributes {
                self.attributes[*a].update(i, &p);
            }
        }
    }

}
