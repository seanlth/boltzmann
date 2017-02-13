

pub trait Attribute {
    fn get_vec(&self) -> &Vec<f64>;
    fn collision_update(&mut self, usize, usize) {  }
    fn update(&mut self, usize) {  }

}

#[macro_export]
macro_rules! impl_collision_attribute {
    ( $c:ident, $self_:ident, $i:ident, $j:ident, $b:block ) => { 
        pub struct $c { 
            t: Vec<f64> 
        }
        impl $c { pub fn new(t: Vec<f64>) -> $c { $c { t: t } } }
        impl Attribute for $c { 
            fn get_vec(&self) -> &Vec<f64> { &self.t } 
            fn collision_update(&mut $self_, $i: usize, $j: usize) { $b }
        }
    };
}

macro_rules! impl_attribute {
    ( $c:ident, $self_:ident, $i:ident, $b:block ) => { 
        pub struct $c { 
            t: Vec<f64> 
        }
        impl $c { pub fn new(t: Vec<f64>) -> $c { $c { t: t } } }
        impl Attribute for $c { 
            fn get_vec(&self) -> &Vec<f64> { &self.t } 
            fn update(&mut $self_, $i: usize) { $b }
        }
    };
}

impl_collision_attribute! { Virus, self, i, j, 
    {
        self.t[i] = self.t[i].max(self.t[j]);
        self.t[j] = self.t[j].max(self.t[i]);  
    }
}

impl_collision_attribute! { Temperature, self, i, j,
    {
        let sum = ( self.t[i] + self.t[j] ) / 2.0;
        self.t[i] = sum; self.t[j] = sum;
    }
}

// pub struct Temperature {
//     t: Vec<f64>
// }
// 
// impl Temperature {
//     pub fn new(t: Vec<f64>) -> Temperature {
//         Temperature {
//             t: t
//         }
//     }
// }
// 
// impl Attribute for Temperature {
//     fn collision_update(&mut self, i: usize, j: usize) {
//         let sum = ( self.t[i] + self.t[j] ) / 2.0;
//         self.t[i] = sum; self.t[j] = sum;
//     }
//     
//     fn get_vec(&self) -> &Vec<f64> {
//         &self.t
//     }
// }


pub struct Tag {
    t: Vec<f64>
}

impl Tag {
    pub fn new(t: Vec<f64>) -> Tag {
        Tag {
            t: t
        }
    }
}

impl Attribute for Tag {
    fn collision_update(&mut self, i: usize, j: usize) {
        let t = self.t[i];
        self.t[i] = self.t[j];
        self.t[j] = t;
        
        // let sum = ( self.t[i] + self.t[j] ) / 2.0;
        // self.t[i] = sum; self.t[j] = sum;
        
        // self.t[i] = self.t[i].max(self.t[j]);
        // self.t[j] = self.t[j].max(self.t[i]);
        // println!("{}", self.t[i] );
    }
    
    fn get_vec(&self) -> &Vec<f64> {
        &self.t
    }
}


// impl_attribute! { Speed, self, i, 
//     {
//         self.t[i] = self.t[i].max(self.t[j]);
//     }
// }
