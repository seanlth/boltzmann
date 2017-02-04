
#[macro_use]
extern crate glium;
extern crate boltzmann;
extern crate rand;

use std::io::prelude::*;
use std::fs::File;
use std::cmp;

use glium::glutin;
use glium::DisplayBuild;
use glium::Program;
use glium::backend::glutin_backend::GlutinFacade;
use glium::VertexBuffer;
use glium::IndexBuffer;
use glium::index::NoIndices;
use glium::Surface;
use glium::index::PrimitiveType;


use boltzmann::simulator::Simulator;
use boltzmann::collision::SpatialPartition;
use boltzmann::vector::*;

#[allow(unused_imports)]
use boltzmann::spatial_hash::SpatialHash;
#[allow(unused_imports)]
use boltzmann::quadtree::Quadtree;

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

// [start1, end1] => [start2, end2]
// [start1 - start1, end1 - start1 ] => [start2, end2]
// [0, end1 - start1] => [start2, end2]
// [0, end1 - start1 / end1 - start1 ] => [start2, end2]
// [0, 1] => [start2, end2]
// [0, 1 * (end2 - start2) ] => [start2, end2]
// [start2, end2 - start2 + start2] => [start2, end2]
fn scale(x: f64, scale1: [f64; 2], scale2: [f64; 2]) -> f64 {
    let a = (x - scale1[0]) / (scale1[1] - scale1[0]);
    let b = a * (scale2[1] - scale2[0]);
    b + scale2[0]
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


fn cubic_interpolate( a: f64, b: f64, c: f64, d: f64, w: f64 ) -> f64 {

    let a0 = d - c - a + b;
    let a1 = a - b - a0;
    let a2 = c - a;
    let a3 = b;

   f64::max(0.0, a0*w*w*w + a1*w*w + a2*w + a3)
}

pub fn linear_interpolate(a: f64, b: f64, w: f64) -> f64 {
	a * w + b * (1.0 - w)
}

fn velocity_density(velocities: Vec<f64>, bins: usize) -> Box<Fn(f64) -> f64> {
    let mut density = vec![0.0; bins];
    let max_velocity = velocities.iter().cloned().fold(0./0., f64::max);
    let mut max = 0.0;

    for v in velocities {
        
        let i = ( bins as f64 * v / max_velocity ) as usize;
        density[ std::cmp::min(i, bins-1) ] += 1.0;
        if density[ std::cmp::min(i, bins-1) ] > max {
            max = density[ std::cmp::min(i, bins-1) ];
        }
    }

    let f = move |x: f64| -> f64 {
        let i = x as usize;
        let j = i + 1;
        let h = if i > 0 {i - 1} else { 0 };
        let k = i + 2;
        let a = density[ std::cmp::max(h, 0) ] as f64;
        let b = density[ std::cmp::min(i, bins-1) ] as f64 ;
        let c = density[ std::cmp::min(j, bins-1) ] as f64 ;
        let d = density[ std::cmp::min(k, bins-1) ] as f64 ;

        let v = x - i as f64;

        cubic_interpolate(a, b, c, d, v) / max
    };

    Box::new(f)
}

fn particle_density(positions: Vec<Vector>, width: f64, height: f64, number_of_rows: usize, number_of_columns: usize) -> (Vec<usize>, usize) {
    let mut bins = vec![0; number_of_rows*number_of_columns];
    let mut max = 0;
    
    let cell_width = width / number_of_columns as f64;
    let cell_height = height / number_of_rows as f64;

    for p in positions {
        let r = (p.y / cell_height) as usize;
        let c = (p.x / cell_width) as usize;
        

        bins[r*number_of_columns + c] += 1;
        max = cmp::max( bins[r*number_of_columns + c], max );
    }
    
    (bins, max)
}

fn create_window(width: u32, height: u32, x: i32, y: i32, title: &str) -> GlutinFacade {
    let w = glutin::WindowBuilder::new()
        .with_title(title)
        .with_dimensions(width, height)
        .with_depth_buffer(24)
        .build_glium()
        .unwrap();
        
    w.get_window().unwrap().set_position(x, y);
    
    w
}

fn compile_shaders(display: &GlutinFacade, vertex_shader: &str, fragment_shader: &str, geometry_shader: Option<&str>) -> Option<Program> {
    let vertex_shader_source = read_file(vertex_shader);
    let fragment_shader_source = read_file(fragment_shader);
    
    let mut program = Err(glium::ProgramCreationError::CompilationError("Couldn't open vertex or fragmnet shader".to_string()));
    
    if let (Some(v), Some(f)) = (vertex_shader_source, fragment_shader_source) {
        if let Some(geometry_shader) = geometry_shader {
            if let Some(g) = read_file(geometry_shader) {
                program = Program::from_source( display, &*v, &*f, Some(&*g) );
            }
        }
        else {
            program = Program::from_source( display, &*v, &*f, None );
        }
    }
        
    if let Err(error) = program {
         println!("{:?}", error);
    }
    else if let Ok(p) = program {
        return Some(p);
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

fn create_plot_buffer(display: &GlutinFacade, number_of_points: usize) -> (VertexBuffer<Vertex>, IndexBuffer<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    
    for i in 0..number_of_points {
        vertices.push( Vertex {
            position: [ 0.0, 0.0 ],
            colour: [ 1.0, 1.0, 1.0, 1.0 ]
        } );
        indices.push( i as u16 );
    }
    ( VertexBuffer::dynamic(display, &vertices).unwrap(), IndexBuffer::new(display, PrimitiveType::LineStrip, &*indices).unwrap() )

}

fn display<T: SpatialPartition>(display: &GlutinFacade, program: &Program, simulator: &Simulator<T>, vertex_buffer: &VertexBuffer<Vertex>, index_buffer: &NoIndices) {
    let params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        .. Default::default()    
    };
    
    let uniforms = uniform! {
        u_Aspect: simulator.height as f32 / simulator.width as f32,
        radius: 2.0 * simulator.radius as f32 / simulator.height as f32
    };
        
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

    target.draw(vertex_buffer, index_buffer, program, &uniforms, &params).unwrap();    
    target.finish().unwrap();
    
}

fn plot<T: SpatialPartition>(display: &GlutinFacade, program: &Program, simulator: &Simulator<T>, vertex_buffer: &VertexBuffer<Vertex>, index_buffer: &IndexBuffer<u16>, number_of_points: usize) {
    let params = glium::DrawParameters {
        line_width: Some(2.0),
        point_size: Some(5.0),
        blend: glium::Blend::alpha_blending(),
        .. Default::default()    
    };
        
    let mut target = display.draw();
    target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

    let number_of_bins = 50;
    
    let vs = velocity_density(simulator.velocities(), number_of_bins);
    let mut points = Vec::new();
    for i in 0..number_of_points {
        let x = scale(i as f64, [0.0, number_of_points as f64-1.0], [-1.0, 1.0]) as f32;
        let t = vs(scale(i as f64, [0.0, number_of_points as f64-1.0], [0.0, number_of_bins as f64 -1.0]));
        let y = scale(t, [0.0, 1.0], [-1.0, 0.8]) as f32;
        
        points.push( Vertex {
            position: [ x, y ],
            colour: [ 0.0, 1.0, 1.0, 1.0 ]
        } );
    }
    vertex_buffer.write(&*points);

    target.draw(vertex_buffer, index_buffer, program, &glium::uniforms::EmptyUniforms, &params).unwrap();
    target.draw(vertex_buffer, glium::index::NoIndices(glium::index::PrimitiveType::Points), program, &glium::uniforms::EmptyUniforms, &params).unwrap();

    target.finish().unwrap();        
}


fn density<T: SpatialPartition>(display: &GlutinFacade, program: &Program, simulator: &Simulator<T>, vertex_buffer: &VertexBuffer<Vertex>, number_of_rows: usize, number_of_columns: usize) {
    
    let params = glium::DrawParameters {
        // point_size: Some(5.0),
        blend: glium::Blend::alpha_blending(),
        .. Default::default()    
    };
    
    let uniforms = uniform! {
        u_Aspect: simulator.height as f32 / simulator.width as f32,
        radius: 1.0 * (simulator.width / number_of_columns as f64 - 1.0) as f32 / simulator.height as f32
    };
    let mut target = display.draw();
    target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

    
    let (bins, max) = particle_density(simulator.positions(), simulator.width, simulator.height, number_of_rows, number_of_columns);
    let mut points = Vec::new();
    for i in 0..number_of_columns*number_of_rows {
        let r = i / number_of_columns;
        let c = i % number_of_columns;
        
        let x = scale(c as f64, [-1.0, number_of_columns as f64], [-1.0, 1.0]) as f32;
        let y = scale(r as f64, [-1.0, number_of_rows as f64], [-1.0, 1.0]) as f32;

        println!("{}", i);
        let (r, g, b) = grey_to_jet(bins[i] as f64, 0.0, max as f64);            
        points.push( Vertex {
            position: [ x, y ],
            colour: [ r, g, b, 1.0 ]
        } );
    }
    vertex_buffer.write(&*points);

    target.draw(vertex_buffer, glium::index::NoIndices(glium::index::PrimitiveType::Points), program, &uniforms, &params).unwrap();

    target.finish().unwrap();        
}


fn main() {
    implement_vertex!(Vertex, position, colour);
    
    // define simulation constants
    let number_of_particles = 5000;
    let number_of_data_points = 1000;
    let dt = 0.0005;
    let radius = 2.0;
    let gravity = 0.0;
    let restitution = 1.0;
    let width = 512;
    let height = 512;
    
    // create windows 
    let simulator_display = create_window(width, height, 664, 50, "boltzmann");
    let density_display = create_window(width, height, 152, 50, "density");
    let plotter_display = create_window(width, height/3, 664, 586, "plot");

    // compile shaders 
    let simulator_program = compile_shaders(&simulator_display, "shader/vertex.glsl", 
                                            "shader/fragment.glsl", Some("shader/geometry.glsl"));
    let plotter_program = compile_shaders(&plotter_display, "shader/plotter_vertex.glsl", 
                                          "shader/plotter_fragment.glsl", None);
    let density_program = compile_shaders(&density_display, "shader/density_vertex.glsl", 
                                            "shader/density_fragment.glsl", Some("shader/density_geometry.glsl"));
    
    if let (Some(s), Some(p), Some(d)) = (simulator_program, plotter_program, density_program) {
        let quad = Quadtree::new(width as f64, height as f64, radius);
        let hash = SpatialHash::new(width as f64, height as f64, 25, 25, radius).unwrap();
        let mut simulator = Simulator::new(quad, number_of_particles, radius, gravity, restitution, width as f64, height as f64, dt);
        let (vertex_buffer, index_buffer) = create_buffer(&simulator_display, number_of_particles);
        let (plotter_vertex_buffer, plotter_index_buffer) = create_plot_buffer(&plotter_display, number_of_data_points);
        let (density_vertex_buffer, _) = create_buffer(&density_display, 2500);

        loop {
            simulator.update();
            display(&simulator_display, &s, &simulator, &vertex_buffer, &index_buffer);
            plot(&plotter_display, &p, &simulator, &plotter_vertex_buffer, &plotter_index_buffer, number_of_data_points);
            density(&density_display, &d, &simulator, &density_vertex_buffer, 50, 50);
        }
    
    }
}
