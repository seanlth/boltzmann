
use vector::Vector;

#[derive(Copy, Clone, PartialEq)]
pub struct Particle {
    position: Vector,
    previous_position: Vector,
    dt: f64
}

impl Particle {
    pub fn new(position: Vector, velocity: Vector, dt: f64) -> Particle {
        Particle {
            position: position,
            previous_position: position - velocity * dt,
            dt: dt
        }
    }

    pub fn verlet(&mut self, a: Vector) {
        let temp = self.position;
        self.position = 2.0*self.position - self.previous_position + a * self.dt * self.dt;
        self.previous_position = temp;
    }

    pub fn get_velocity(&self) -> Vector {
        ( self.position - self.previous_position ) / self.dt
    }

    pub fn set_velocity(&mut self, v: Vector) {
        self.previous_position = self.position - v * self.dt;
    }

    pub fn get_position(&self) -> Vector {
        self.position
    }

    pub fn set_position(&mut self, p: Vector) {
        let v = self.get_velocity();
        self.position = p;
        self.previous_position = self.position - v * self.dt;
    }
}
