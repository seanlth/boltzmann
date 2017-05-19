/*! 

# boltzmann

**boltzmann** is a library for simulating collisions between hard circles.

*/

extern crate rand;
extern crate rustc_serialize;
#[macro_use]
extern crate glium;
extern crate scoped_pool;

pub mod config;
pub mod vector;
pub mod particle;
pub mod collision;
pub mod spatial_hash;
pub mod quadtree;
pub mod simulator;
pub mod attribute;
pub mod common;
pub mod drawing;

use std::io::prelude::*;
use std::fs::File;
use std::cmp;

/// <img src="https://seanlth.github.io/boltzmann/images/scale.svg" width="400px"> <br>
/// Scales the input x from scale1 to scale2.

pub fn scale(x: f64, scale1: [f64; 2], scale2: [f64; 2]) -> f64 {
    let a = (x - scale1[0]) / (scale1[1] - scale1[0]);
    let b = a * (scale2[1] - scale2[0]);
    b + scale2[0]
}

/// Ensures the input value x is bounded by the input bounds.

pub fn bound(x: f64, bounds: [f64; 2]) -> f64 {
    f64::max(bounds[0], f64::min(x, bounds[1]))
}

/// Maps the value v to an RGB value defined by the jet
/// colour map.

pub fn grey_to_jet(mut v: f64, min: f64, max: f64) -> (f32, f32, f32)
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
 

/// <img src="https://seanlth.github.io/boltzmann/images/cubic.svg" width="300px"> <br> 
/// Interpolate the value at w between b and c where w is between 0 and 1.

pub fn cubic_interpolate( a: f64, b: f64, c: f64, d: f64, w: f64 ) -> f64 {

    let a0 = d - c - a + b;
    let a1 = a - b - a0;
    let a2 = c - a;
    let a3 = b;

   f64::max(0.0, a0*w*w*w + a1*w*w + a2*w + a3)
}

/// <img src="https://seanlth.github.io/boltzmann/images/linear.svg" width="300px"> <br> 
/// Interpolate the value at w between a and b where w is between 0 and 1.

pub fn linear_interpolate(a: f64, b: f64, w: f64) -> f64 {
	a * w + b * (1.0 - w)
}

/// Given a list of data points return a list of
/// n interpolated points using linear interpolation. 

pub fn linear_interpolate_vec(data_points: &Vec<f64>, n: usize) -> Vec<f64> {
    let mut new = Vec::new();
    
    let len = data_points.len();
    
    for index in 0..n {
        let x = scale(index as f64, [0.0, n as f64 - 1.0], [0.0, len as f64 - 1.0]);
        let i = x as i32;
        let j = cmp::min(i+1, len as i32 - 1);

        let a = data_points[ i as usize ] as f64;
        let b = data_points[ j as usize ] as f64;
                
        new.push( linear_interpolate(b, a, x - i as f64) );
    }
    
    new
}

/// Given a list of data points return a list of
/// n interpolated points using cubic interpolation. 

pub fn cubic_interpolate_vec(data_points: &Vec<f64>, n: usize) -> Vec<f64> {
    let mut new = Vec::new();
    
    let len = data_points.len();
    
    for index in 0..n {
        let x = scale(index as f64, [0.0, n as f64 - 1.0], [0.0, len as f64 - 1.0]);
        
        let i = x as i32;
        let j = cmp::min(i+1, len as i32 - 1);
        let h = cmp::max(i-1, 0);
        let k = cmp::min(i+2, len as i32 - 1);
        
        let a = data_points[ h as usize ] as f64;
        let b = data_points[ i as usize ] as f64;
        let c = data_points[ j as usize ] as f64;
        let d = data_points[ k as usize ] as f64;
        
        new.push( cubic_interpolate(a, b, c, d, x - i as f64) );
    }
    
    new
}

/// Read a file and return the contents if valid.

pub fn read_file(file_name: &str) -> Option<String> {
    let mut r = None;
    if let Ok(mut f) = File::open(file_name) {
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);
        r = Some(s)
    }
    r
}
