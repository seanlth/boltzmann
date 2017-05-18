//! Particle definition.

use vector::Vector;

/// Particle.
#[derive(Copy, Clone, PartialEq)]
pub struct Particle {
    position: Vector,
    previous_position: Vector,
    pub radius: f64,
    dt: f64
}

impl Particle {

    /// Make a particle.
    pub fn new(position: Vector, velocity: Vector, radius: f64, dt: f64) -> Particle {
        Particle {
            position: position,
            previous_position: position - velocity * dt,
            radius: radius,
            dt: dt
        }
    }

    /// Apply acceleration using verlet integration.
    pub fn verlet(&mut self, a: Vector) {
        let temp = self.position;
        self.position = 2.0*self.position - self.previous_position + a * self.dt * self.dt;
        self.previous_position = temp;
    }

    /// Get particle velocity.
    pub fn get_velocity(&self) -> Vector {
        ( self.position - self.previous_position ) / self.dt
    }

    /// Set particle velocity.
    pub fn set_velocity(&mut self, v: Vector) {
        self.previous_position = self.position - v * self.dt;
    }

    /// Get current position.
    pub fn get_position(&self) -> Vector {
        self.position
    }

    /// Set current position.
    pub fn set_position(&mut self, p: Vector) {
        let v = self.get_velocity();
        self.position = p;
        self.previous_position = self.position - v * self.dt;
    }
}
