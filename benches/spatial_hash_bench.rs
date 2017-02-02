#![feature(test)]

extern crate test;

extern crate boltzmann;
extern crate rand;

use boltzmann::spatial_hash::SpatialHash;
use boltzmann::collision::*;
use boltzmann::vector::Vector;

use std::cmp::Ordering;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_spatial_hash(b: &mut Bencher) {
        let mut spatial_hash = SpatialHash::new(2048.0, 1536.0, 800, 800, 5.0);
                
        for i in 0..10000 {
            let x = rand::random::<f64>() * 2048.0;
            let y = rand::random::<f64>() * 1536.0;
            let p = Vector::new(x, y);
            spatial_hash.insert( i, p );
        }
        
        b.iter(|| spatial_hash.collision_check() );
    }
}
