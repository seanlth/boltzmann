#![feature(test)]

extern crate test;

extern crate boltzmann;
extern crate rand;

use boltzmann::quadtree::Quadtree;
use boltzmann::collision::*;
use boltzmann::vector::Vector;

use std::cmp::Ordering;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_quadtree(b: &mut Bencher) {
        let mut quadtree = Quadtree::new(512.0, 512.0, 2.0);
                
        for i in 0..10000 {
            let x = rand::random::<f64>() * 512.0;
            let y = rand::random::<f64>() * 512.0;
            let p = Vector::new(x, y);
            quadtree.insert( i, p );
        }
        
        b.iter(|| quadtree.collision_check() );
    }
    
    #[bench]
    fn bench_quadtree_parallel(b: &mut Bencher) {
        let mut quadtree = Quadtree::new(512.0, 512.0, 2.0);
                
        for i in 0..10000 {
            let x = rand::random::<f64>() * 512.0;
            let y = rand::random::<f64>() * 512.0;
            let p = Vector::new(x, y);
            quadtree.insert( i, p );
        }
        
        b.iter(|| quadtree.collision_check_parallel() );
    }
}
