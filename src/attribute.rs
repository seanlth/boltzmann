use particle::Particle;
use common::scale;

pub trait Attribute {
    fn new() -> Self where Self: Sized; 
    fn get_data(&self) -> &Vec<f64>;
    fn collision_update(&mut self, usize, usize, _: &Particle, _: &Particle) {  }
    fn update(&mut self, usize, _: &Particle) {  }
    fn collision_listener(&self) -> bool;
    fn initialise(&mut self, t: Vec<f64>);
    fn set(&mut self, f: f64, i: f64);
    fn data_bounds(&self) -> (f64, f64) {
        let max = self.get_data().iter().cloned().fold(0./0., f64::max);
        (0.0, max)
    }
}

#[macro_export]
macro_rules! impl_collision_attribute {
    ( $c:ident, $self_:ident, $i:ident, $j:ident, $p1:ident, $p2:ident, $l:expr, $u:expr, $b:block ) => { 
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

macro_rules! impl_attribute {
    ( $c:ident, $self_:ident, $i:ident, $p:ident, $l:expr, $u:expr, $b:block ) => { 
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

impl_collision_attribute! { Virus, self, i, j, p1, p2, 0.0, 1.0,
    {
        self.t[i] = self.t[i].max(self.t[j]);
        self.t[j] = self.t[j].max(self.t[i]);  
    }
}

impl_collision_attribute! { Density, self, i, j, p1, p2, 0.0, 1.0,
    {
        let sum = ( self.t[i] + self.t[j] ) / 2.0;
        self.t[i] = sum; self.t[j] = sum;
    }
}

impl_collision_attribute! { Tag, self, i, j, p1, p2, 0.0, 1.0,
    {
        let t = self.t[i];
        self.t[i] = self.t[j];
        self.t[j] = t;
    }
}

impl_collision_attribute! { Visted, self, i, j, p1, p2, 0.0, 1.0, 
    {
        let t1 = self.t[i];
        let t2 = self.t[j];
        if t1 == 1.0 { self.t[j] = 1.0; self.t[i] = 0.5; }
        else if t2 == 1.0 { self.t[i] = 1.0; self.t[j] = 0.5; }
    }
}

impl_collision_attribute! { Range, self, i, j, p1, p2, 0.0, 1.0,
    {
        let t1 = self.t[i];
        let t2 = self.t[j];
        if t1 == 1.0 { self.t[j] = 0.5; }
        else if t2 == 1.0 { self.t[i] = 0.5; }
    }
}

impl_attribute! { Speed, self, i, p,
    {
        self.t[i] = p.get_velocity().magnitude();
    }
}
