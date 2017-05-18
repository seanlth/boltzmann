
//! Simulation configuration reading.

extern crate toml;

use common::read_file;

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
