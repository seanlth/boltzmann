
use vector::Vector;

#[derive(Copy, Clone, PartialEq)]
pub struct Particle {
    pub position: Vector,
    pub previous_position: Vector,
    pub dt: f64
}

impl Particle {
    pub fn new(position: Vector, velocity: Vector, dt: f64) -> Particle {
        Particle {
            position: position,
            previous_position: position - velocity*dt,
            dt: dt
        }
    }

    pub fn verlet(&mut self, a: Vector) {
        let temp = self.position;
        self.position = 2.0*self.position - self.previous_position + a * self.dt * self.dt;
        self.previous_position = temp;
    }
}
