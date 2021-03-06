use std::io::prelude::*;
use std::fs::File;
use std::cmp;


// [start1, end1] => [start2, end2]
// [start1 - start1, end1 - start1 ] => [start2, end2]
// [0, end1 - start1] => [start2, end2]
// [0, end1 - start1 / end1 - start1 ] => [start2, end2]
// [0, 1] => [start2, end2]
// [0, 1 * (end2 - start2) ] => [start2, end2]
// [start2, end2 - start2 + start2] => [start2, end2]
pub fn scale(x: f64, scale1: [f64; 2], scale2: [f64; 2]) -> f64 {
    let a = (x - scale1[0]) / (scale1[1] - scale1[0]);
    let b = a * (scale2[1] - scale2[0]);
    b + scale2[0]
}

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


pub fn cubic_interpolate( a: f64, b: f64, c: f64, d: f64, w: f64 ) -> f64 {

    let a0 = d - c - a + b;
    let a1 = a - b - a0;
    let a2 = c - a;
    let a3 = b;

   f64::max(0.0, a0*w*w*w + a1*w*w + a2*w + a3)
}

pub fn linear_interpolate(a: f64, b: f64, w: f64) -> f64 {
	a * w + b * (1.0 - w)
}


pub fn linear_interpolate_vec(vs: &Vec<f64>, n: usize) -> Vec<f64> {
    let mut new = Vec::new();
    
    let len = vs.len();
    
    for index in 0..n {
        let x = scale(index as f64, [0.0, n as f64 - 1.0], [0.0, len as f64 - 1.0]);
        let i = x as i32;
        let j = cmp::min(i+1, len as i32 - 1);

        let a = vs[ i as usize ] as f64;
        let b = vs[ j as usize ] as f64;
                
        new.push( linear_interpolate(b, a, x - i as f64) );
    }
    
    new
}

pub fn cubic_interpolate_vec(vs: &Vec<f64>, n: usize) -> Vec<f64> {
    let mut new = Vec::new();
    
    let len = vs.len();
    
    for index in 0..n {
        let x = scale(index as f64, [0.0, n as f64 - 1.0], [0.0, len as f64 - 1.0]);
        
        let i = x as i32;
        let j = cmp::min(i+1, len as i32 - 1);
        let h = cmp::max(i-1, 0);
        let k = cmp::min(i+2, len as i32 - 1);
        
        let a = vs[ h as usize ] as f64;
        let b = vs[ i as usize ] as f64;
        let c = vs[ j as usize ] as f64;
        let d = vs[ k as usize ] as f64;
        
        new.push( cubic_interpolate(a, b, c, d, x - i as f64) );
    }
    
    new
}

pub fn read_file(file_name: &str) -> Option<String> {
    let mut r = None;
    if let Ok(mut f) = File::open(file_name) {
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);
        r = Some(s)
    }
    r
}
