//! An example showing off the usage of `RustcDecodable` to automatically decode
//! TOML into a Rust `struct`
//!
//! Note that this works similarly with `serde` as well.

#![deny(warnings)]

extern crate toml;

use std::io::prelude::*;
use std::fs::File;

pub fn read_file(file_name: &str) -> Option<String> {
    let mut r = None;
    if let Ok(mut f) = File::open(file_name) {
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);
        r = Some(s)
    }
    r
}

/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
#[derive(Debug, RustcDecodable)]
pub struct Config {
    pub number_of_particles: Option<usize>,
    pub number_of_data_points: Option<usize>,
    pub dt: Option<f64>,
    pub radius: Option<f64>,
    pub gravity: Option<f64>,
    pub restitution: Option<f64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub density_number_of_rows: Option<usize>,
    pub density_number_of_columns: Option<usize>,
    pub collisions: Option<String>,
    pub spatial_hash: Option<SpatialHashConfig>
}

#[derive(Debug, RustcDecodable)]
pub struct SpatialHashConfig {
    pub number_of_rows: Option<usize>,
    pub number_of_columns: Option<usize>,
}

pub fn read_config(config_path: &str) -> Config {
    let toml_str = &*read_file(config_path).unwrap();

    let decoded: Config = toml::decode_str(toml_str).unwrap();
    decoded
}
