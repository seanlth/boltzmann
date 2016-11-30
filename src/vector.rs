use std::f64;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Neg;


#[derive(Copy, Clone, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Vector {
        Vector {
            x: x,
            y: y
        }
    }

    pub fn normalise(&self) -> Vector {
        let d = ( (self.x * self.x) + (self.y * self.y) ).sqrt();
        Vector::new(self.x/d, self.y/d)
    }

    pub fn normal(&self) -> Vector {
        Vector::new( -self.y , self.x )
    }

    pub fn magnitude(&self) -> f64 {
        ( (self.x * self.x) + (self.y * self.y) ).sqrt()
    }

    pub fn distance(&self, v: Vector) -> f64 {
        ( ( (self.x - v.x) * (self.x - v.x)) + ((self.y - v.y) * (self.y - v.y) ) ).sqrt()
    }

    pub fn dot(&self, v: Vector) -> f64 {
        self.x * v.x + self.y * v.y
    }

    pub fn cross(&self, v: Vector) -> f64 {
        self.x * v.y - self.y * v.x
    }

    pub fn print(&self) {
        println!("(x: {}, y: {})", self.x, self.y);
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, v: Vector) -> Vector {
        Vector::new(self.x + v.x, self.y + v.y)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, v: Vector) -> Vector {
        Vector::new(self.x - v.x, self.y - v.y)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, v: f64) -> Vector {
        Vector::new(self.x * v, self.y * v)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, v: Vector) -> Vector {
        Vector::new(self * v.x, self * v.y)
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, v: f64) -> Vector {
        Vector::new(self.x / v, self.y / v)
    }
}

impl Div<Vector> for Vector {
    type Output = Vector;

    fn div(self, v: Vector) -> Vector {
        Vector::new(self.x / v.x, self.y / v.y)
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector::new(-self.x, -self.y)
    }
}

pub const ZERO_VECTOR: Vector = Vector { x: 0.0, y: 0.0 };
pub const MAX_VECTOR: Vector = Vector { x: f64::MAX, y: f64::MAX };
