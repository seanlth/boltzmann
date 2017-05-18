//! Attribute definition and built in attributes. 
//! Attributes attach values to particles that can be 
//! updated when particles collide or on each simulation 
//! step.

use particle::Particle;
use common::scale;

/// Attribute definition. 
pub trait Attribute {
    /// Make a new attribute.
    fn new() -> Self where Self: Sized; 
    /// Get attribute data.
    fn get_data(&self) -> &Vec<f64>;
    /// Collision listener.
    fn collision_update(&mut self, usize, usize, _: &Particle, _: &Particle) {  }
    /// Update data.
    fn update(&mut self, usize, _: &Particle) {  }
    /// Returns true if collision listener is valid.
    fn collision_listener(&self) -> bool;
    /// Initialise the data.
    fn initialise(&mut self, t: Vec<f64>);
    /// Set attribute value for one particle. 
    fn set(&mut self, f: f64, i: f64);
    /// Get the maximum value in the attribute data.
    fn data_bounds(&self) -> (f64, f64) {
        let max = self.get_data().iter().cloned().fold(0./0., f64::max);
        (0.0, max)
    }
}

#[macro_export]
/// Macro for implementing a collision attribute.
macro_rules! impl_collision_attribute {
    ( $c:ident, $self_:ident, $i:ident, $j:ident, $p1:ident, $p2:ident, $l:expr, $u:expr, $b:block ) => { 
        #[allow(non_camel_case_types)]         
        pub struct $c { 
            t: Vec<f64> 
        }
        impl Attribute for $c { 
            fn new() -> $c { $c { t: vec![] } }
            fn get_data(&self) -> &Vec<f64> { &self.t } 
            fn collision_listener(&self) -> bool { true }
            fn collision_update(&mut $self_, $i: usize, $j: usize, $p1: &Particle, $p2: &Particle) { $b }
            fn initialise(&mut self, t: Vec<f64>) { self.t = t }
            fn set(&mut self, f: f64, i: f64) { let l = self.t.len() as f64; self.t[scale(i, [0.0, 1.0], [0.0, l - 1.0]) as usize] = f; } 
            fn data_bounds(&self) -> (f64, f64) { ($l, $u) }
        }
    };
    ( $c:ident, $self_:ident, $i:ident, $j:ident, $p1:ident, $p2:ident, $b:block ) => { 
        #[allow(non_camel_case_types)] 
        pub struct $c { 
            t: Vec<f64> 
        }
        impl Attribute for $c { 
            fn new() -> $c { $c { t: vec![] } }
            fn get_data(&self) -> &Vec<f64> { &self.t } 
            fn collision_listener(&self) -> bool { true }
            fn collision_update(&mut $self_, $i: usize, $j: usize, $p1: &Particle, $p2: &Particle) { $b }
            fn initialise(&mut self, t: Vec<f64>) { self.t = t }
            fn set(&mut self, f: f64, i: f64) { let l = self.t.len() as f64; self.t[scale(i, [0.0, 1.0], [0.0, l - 1.0]) as usize] = f; } 
        }
    };
}

/// Macro for implementing a normal attribute.
macro_rules! impl_attribute {
    ( $c:ident, $self_:ident, $i:ident, $p:ident, $l:expr, $u:expr, $b:block ) => { 
        #[allow(non_camel_case_types)]         
        pub struct $c { 
            t: Vec<f64> 
        }
        impl Attribute for $c { 
            fn new() -> $c { $c { t: vec![] } }
            fn get_data(&self) -> &Vec<f64> { &self.t } 
            fn collision_listener(&self) -> bool { false }
            fn update(&mut $self_, $i: usize, $p: &Particle) { $b }
            fn initialise(&mut self, t: Vec<f64>) { self.t = t }
            fn set(&mut self, f: f64, i: f64) { let l = self.t.len() as f64; self.t[scale(i, [0.0, 1.0], [0.0, l - 1.0]) as usize] = f; } 
            fn data_bounds(&self) -> (f64, f64) { ($l, $u) }
        }
    };
    ( $c:ident, $self_:ident, $i:ident, $p:ident, $b:block ) => { 
        #[allow(non_camel_case_types)] 
        pub struct $c { 
            t: Vec<f64> 
        }
        impl Attribute for $c { 
            fn new() -> $c { $c { t: vec![] } }
            fn get_data(&self) -> &Vec<f64> { &self.t } 
            fn collision_listener(&self) -> bool { false }
            fn update(&mut $self_, $i: usize, $p: &Particle) { $b }
            fn initialise(&mut self, t: Vec<f64>) { self.t = t }
            fn set(&mut self, f: f64, i: f64) { let l = self.t.len() as f64; self.t[scale(i, [0.0, 1.0], [0.0, l - 1.0]) as usize] = f; } 
        }
    };
}

#[allow(dead_code)]
/// Call the function symmetrically. 
pub fn symmetric<F: FnMut(usize, usize)>(i: usize, j: usize, mut f: F) {
    f(i, j);
    f(j, i);
}

/// Virus attribute.
impl_collision_attribute! { virus_attr, self, i, j, _p1, _p2, 0.0, 1.0,
    {
        //let max = self.t[i].max(self.t[j]);
        
        if self.t[i] == 1.0 && self.t[j] == 0.0 {
            self.t[j] = 0.5;
        }
        else if self.t[i] == 1.0 && self.t[j] == 0.5 {
            self.t[j] = 1.0;
        }
        else if self.t[j] == 1.0 && self.t[i] == 0.0 {
            self.t[i] = 0.5;
        }
        else if self.t[j] == 1.0 && self.t[i] == 0.5 {
            self.t[i] = 1.0;
        }
    }
}



//impl_collision_attribute! { virus2_attr, self, i, j, _p1, _p2, 0.0, 3.0,
    //{
        //let old = [self.t[i], self.t[j]];
        //let mut new = [self.t[i], self.t[j]];
        
        //symmetric(0, 1, |x, y| {
            //match (old[x], old[y]) {
                //(1.0, 0.0) => { new[y] = 0.5; }
                //(1.0, 0.5) => { new[y] = 0.8; }
                //(1.0, 0.8) => { new[y] = 1.0; }
                //(0.5, 0.0) => { new[y] = 0.5; }
                //(2.0, 0.0) => { new[y] = 3.0; }                
                //(2.0, 0.5) => { new[y] = 0.0; }
                //(2.0, 0.8) => { new[y] = 0.0; }     
                //(2.0, 1.0) => { new[y] = 0.0; }                                
                //(1.0, 3.0) => { new[y] = 0.5; }
                //(1.0, 2.5) => { new[y] = 0.5; }                
                //(_, _) => {}
            //}
        //});

        //self.t[i] = new[0];
        //self.t[j] = new[1];

        ////match (self.t[i], self.t[j]) {
            ////(1.0, 0.0) => { self.t[j] = 0.5; }
            ////(1.0, 0.5) => { self.t[j] = 0.8; }
            ////(1.0, 0.8) => { self.t[j] = 1.0; }
            ////(0.5, 0.0) => { self.t[j] = 0.5; }
            ////(2.0, _) => { self.t[j] = 3.0; }               
            ////(0.0, 1.0) => { self.t[i] = 0.5; }
            ////(0.5, 1.0) => { self.t[i] = 0.8; }
            ////(0.8, 1.0) => { self.t[i] = 1.0; }
            ////(0.0, 0.5) => { self.t[i] = 0.5; }
            ////(_, 2.0) => { self.t[i] = 3.0; }                        
            ////(_, _) => {}
        ////} 
    //}
//}


impl_collision_attribute! { density_attr, self, i, j, _p1, _p2,
    {
        let sum = ( self.t[i] + self.t[j] ) / 2.0;
        self.t[i] = sum; self.t[j] = sum;
    }
}

impl_collision_attribute! { tag_attr, self, i, j, _p1, _p2, 0.0, 1.0,
    {
        let t = self.t[i];
        self.t[i] = self.t[j];
        self.t[j] = t;
    }
}

impl_collision_attribute! { visited_attr, self, i, j, _p1, _p2, 0.0, 1.0, 
    {
        let t1 = self.t[i];
        let t2 = self.t[j];
        if t1 == 1.0 { self.t[j] = 1.0; self.t[i] = 0.5; }
        else if t2 == 1.0 { self.t[i] = 1.0; self.t[j] = 0.5; }
    }
}

#[allow(unused_variables, non_camel_case_types)] 
impl_collision_attribute! { range_attr, self, i, j, _p1, _p2, 0.0, 1.0,
    {
        let t1 = self.t[i];
        let t2 = self.t[j];
        if t1 == 1.0 { self.t[j] = 0.5; }
        else if t2 == 1.0 { self.t[i] = 0.5; }
    }
}

impl_attribute! { speed_attr, self, i, p,
    {
        self.t[i] = p.get_velocity().magnitude();
    }
}

