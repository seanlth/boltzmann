extern crate rand;

use vector::*;
use particle::Particle;
use rand::Rng;
use piston_window::*;


pub struct Simulator {
    particles: Vec<Particle>,
    radius: f64,
    width: u64,
    height: u64,
    window: PistonWindow
}

impl Simulator {
    pub fn new(number_of_particles: usize, radius: f64, width: u64, height: u64, dt: f64) -> Simulator {
        let mut particles = Vec::new();
        for _ in 0..number_of_particles {
            let x = (rand::random::<u64>() % width) as f64;
            let y = (rand::random::<u64>() % height) as f64;

            particles.push( Particle::new(Vector::new(x, y), Vector::new(10.0, 0.0), dt) )
        }

        Simulator {
            particles: particles,
            radius: radius,
            width: width,
            height: height,
            window: WindowSettings::new("Hello Piston!", [640, 480])
            .exit_on_esc(true).build().unwrap()
        }
    }

    pub fn draw(&mut self) {
        let ps = self.particles.clone();
        let r = self.radius;

        if let Some(e) = self.window.next() {
            self.window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);

                for p in ps {
                    ellipse([0.0, 0.0, 0.0, 1.0], // red
                            [p.position.x, p.position.y, r, r],
                            c.transform, g);
                }
            });
        }
        self.update();
    }

    pub fn update(&mut self) {
        for p in &mut self.particles {
            if p.position.x - self.radius < 0.0 {
                p.position.x = self.radius;
            }
            if p.position.x + self.radius > self.width as f64 {
                p.position.x = self.width as f64 - self.radius;
            }
            if p.position.y - self.radius < 0.0 {
                p.position.y = self.radius;
            }
            if p.position.y + self.radius > self.height as f64 {
                p.position.y = self.height as f64 - self.radius;
            }   

            p.verlet(Vector::new(0.0, 0.0));
        }


    }

}
