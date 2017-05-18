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


