// extern crate boltzmann;
// 
// use boltzmann::simulator::Simulator;
// use boltzmann::quadtree::Quadtree;
// use boltzmann::spatial_hash::SpatialHash;
// use boltzmann::vector::Vector;
// use boltzmann::collision::SpatialPartition;
// 
// 
// extern crate rand;
// #[macro_use]
// extern crate gfx;
// extern crate gfx_app;
// 
// use std::time::Instant;
// 
// pub use gfx_app::{ColorFormat, DepthFormat};
// use gfx::{Bundle, ShaderSet, Primitive, buffer, Bind, Slice};
// use gfx::state::Rasterizer;
// 
// const NUMBER_OF_PARTICLES: usize = 2;
// 
// fn grey_to_jet(mut v: f64, min: f64, max: f64) -> (f32, f32, f32)
// {
//     let mut c_r = 1.0;
//     let mut c_g = 1.0;
//     let mut c_b = 1.0;
// 
//     if v < min { v = min; }
//     if v > max { v = max; }
//     let dv = max - min;
// 
//     if v < (min + 0.25 * dv) {
//       c_r = 0.0;
//       c_g = 4.0 * (v - min) / dv;
//     }
//     else if v < (min + 0.5 * dv) {
//       c_r = 0.0;
//       c_b = 1.0 + 4.0 * (min + 0.25 * dv - v) / dv;
//     }
//     else if v < (min + 0.75 * dv) {
//       c_r = 4.0 * (v - min - 0.5 * dv) / dv;
//       c_b = 0.0;
//     }
//     else {
//       c_g = 1.0 + 4.0 * (min + 0.75 * dv - v) / dv;
//       c_b = 0.0;
//     }
// 
//     (c_r as f32, c_g as f32, c_b as f32)
// }
// 
// // Declare the vertex format suitable for drawing,
// // as well as the constants used by the shaders
// // and the pipeline state object format.
// gfx_defines!{
//     // Data for each particle
//     vertex Vertex {
//         pos: [f32; 2] = "a_Pos",
//         vel: [f32; 2] = "a_Vel",
//         color: [f32; 4] = "a_Color",
//     }
// 
//     // Aspect ratio to keep particles round
//     constant Locals {
//         aspect: f32 = "u_Aspect",
//         radius: f32 = "radius",
//     }
// 
//     // Particle render pipeline
//     pipeline particles {
//         vbuf: gfx::VertexBuffer<Vertex> = (),
//         locals: gfx::ConstantBuffer<Locals> = "Locals",
//         out_color: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
//     }
// }
// 
// 
// impl Vertex {
//     // Construct new particles far away so they can't be seen initially
//     fn new() -> Vertex {
//         Vertex {
//             pos: [std::f32::INFINITY, std::f32::INFINITY],
//             vel: Default::default(),
//             color: [1.0, 1.0, 1.0, 1.0],
//         }
//     }
// }
// 
// struct App<R: gfx::Resources>{
//     bundle: Bundle<R, particles::Data<R>>,
//     particles: Vec<Vertex>,
//     s: Simulator<SpatialHash>,
//     width: u16,
//     height: u16,
//     aspect: f32,
//     time_start: Instant,
// }
// 
// fn create_shader_set<R: gfx::Resources, F: gfx::Factory<R>>(factory: &mut F, vs_code: &[u8], gs_code: &[u8], ps_code: &[u8]) -> ShaderSet<R> {
//     let vs = factory.create_shader_vertex(vs_code).expect("Failed to compile vertex shader");
//     let gs = factory.create_shader_geometry(gs_code).expect("Failed to compile geometry shader");
//     let ps = factory.create_shader_pixel(ps_code).expect("Failed to compile pixel shader");
//     ShaderSet::Geometry(vs, gs, ps)
// }
// 
// impl<R: gfx::Resources> gfx_app::Application<R> for App<R> {
//     fn new<F: gfx::Factory<R>>(mut factory: F, init: gfx_app::Init<R>) -> Self {
//         use gfx::traits::FactoryExt;
// 
//         // Compute the aspect ratio so that our particles aren't stretched
//         let (width, height, _, _) = init.color.get_dimensions();
//         let aspect = (height as f32)/(width as f32);
// 
//         // Load in our vertex, geometry and pixel shaders
//         let vs = gfx_app::shade::Source {
//             glsl_150: include_bytes!("../shader/vertex.glsl"),
//             .. gfx_app::shade::Source::empty()
//         };
//         let gs = gfx_app::shade::Source {
//             glsl_150: include_bytes!("../shader/geometry.glsl"),
//             .. gfx_app::shade::Source::empty()
//         };
//         let ps = gfx_app::shade::Source {
//             glsl_150: include_bytes!("../shader/fragment.glsl"),
//             .. gfx_app::shade::Source::empty()
//         };
// 
//         let shader_set = create_shader_set(
//             &mut factory,
//             vs.select(init.backend).unwrap(),
//             gs.select(init.backend).unwrap(),
//             ps.select(init.backend).unwrap(),
//         );
// 
//         let particles = vec![Vertex::new(); NUMBER_OF_PARTICLES];
// 
//         // for p in particles.iter_mut() {
//         //     p.color = [rand::random(), rand::random(), rand::random(), rand::random()];
//         // }
// 
//         let vbuf = factory.create_buffer_dynamic(
//             particles.len(), buffer::Role::Vertex, Bind::empty()
//         ).expect("Failed to create vertex buffer");
// 
//         let slice = Slice::new_match_vertex_buffer(&vbuf);
// 
//         let pso = factory.create_pipeline_state(
//             &shader_set,
//             Primitive::PointList,
//             Rasterizer::new_fill(),
//             particles::new()
//         ).unwrap();
// 
//         let data = particles::Data {
//             vbuf: vbuf,
//             locals: factory.create_constant_buffer(1),
//             out_color: init.color,
//         };
// 
// 
//         App {
//             bundle: Bundle::new(slice, pso, data),
//             particles: particles,
//             s: Simulator::<SpatialHash>::new(NUMBER_OF_PARTICLES, 50.0, -9.8, 0.4, width as f64, height as f64, 0.01),
//             width: width,
//             height: height,
//             aspect: aspect,
//             time_start: Instant::now(),
//         }
//     }
// 
//     fn render<C: gfx::CommandBuffer<R>>(&mut self, encoder: &mut gfx::Encoder<R, C>) {
//         // Compute the time since last frame
//         let delta = self.time_start.elapsed();
//         self.time_start = Instant::now();
//         let delta = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1000_000_000.0;
// 
//         // println!("{}", delta);
// 
//         for (i, p) in self.particles.iter_mut().enumerate() {
// 
//             let (red, green, blue) = grey_to_jet(self.s.particles[i].get_velocity().magnitude(), 0.0, 707.0);
//             p.color[0] = red;
//             p.color[1] = green;
//             p.color[2] = blue;
// 
//             p.pos[0] = (self.s.particles[i].get_position().x as f32 / (self.width as f32 / 2.0)) - 1.0;
//             p.pos[1] = (self.s.particles[i].get_position().y as f32 / (self.height as f32 / 2.0)) - 1.0;
//         }
// 
//         // Pass in the aspect ratio to the geometry shader
//         let locals = Locals { aspect: self.aspect, radius: 2.0 * self.s.radius as f32 / self.height as f32};
//         encoder.update_constant_buffer(&self.bundle.data.locals, &locals);
//         // Update the vertex data with the changes to the particles array
//         encoder.update_buffer(&self.bundle.data.vbuf, &self.particles, 0).unwrap();
//         // Clear the background to dark blue
//         encoder.clear(&self.bundle.data.out_color, [0.0, 0.0, 0.0, 1.0]);
//         // Draw the particles!
//         self.bundle.encode(encoder);
//         self.s.update();
//         // self.s.update();
//         // self.s.update();
//         // self.s.update();
//     }
// }
// 
// pub fn main() {
//     use gfx_app::{Application, ApplicationGL, DEFAULT_CONFIG, WrapGL2};
//     WrapGL2::<App<_>>::launch("Particle example", DEFAULT_CONFIG);
//     // use gfx_app::Application;
//     // App::launch_default("Particle example");
// }


// extern crate boltzmann;
//
// use boltzmann::simulator::Simulator;
// use boltzmann::quadtree::Quadtree;
// use boltzmann::spatial_hash::SpatialHash;
// use boltzmann::vector::Vector;
// use boltzmann::collision::SpatialPartition;
//
//
//
// extern crate rand;
// #[macro_use]
// extern crate gfx;
// extern crate gfx_app;
//
// use std::time::Instant;
//
// pub use gfx_app::{ColorFormat, DepthFormat};
// use gfx::{Bundle, ShaderSet, Primitive, buffer, Bind, Slice};
// use gfx::state::Rasterizer;
//
//
// fn grey_to_jet(mut v: f64, min: f64, max: f64) -> (f32, f32, f32)
// {
//     let mut c_r = 1.0;
//     let mut c_g = 1.0;
//     let mut c_b = 1.0;
//
//     if v < min { v = min; }
//     if v > max { v = max; }
//     let dv = max - min;
//
//     if v < (min + 0.25 * dv) {
//       c_r = 0.0;
//       c_g = 4.0 * (v - min) / dv;
//     }
//     else if v < (min + 0.5 * dv) {
//       c_r = 0.0;
//       c_b = 1.0 + 4.0 * (min + 0.25 * dv - v) / dv;
//     }
//     else if v < (min + 0.75 * dv) {
//       c_r = 4.0 * (v - min - 0.5 * dv) / dv;
//       c_b = 0.0;
//     }
//     else {
//       c_g = 1.0 + 4.0 * (min + 0.75 * dv - v) / dv;
//       c_b = 0.0;
//     }
//
//     (c_r as f32, c_g as f32, c_b as f32)
// }
//
// // Declare the vertex format suitable for drawing,
// // as well as the constants used by the shaders
// // and the pipeline state object format.
// gfx_defines!{
//     // Data for each particle
//     vertex Vertex {
//         pos: [f32; 2] = "a_Pos",
//         vel: [f32; 2] = "a_Vel",
//         color: [f32; 4] = "a_Color",
//     }
//
//     // Aspect ratio to keep particles round
//     constant Locals {
//         aspect: f32 = "u_Aspect",
//         radius: f32 = "radius",
//     }
//
//     // Particle render pipeline
//     pipeline particles {
//         vbuf: gfx::VertexBuffer<Vertex> = (),
//         locals: gfx::ConstantBuffer<Locals> = "Locals",
//         out_color: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
//     }
// }
//
//
// impl Vertex {
//     // Construct new particles far away so they can't be seen initially
//     fn new() -> Vertex {
//         Vertex {
//             pos: [std::f32::INFINITY, std::f32::INFINITY],
//             vel: Default::default(),
//             color: Default::default(),
//         }
//     }
// }
//
// struct App<R: gfx::Resources>{
//     bundle: Bundle<R, particles::Data<R>>,
//     particles: Vec<Vertex>,
//     s: Simulator<Quadtree>,
//     width: u16,
//     height: u16,
//     aspect: f32,
//     time_start: Instant,
// }
//
// fn create_shader_set<R: gfx::Resources, F: gfx::Factory<R>>(factory: &mut F, vs_code: &[u8], gs_code: &[u8], ps_code: &[u8]) -> ShaderSet<R> {
//     let vs = factory.create_shader_vertex(vs_code).expect("Failed to compile vertex shader");
//     let gs = factory.create_shader_geometry(gs_code).expect("Failed to compile geometry shader");
//     let ps = factory.create_shader_pixel(ps_code).expect("Failed to compile pixel shader");
//     ShaderSet::Geometry(vs, gs, ps)
// }
//
// impl<R: gfx::Resources> gfx_app::Application<R> for App<R> {
//     fn new<F: gfx::Factory<R>>(mut factory: F, init: gfx_app::Init<R>) -> Self {
//         use gfx::traits::FactoryExt;
//
//         // Compute the aspect ratio so that our particles aren't stretched
//         let (width, height, _, _) = init.color.get_dimensions();
//         let aspect = (height as f32)/(width as f32);
//
//         // Load in our vertex, geometry and pixel shaders
//         let vs = gfx_app::shade::Source {
//             glsl_150: include_bytes!("../shader/vertex.glsl"),
//             // hlsl_40:  include_bytes!("../data/vs_particle.fx"),
//             .. gfx_app::shade::Source::empty()
//         };
//         let gs = gfx_app::shade::Source {
//             glsl_150: include_bytes!("../shader/geometry.glsl"),
//             // hlsl_40:  include_bytes!("data/gs_particle.fx"),
//             .. gfx_app::shade::Source::empty()
//         };
//         let ps = gfx_app::shade::Source {
//             glsl_150: include_bytes!("../shader/fragment.glsl"),
//             // hlsl_40:  include_bytes!("data/ps_particle.fx"),
//             .. gfx_app::shade::Source::empty()
//         };
//
//         let shader_set = create_shader_set(
//             &mut factory,
//             vs.select(init.backend).unwrap(),
//             gs.select(init.backend).unwrap(),
//             ps.select(init.backend).unwrap(),
//         );
//
//         // Create 4096 particles, using one point vertex per particle
//         let mut particles = vec![Vertex::new(); 4000];
//
//         // Create a dynamic vertex buffer to hold the particle data
//         let vbuf = factory.create_buffer_dynamic(
//             particles.len(), buffer::Role::Vertex, Bind::empty()
//         ).expect("Failed to create vertex buffer");
//         let slice = Slice::new_match_vertex_buffer(&vbuf);
//
//         // Construct our pipeline state
//         let pso = factory.create_pipeline_state(
//             &shader_set,
//             Primitive::PointList,
//             Rasterizer::new_fill(),
//             particles::new()
//         ).unwrap();
//
//         let data = particles::Data {
//             vbuf: vbuf,
//             locals: factory.create_constant_buffer(1),
//             out_color: init.color,
//         };
//
//         // Initialize the particles with random colours
//         // (the alpha value doubles as the particle's "remaining life")
//         for p in particles.iter_mut() {
//             p.color = [rand::random(), rand::random(), rand::random(), rand::random()];
//         }
//
//         App {
//             bundle: Bundle::new(slice, pso, data),
//             particles: particles,
//             s: Simulator::<Quadtree>::new(4000, 5.0, 0.0, 1.0, width as f64, height as f64, 0.001),
//             width: width,
//             height: height,
//             aspect: aspect,
//             time_start: Instant::now(),
//         }
//     }
//
//     fn render<C: gfx::CommandBuffer<R>>(&mut self, encoder: &mut gfx::Encoder<R, C>) {
//         // Compute the time since last frame
//         let delta = self.time_start.elapsed();
//         self.time_start = Instant::now();
//         let delta = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1000_000_000.0;
//
//         println!("{}", delta);
//
//         for (i, p) in self.particles.iter_mut().enumerate() {
//             // Particles are under constant acceleration, so use the exact formulae:
//             // s = ut + 1/2 at^2
//             // v = u + at
//             // p.pos[0] += p.vel[0]*delta;
//             // p.pos[1] += p.vel[1]*delta + 0.5*acc*delta*delta;
//             // p.vel[1] += acc*delta;
//             //
//             // // Fade out steadily
//             // p.color[3] -= 1.0*delta;
//             //
//             // // If particle has faded out completely
//             // if p.color[3] <= 0.0 {
//             //     // Put it back at the emitter with new random parameters
//             //     p.color[3] += 1.0;
//             //     p.pos = [0.0, -1.0];
//             //     let angle: f32 = (rand::random::<f32>()-0.5)*std::f32::consts::PI*0.2;
//             //     let speed: f32 = rand::random::<f32>()*4.0 + 3.0;
//             //     p.vel = [angle.sin()*speed, angle.cos()*speed];
//             // }
//
//             let (red, green, blue) = grey_to_jet(self.s.particles[i].get_velocity().magnitude(), 0.0, 707.0);
//             p.color[0] = red;
//             p.color[1] = green;
//             p.color[2] = blue;
//
//             p.pos[0] = (self.s.particles[i].get_position().x as f32 / (self.width as f32 / 2.0)) - 1.0;
//             p.pos[1] = (self.s.particles[i].get_position().y as f32 / (self.height as f32 / 2.0)) - 1.0;
//         }
//
//         // Pass in the aspect ratio to the geometry shader
//         let locals = Locals { aspect: self.aspect, radius: 2.0 * self.s.radius as f32 / self.height as f32};
//         encoder.update_constant_buffer(&self.bundle.data.locals, &locals);
//         // Update the vertex data with the changes to the particles array
//         encoder.update_buffer(&self.bundle.data.vbuf, &self.particles, 0).unwrap();
//         // Clear the background to dark blue
//         encoder.clear(&self.bundle.data.out_color, [0.0, 0.0, 0.0, 1.0]);
//         // Draw the particles!
//         self.bundle.encode(encoder);
//         self.s.update();
//         // self.s.update();
//         // self.s.update();
//         // self.s.update();
//     }
// }
//
// pub fn main() {
//     use gfx_app::{Application, ApplicationGL, DEFAULT_CONFIG, WrapGL2};
//     WrapGL2::<App<_>>::launch("Particle example", DEFAULT_CONFIG);
//     // use gfx_app::Application;
//     // App::launch_default("Particle example");
// }

#[macro_use]
extern crate glium;
extern crate boltzmann;

use glium::glutin;
use glium::DisplayBuild;
use glium::Program;
use glium::backend::glutin_backend::GlutinFacade;
use glium::VertexBuffer;
use glium::IndexBuffer;
use glium::index::NoIndices;
use glium::Surface;

use std::io::prelude::*;
use std::fs::File;

use boltzmann::simulator::Simulator;
use boltzmann::collision::SpatialPartition;
use boltzmann::spatial_hash::SpatialHash;
use boltzmann::quadtree::Quadtree;
use boltzmann::vector::Vector;
use boltzmann::particle::Particle;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    colour: [f32; 4],
}

fn read_file(file_name: &str) -> Option<String> {
    let mut r = None;
    if let Ok(mut f) = File::open(file_name) {
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);
        r = Some(s)
    }
    r
}

fn grey_to_jet(mut v: f64, min: f64, max: f64) -> (f32, f32, f32)
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

fn setup_glium() -> Option<(GlutinFacade, Program)> {
    implement_vertex!(Vertex, position, colour);
    
    let display = glutin::WindowBuilder::new()
        .with_title("Particles")
        .with_depth_buffer(24)
        .build_glium()
        .unwrap();
    
    if let Some(v) = read_file("shader/vertex.glsl") {
        if let Some(f) = read_file("shader/fragment.glsl") {
            if let Some(g) = read_file("shader/geometry.glsl") {
                let out = glium::Program::from_source( &display, &*v, &*f, Some(&*g) );
                if let Ok(program) = out {
                    return Some((display, program))
                }
                else if let Err(error) = out {
                     println!("{:?}", error);
                }
            }
            else {
                println!("Couldn't read geometry shader");
            }
        }
        else {
            println!("Couldn't read fragment shader");
        }
    }
    else {
        println!("Couldn't read vertex shader");
    }
        
    None
}

fn create_buffer(display: &GlutinFacade, number_of_particles: usize) -> (VertexBuffer<Vertex>, NoIndices) {
        
    let mut vertices = Vec::new();

    for _ in 0..number_of_particles {
        vertices.push( Vertex {
            position: [ 0.0, 0.0 ],
            colour: [ 1.0, 1.0, 1.0, 1.0 ]
        } );
    }    
    ( VertexBuffer::dynamic(display, &vertices).unwrap(), NoIndices(glium::index::PrimitiveType::Points) )
}

fn draw<T: SpatialPartition>(display: GlutinFacade, program: Program, mut simulator: Simulator<T>, vertex_buffer: VertexBuffer<Vertex>, index_buffer: NoIndices) {    
    
    let params = glium::DrawParameters {
        // point_size: Some(1.0),
        blend: glium::Blend::alpha_blending(),
        .. Default::default()    
    };
    
    let uniforms = uniform! {
        u_Aspect: simulator.height as f32 / simulator.width as f32,
        radius: 2.0 * simulator.radius as f32 / simulator.height as f32
    };
        
    loop {                                    
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        
        let mut ps = Vec::new();
        for p in &simulator.particles {
            let position = p.get_position();
            let velocity = p.get_velocity();
            let (red, green, blue) = grey_to_jet(velocity.magnitude(), 0.0, 707.0);
            
            let x = ( position.x as f32 / ( (simulator.width as f32 / 2.0) ) ) - 1.0;
            let y = ( position.y as f32 / ( (simulator.height as f32 / 2.0) ) ) - 1.0;
            
            ps.push( Vertex {
                position: [ x, y ],
                colour: [ red, green, blue, 1.0 ]
            } );  
        }        
                            
        vertex_buffer.write(&*ps);

        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &params).unwrap();
        
        target.finish().unwrap();
        
        simulator.update();
    }
}

fn main() {
    let number_of_particles = 10000;
    
    if let Some( (display, program) ) = setup_glium() {
        let (width, height): (u32, u32) = display.get_window().unwrap().get_inner_size_pixels().unwrap();
        let simulator = Simulator::<Quadtree>::new(number_of_particles, 5.0, 0.0, 1.0, width as f64, height as f64, 0.01);
        let (vertex_buffer, index_buffer) = create_buffer(&display, number_of_particles);
        draw(display, program, simulator, vertex_buffer, index_buffer);
    }
    
    
}
