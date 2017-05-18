
#[macro_use]
extern crate glium;
extern crate boltzmann;
extern crate rand;

use std::cmp;

use glium::glutin;
use glium::DisplayBuild;
use glium::Program;
use glium::backend::glutin_backend::GlutinFacade;

use boltzmann::simulator::Simulator;
use boltzmann::collision::SpatialPartition;
use boltzmann::vector::*;
use boltzmann::attribute::*;
use boltzmann::common::*;
use boltzmann::drawing::*;

#[allow(unused_imports)]
use boltzmann::spatial_hash::SpatialHash;
#[allow(unused_imports)]
use boltzmann::quadtree::Quadtree;
use boltzmann::config::*;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    colour: [f32; 4],
}

implement_vertex!(Vertex, position, colour);


fn histogram_1d(data: Vec<f64>, bins: usize) -> (f64, Vec<f64>) {
    let mut histogram = vec![0.0; bins];
    let max_data = data.iter().cloned().fold(0./0., f64::max);
    let mut max = 0.0;

    for v in data {
        let i = scale(v, [0.0, max_data], [0.0, bins as f64 - 1.0]) as usize;
        histogram[ cmp::max(i, 0) ] += 1.0;
        max = f64::max( histogram[i], max );
    }
    
    (max, histogram)
} 

fn histogram_2d_temp(data: Vec<(Vector, f64)>, max_x: f64, max_y: f64, number_of_rows: usize, number_of_columns: usize) -> (f64, Vec<f64>) {
    let mut histogram = vec![0.0; number_of_rows*number_of_columns];
    let mut max = 0.0;

    for p in data {
        let r = scale(p.0.y, [0.0, max_y], [0.0, number_of_rows as f64]) as usize;
        let c = scale(p.0.x, [0.0, max_x], [0.0, number_of_columns as f64]) as usize;
        
        histogram[r*number_of_columns + c] += p.1;
        max = f64::max( histogram[r*number_of_columns + c], max );
    }
    
    (max, histogram)
}

fn histogram_2d(data: Vec<Vector>, max_x: f64, max_y: f64, number_of_rows: usize, number_of_columns: usize) -> (f64, Vec<f64>) {
    let mut histogram = vec![0.0; number_of_rows*number_of_columns];
    let mut max = 0.0;

    for p in data {
        let r = scale(p.y, [0.0, max_y], [0.0, number_of_rows as f64]) as usize;
        let c = scale(p.x, [0.0, max_x], [0.0, number_of_columns as f64]) as usize;
        
        histogram[r*number_of_columns + c] += 1.0;
        max = f64::max( histogram[r*number_of_columns + c], max );
    }
    
    (max, histogram)
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

fn p_x() -> f64 { rand::random::<f64>() }
fn p_y() -> f64 { rand::random::<f64>() }
fn v_x() -> f64 { rand::random::<f64>() }
fn v_y() -> f64 { rand::random::<f64>() }
fn radii() -> f64 { rand::random::<f64>() * 10.0 + 1.0 }

fn particle_data<T: SpatialPartition>(simulation: &Simulator<T>) -> Vec<(Vector, (f32, f32, f32), f32)> {
    let a = simulation.attribute(0).get_data();
    let (min, max) = simulation.attribute(0).data_bounds();

    let mut ps = Vec::new();
    for (i, p) in simulation.particles.iter().enumerate() {
        let position = p.get_position();
        let (red, green, blue) = grey_to_jet(a[i], min, max);
        ps.push( (position, (red, green, blue), p.radius as f32) );  
    }        
    ps
}

fn plotter_data<T: SpatialPartition>(simulation: &Simulator<T>, number_of_data_points: usize) -> (f64, Vec<(f64, f64)>) {    
    let xs: Vec<f64> = (0..number_of_data_points).map(|n| n as u64 as f64).collect();
    let data = simulation.velocities();
    let (max, histogram) = histogram_1d(data, 50);
    let ys = linear_interpolate_vec(&histogram, number_of_data_points);
    let ds: Vec<(f64, f64)> = xs.into_iter().zip( ys.into_iter() ).collect();
    
    (max, ds)
}

fn density_data( data: Vec<f64>, max: f64 ) -> Vec<(f32, f32, f32)> {
    let mut new = Vec::new();
    
    for d in data {
        new.push( grey_to_jet(d, 0.0, max) );
    }
    
    new
}

fn histogram_data<T: SpatialPartition>(simulation: &Simulator<T>) -> Vec<(Vector, f64)> {
    let ds: Vec<(Vector, f64)> = simulation.positions().into_iter().zip( simulation.attribute(0).get_data().clone().into_iter() ).collect();
    ds
}


fn test() {
    let mut hash = SpatialHash::new(100 as f64, 100 as f64, 10, 10);    
    hash.insert(0, Vector::new(10.0, 10.0), 10.0);
    hash.insert(1, Vector::new(15.0, 15.0), 1.0);

    let colls = hash.collision_check();
    
    for c in colls {
        println!("{}, {}", c.p1, c.p2);
    }
}


fn main() {
    //test();
    //return;

    let config = read_config("simulation_config.toml");
    
    // define simulation constants
    let number_of_particles = config.number_of_particles.unwrap();
    let number_of_data_points = config.number_of_data_points.unwrap();
    let dt = config.dt.unwrap();
    let radius = config.radius.unwrap();
    let gravity = config.gravity.unwrap();
    let restitution = config.restitution.unwrap();
    let width = config.width.unwrap();
    let height = config.height.unwrap();
    let density_number_of_rows = config.density_number_of_rows.unwrap();
    let density_number_of_columns = config.density_number_of_columns.unwrap();
    
    let simulator_display = create_window(width, height, 664, 50, "boltzmann");
    let density_display = create_window(width, height, 152, 50, "density");
    let plotter_display = create_window(width, height/3, 664, 586, "plot");


    let simulator_program = compile_shaders(&simulator_display, "shader/vertex.glsl", "shader/fragment.glsl", Some("shader/geometry.glsl")).unwrap();
    let density_program = compile_shaders(&density_display, "shader/density_vertex.glsl", "shader/density_fragment.glsl", Some("shader/density_geometry.glsl")).unwrap();
    let plotter_program = compile_shaders(&plotter_display, "shader/plotter_vertex.glsl", "shader/plotter_fragment.glsl", None).unwrap();      
            
            
    let quad = Quadtree::new(width as f64, height as f64, radius);
    let hash = SpatialHash::new(width as f64, height as f64, 10, 10);
    let mut simulator = Simulator::new(hash, number_of_particles, gravity, restitution, restitution, width as f64, height as f64, 200.0, dt);

    simulator.probabilistic_initial_conditions( (&p_x, &p_y), (&v_x, &v_y), &radii );
    
    simulator.bind_attribute::<speed_attr>();
    //simulator.set_attribute(0, 1.0, 0.0);
    //simulator.set_attribute(0, 2.0, 0.5);

    simulator.particles[0].radius = 15.0;

    simulator.update();
    
    //let (max, data) = plotter_data(&simulator, number_of_data_points);
    
    
    // let histogra_2d = histogram_2d(simulator.positions(), width as f64, height as f64, density_number_of_rows, density_number_of_columns);
   
    //let histogram_2d = histogram_2d_temp(histogram_data(&simulator), width as f64, height as f64, density_number_of_rows, density_number_of_columns);
    let mut particles = Particles::new((simulator_display, simulator_program), particle_data(&simulator), width as f64, height as f64);
    //let mut plotter = Plotter::new((plotter_display, plotter_program), data, 2.0, 5.0)
                      //.y_range((0.0, max));
    //let mut density = boltzmann::drawing::Density::new((density_display, density_program), density_data(histogram_2d.1, histogram_2d.0 ), density_number_of_rows, density_number_of_columns);
    

    // run simulation 
    loop {
        simulator.update();
        //simulator.update();
        //simulator.update();
        //simulator.update();
        //simulator.update();
        //simulator.update();
        //simulator.update();

        particles.draw();
        particles.update(particle_data(&simulator));
        //plotter.plot();
        //let (max, data) = plotter_data(&simulator, number_of_data_points);
        //plotter.update( data );
        //plotter = plotter.y_range((0.0, max));
        //density.draw();
        //let histogram_2d = histogram_2d_temp(histogram_data(&simulator), width as f64, height as f64, density_number_of_rows, density_number_of_columns);
        // let histogram_2d = histogram_2d(simulator.positions(), width as f64, height as f64, density_number_of_rows, density_number_of_columns);
        //density.update( density_data(histogram_2d.1, histogram_2d.0 ) )
    }
}
                                                                           

