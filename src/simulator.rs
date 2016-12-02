extern crate rand;

use vector::*;
use particle::Particle;
use rand::Rng;
use piston_window::*;

fn grey_to_jet(mut v: f64, min: f64, max: f64) -> (f32, f32, f32)
{
    let mut c_r = 1.0;
    let mut c_g = 1.0;
    let mut c_b = 1.0;

    if v < min { v = min; }
    if v > max { v = max; }
    let dv = max - min;

    if v < (min + 0.25 * dv) {
      c_r = 0.0;
      c_g = 4.0 * (v - min) / dv;
    }
    else if v < (min + 0.5 * dv) {
      c_r = 0.0;
      c_b = 1.0 + 4.0 * (min + 0.25 * dv - v) / dv;
    }
    else if v < (min + 0.75 * dv) {
      c_r = 4.0 * (v - min - 0.5 * dv) / dv;
      c_b = 0.0;
    }
    else {
      c_g = 1.0 + 4.0 * (min + 0.75 * dv - v) / dv;
      c_b = 0.0;
    }

    (c_r as f32, c_g as f32, c_b as f32)
}


pub struct Simulator {
    particles: Vec<Particle>,
    radius: f64,
    width: u32,
    height: u32,
    window: PistonWindow
}

impl Simulator {
    pub fn new(number_of_particles: usize, radius: f64, width: u32, height: u32, dt: f64) -> Simulator {
        let mut particles = Vec::new();
        println!("{}", number_of_particles);
        for i in 0..number_of_particles {
            let p_x = (rand::random::<u32>() % width) as f64;
            let p_y = (rand::random::<u32>() % height) as f64;

            let v_x = (rand::random::<u32>() % 100) as f64;
            let v_y = (rand::random::<u32>() % 20) as f64;

            particles.push( Particle::new( Vector::new(50.0 + 0 as f64, height as f64 -  p_y ), Vector::new(0.0, 0.0), dt) )
        }

        Simulator {
            particles: particles,
            radius: radius,
            width: width,
            height: height,
            window: WindowSettings::new("Hello Piston!", [width, height])
            .exit_on_esc(true).build().unwrap()
        }
    }

    pub fn total_energy(&mut self) -> f64 {
        let mut energy = 0.0;
        for p in &self.particles {
            let v = p.get_velocity();
            energy += 0.5 * v.dot(v);
        }
        energy
    }

    pub fn draw(&mut self) {
        let ps = self.particles.clone();
        let r = self.radius;

        if let Some(e) = self.window.next() {
            self.window.draw_2d(&e, |c, g| {
                clear([0.1, 0.1, 0.1, 1.0], g);

                for p in ps {
                    let (red, green, blue) = grey_to_jet(p.get_velocity().magnitude(), 0.0, 100.0);
                    ellipse([red, green, blue, 1.0],
                            [p.get_position().x - r / 2.0, p.get_position().y - r / 2.0, r, r],
                            c.transform, g);
                }
            });
        }
        self.update();
    }

    pub fn update(&mut self) {
        for i in 0..self.particles.len() {
            let p = self.particles[i];
            let mut new_position = p.get_position();
            let mut new_velocity = p.get_velocity();

            let p_position = p.get_position();
            let p_velocity = p.get_velocity();
            if p_position.x - self.radius / 2.0 < 0.0 {
                new_position = Vector::new( self.radius / 2.0, p_position.y );
                new_velocity = Vector::new( -p_velocity.x, p_velocity.y );
            }
            if p_position.x + self.radius / 2.0 > self.width as f64 {
                new_position = Vector::new( self.width as f64 - self.radius / 2.0, p_position.y );
                new_velocity = Vector::new( -p_velocity.x, p_velocity.y );
            }
            if p_position.y - self.radius / 2.0 < 0.0 {
                new_position = Vector::new( p_position.x, self.radius / 2.0 );
                new_velocity = Vector::new( p_velocity.x, -p_velocity.y ) * 0.5;
            }
            if p_position.y + self.radius / 2.0 > self.height as f64 {
                new_position = Vector::new( p_position.x, self.height as f64 - self.radius / 2.0 );
                new_velocity = Vector::new( p_velocity.x, -p_velocity.y ) * 0.5;
            }


            self.particles[i].set_position( new_position );
            self.particles[i].set_velocity( new_velocity );


            for j in (i+1)..self.particles.len() {
                let q = self.particles[j];
                let q_position = q.get_position();
                let q_velocity = q.get_velocity();

                if p_position.distance( q_position ) < self.radius {
                    // let line = (q_position - p_position).normalise();
                    // let p = (q_velocity - p_velocity).dot( line );
                    // self.particles[i].set_velocity( p_velocity + p * line );
                    // self.particles[j].set_velocity( q_velocity + p * line );
                    // println!("{}", p);
                    // let middle = (p_position + q_position) / 2.0;
                    // let line = (p_position - q_position).normalise();
                    // // let mag = line.magnitude();
                    // // let factor = (mag - self.radius) / mag;
                    //
                    // self.particles[i].set_position( middle + 5.0*line );
                    // self.particles[j].set_position( middle - 5.0*line );
                    // // self.particles[i].set_velocity( ZERO_VECTOR );
                    // // self.particles[j].set_velocity( ZERO_VECTOR );
                    // self.particles[i].get_position().print();
                    //
                    // // println!("{}", (p_velocity - q_velocity).magnitude()  );
                    //
                    // // if -(p_velocity - q_velocity).dot( line ) <= 0.5 {
                         self.particles[i].set_velocity( q_velocity );
                         self.particles[j].set_velocity( p_velocity );
                    // // }
                }
            }
            self.particles[i].verlet( Vector::new(0.0, 1.0) );
        }


    }

}
