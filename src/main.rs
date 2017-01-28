extern crate boltzmann;

use boltzmann::simulator::Simulator;
use boltzmann::quadtree::Quadtree;
use boltzmann::spatial_hash::SpatialHash;
use boltzmann::vector::Vector;
use boltzmann::collision::SpatialPartition;


extern crate rand;
#[macro_use]
extern crate gfx;
extern crate gfx_app;

use std::time::Instant;

pub use gfx_app::{ColorFormat, DepthFormat};
use gfx::{Bundle, ShaderSet, Primitive, buffer, Bind, Slice};
use gfx::state::Rasterizer;

const NUMBER_OF_PARTICLES: usize = 2;

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

// Declare the vertex format suitable for drawing,
// as well as the constants used by the shaders
// and the pipeline state object format.
gfx_defines!{
    // Data for each particle
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        vel: [f32; 2] = "a_Vel",
        color: [f32; 4] = "a_Color",
    }

    // Aspect ratio to keep particles round
    constant Locals {
        aspect: f32 = "u_Aspect",
        radius: f32 = "radius",
    }

    // Particle render pipeline
    pipeline particles {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        out_color: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    }
}


impl Vertex {
    // Construct new particles far away so they can't be seen initially
    fn new() -> Vertex {
        Vertex {
            pos: [std::f32::INFINITY, std::f32::INFINITY],
            vel: Default::default(),
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

struct App<R: gfx::Resources>{
    bundle: Bundle<R, particles::Data<R>>,
    particles: Vec<Vertex>,
    s: Simulator<SpatialHash>,
    width: u16,
    height: u16,
    aspect: f32,
    time_start: Instant,
}

fn create_shader_set<R: gfx::Resources, F: gfx::Factory<R>>(factory: &mut F, vs_code: &[u8], gs_code: &[u8], ps_code: &[u8]) -> ShaderSet<R> {
    let vs = factory.create_shader_vertex(vs_code).expect("Failed to compile vertex shader");
    let gs = factory.create_shader_geometry(gs_code).expect("Failed to compile geometry shader");
    let ps = factory.create_shader_pixel(ps_code).expect("Failed to compile pixel shader");
    ShaderSet::Geometry(vs, gs, ps)
}

impl<R: gfx::Resources> gfx_app::Application<R> for App<R> {
    fn new<F: gfx::Factory<R>>(mut factory: F, init: gfx_app::Init<R>) -> Self {
        use gfx::traits::FactoryExt;

        // Compute the aspect ratio so that our particles aren't stretched
        let (width, height, _, _) = init.color.get_dimensions();
        let aspect = (height as f32)/(width as f32);

        // Load in our vertex, geometry and pixel shaders
        let vs = gfx_app::shade::Source {
            glsl_150: include_bytes!("../shader/vertex.glsl"),
            .. gfx_app::shade::Source::empty()
        };
        let gs = gfx_app::shade::Source {
            glsl_150: include_bytes!("../shader/geometry.glsl"),
            .. gfx_app::shade::Source::empty()
        };
        let ps = gfx_app::shade::Source {
            glsl_150: include_bytes!("../shader/fragment.glsl"),
            .. gfx_app::shade::Source::empty()
        };

        let shader_set = create_shader_set(
            &mut factory,
            vs.select(init.backend).unwrap(),
            gs.select(init.backend).unwrap(),
            ps.select(init.backend).unwrap(),
        );

        let particles = vec![Vertex::new(); NUMBER_OF_PARTICLES];

        // for p in particles.iter_mut() {
        //     p.color = [rand::random(), rand::random(), rand::random(), rand::random()];
        // }

        let vbuf = factory.create_buffer_dynamic(
            particles.len(), buffer::Role::Vertex, Bind::empty()
        ).expect("Failed to create vertex buffer");

        let slice = Slice::new_match_vertex_buffer(&vbuf);

        let pso = factory.create_pipeline_state(
            &shader_set,
            Primitive::PointList,
            Rasterizer::new_fill(),
            particles::new()
        ).unwrap();

        let data = particles::Data {
            vbuf: vbuf,
            locals: factory.create_constant_buffer(1),
            out_color: init.color,
        };


        App {
            bundle: Bundle::new(slice, pso, data),
            particles: particles,
            s: Simulator::<SpatialHash>::new(NUMBER_OF_PARTICLES, 50.0, -9.8, 0.4, width as f64, height as f64, 0.01),
            width: width,
            height: height,
            aspect: aspect,
            time_start: Instant::now(),
        }
    }

    fn render<C: gfx::CommandBuffer<R>>(&mut self, encoder: &mut gfx::Encoder<R, C>) {
        // Compute the time since last frame
        let delta = self.time_start.elapsed();
        self.time_start = Instant::now();
        let delta = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1000_000_000.0;

        // println!("{}", delta);

        for (i, p) in self.particles.iter_mut().enumerate() {

            let (red, green, blue) = grey_to_jet(self.s.particles[i].get_velocity().magnitude(), 0.0, 707.0);
            p.color[0] = red;
            p.color[1] = green;
            p.color[2] = blue;

            p.pos[0] = (self.s.particles[i].get_position().x as f32 / (self.width as f32 / 2.0)) - 1.0;
            p.pos[1] = (self.s.particles[i].get_position().y as f32 / (self.height as f32 / 2.0)) - 1.0;
        }

        // Pass in the aspect ratio to the geometry shader
        let locals = Locals { aspect: self.aspect, radius: 2.0 * self.s.radius as f32 / self.height as f32};
        encoder.update_constant_buffer(&self.bundle.data.locals, &locals);
        // Update the vertex data with the changes to the particles array
        encoder.update_buffer(&self.bundle.data.vbuf, &self.particles, 0).unwrap();
        // Clear the background to dark blue
        encoder.clear(&self.bundle.data.out_color, [0.0, 0.0, 0.0, 1.0]);
        // Draw the particles!
        self.bundle.encode(encoder);
        self.s.update();
        // self.s.update();
        // self.s.update();
        // self.s.update();
    }
}

pub fn main() {
    use gfx_app::{Application, ApplicationGL, DEFAULT_CONFIG, WrapGL2};
    WrapGL2::<App<_>>::launch("Particle example", DEFAULT_CONFIG);
    // use gfx_app::Application;
    // App::launch_default("Particle example");
}


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



// Copyright 2015 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// extern crate cgmath;
// #[macro_use]
// extern crate gfx;
// extern crate gfx_device_gl;
// extern crate gfx_window_glutin;
// extern crate glutin;
// 
// use gfx::traits::FactoryExt;
// use gfx::{Bundle, texture};
// use gfx::Device;
// use gfx::Factory;
// 
// use cgmath::{Point3, Vector3};
// use cgmath::{Transform, Matrix4};
// 
// pub type ColorFormat = gfx::format::Rgba8;
// pub type DepthFormat = gfx::format::DepthStencil;
// 
// gfx_defines!{
//     vertex Vertex {
//         pos: [f32; 4] = "a_Pos",
//         tex_coord: [f32; 2] = "a_TexCoord",
//     }
// 
//     constant Locals {
//         transform: [[f32; 4]; 4] = "u_Transform",
//     }
// 
//     pipeline pipe {
//         vbuf: gfx::VertexBuffer<Vertex> = (),
//         transform: gfx::Global<[[f32; 4]; 4]> = "u_Transform",
//         locals: gfx::ConstantBuffer<Locals> = "Locals",
//         color: gfx::TextureSampler<[f32; 4]> = "t_Color",
//         out_color: gfx::RenderTarget<ColorFormat> = "Target0",
//         out_depth: gfx::DepthTarget<DepthFormat> =
//             gfx::preset::depth::LESS_EQUAL_WRITE,
//     }
// }
// 
// fn default_view() -> Matrix4<f32> {
//     Transform::look_at(
//         Point3::new(1.5f32, -5.0, 3.0),
//         Point3::new(0f32, 0.0, 0.0),
//         Vector3::unit_z(),
//     )
// }
// 
// impl Vertex {
//     fn new(p: [i8; 3], t: [i8; 2]) -> Vertex {
//         Vertex {
//             pos: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
//             tex_coord: [t[0] as f32, t[1] as f32],
//         }
//     }
// }
// 
// // const TRIANGLE: [Vertex; 4] = [
// //     Vertex { pos: [ -0.5, -0.5, 0.0 ], color: [1.0, 0.0, 0.0] },
// //     Vertex { pos: [  -0.5, 0.5, 0.0 ], color: [0.0, 1.0, 0.0] },
// //     Vertex { pos: [  0.5,  0.5, 0.5 ], color: [0.0, 0.0, 1.0] },
// //     Vertex { pos: [  0.5,  -0.5, 0.5 ], color: [0.0, 0.0, 1.0] }
// // ];
// 
// 
//        
// const index_data: &'static [u16] = &[
//     0,  1,  2,  2,  3,  0, // top
//     4,  5,  6,  6,  7,  4, // bottom
//     8,  9, 10, 10, 11,  8, // right
//     12, 13, 14, 14, 15, 12, // left
//     16, 17, 18, 18, 19, 16, // front
//     20, 21, 22, 22, 23, 20, // back
// ];
// 
// const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];
// 
// pub fn main() {
//     let builder = glutin::WindowBuilder::new()
//         .with_title("Triangle example".to_string())
//         .with_dimensions(1024, 768)
//         .with_vsync();
//         
//     let (window, mut device, mut factory, main_color, mut main_depth) =
//         gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
//         
//     let mut encoder: gfx::Encoder<gfx_device_gl::Resources, _> = factory.create_command_buffer().into();
//     
//     let pso = factory.create_pipeline_simple(
//         include_bytes!("../shader/cube_150.glslv"),
//         include_bytes!("../shader/cube_150.glslf"),
//         pipe::new()
//     ).unwrap();
//     
//     let vertex_data: [Vertex; 24] = [
//         // top (0, 0, 1)
//         Vertex::new([-1, -1,  1], [0, 0]),
//         Vertex::new([ 1, -1,  1], [1, 0]),
//         Vertex::new([ 1,  1,  1], [1, 1]),
//         Vertex::new([-1,  1,  1], [0, 1]),
//         // bottom (0, 0, -1)
//         Vertex::new([-1,  1, -1], [1, 0]),
//         Vertex::new([ 1,  1, -1], [0, 0]),
//         Vertex::new([ 1, -1, -1], [0, 1]),
//         Vertex::new([-1, -1, -1], [1, 1]),
//         // right (1, 0, 0)
//         Vertex::new([ 1, -1, -1], [0, 0]),
//         Vertex::new([ 1,  1, -1], [1, 0]),
//         Vertex::new([ 1,  1,  1], [1, 1]),
//         Vertex::new([ 1, -1,  1], [0, 1]),
//         // left (-1, 0, 0)
//         Vertex::new([-1, -1,  1], [1, 0]),
//         Vertex::new([-1,  1,  1], [0, 0]),
//         Vertex::new([-1,  1, -1], [0, 1]),
//         Vertex::new([-1, -1, -1], [1, 1]),
//         // front (0, 1, 0)
//         Vertex::new([ 1,  1, -1], [1, 0]),
//         Vertex::new([-1,  1, -1], [0, 0]),
//         Vertex::new([-1,  1,  1], [0, 1]),
//         Vertex::new([ 1,  1,  1], [1, 1]),
//         // back (0, -1, 0)
//         Vertex::new([ 1, -1,  1], [0, 0]),
//         Vertex::new([-1, -1,  1], [1, 0]),
//         Vertex::new([-1, -1, -1], [1, 1]),
//         Vertex::new([ 1, -1, -1], [0, 1]),
//     ];
//     
//     let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, index_data);
//     let texels = [[0x20, 0xA0, 0xC0, 0x00]];
//     let (_, texture_view) = factory.create_texture_immutable::<gfx::format::Rgba8>(
//         texture::Kind::D2(1, 1, texture::AaMode::Single), &[&texels]
//         ).unwrap();
// 
//     let sinfo = texture::SamplerInfo::new(
//         texture::FilterMethod::Bilinear,
//         texture::WrapMode::Clamp);
//       
//         
//     let proj = cgmath::perspective(cgmath::Deg(45.0f32), 1.33333, 1.0, 10.0);
//     
//     let mut data = pipe::Data {
//         vbuf: vertex_buffer,
//         transform: (proj.into()),
//         locals: factory.create_constant_buffer(1),
//         color: (texture_view, factory.create_sampler(sinfo)),
//         out_color: main_color,
//         out_depth: main_depth.clone(),
//     };
//     
//     // let mut data = pipe::Data {
//     //     vbuf: vertex_buffer,
//     //     out: main_color
//     // };
// 
//     'main: loop {
//         // loop over events
//         for event in window.poll_events() {
//             match event {
//                 glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
//                 glutin::Event::Closed => break 'main,
//                 glutin::Event::Resized(_width, _height) => {
//                 gfx_window_glutin::update_views(&window, &mut data.out_color, &mut main_depth);
//                 },
//                 _ => {},
//             }
//         }
//         // draw a frame
//         encoder.clear(&data.out_color, CLEAR_COLOR);
//         encoder.draw(&slice, &pso, &data);
//         encoder.flush(&mut device);
//         window.swap_buffers().unwrap();
//         device.cleanup();
//     }
// }
