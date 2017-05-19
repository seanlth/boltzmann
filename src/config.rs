
//! Simulation configuration reading.

extern crate toml;

use read_file;

#[derive(Debug, RustcDecodable)]
pub struct Config {
    pub number_of_particles: Option<usize>,
    pub dt: Option<f64>,
    pub gravity: Option<f64>,
    pub particle_restitution: Option<f64>,
    pub wall_restitution: Option<f64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

pub fn read_config(config_path: &str) -> Config {
    let toml_str = &*read_file(config_path).unwrap();

    let decoded: Config = toml::decode_str(toml_str).unwrap();
    decoded
}
