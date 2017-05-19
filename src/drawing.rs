//! Particle viewer, density viewer, graph plotter. 

use glium;
use glium::Program;
use glium::backend::glutin_backend::GlutinFacade;
use glium::VertexBuffer;
use glium::IndexBuffer;
use glium::index::NoIndices;
use glium::Surface;
use glium::index::PrimitiveType;
use glium::DrawParameters;

use vector::Vector;

use scale;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    colour: [f32; 4],
    radius: f32
}

implement_vertex!(Vertex, position, colour, radius);

pub struct Plotter<'a> {
    context: (GlutinFacade, Program),
    buffer: (VertexBuffer<Vertex>, IndexBuffer<u16>),
    data: Vec<(f64, f64)>,
    parameters: DrawParameters<'a>,
    x_range: (f64, f64),
    y_range: (f64, f64)
}

impl<'a> Plotter<'a> {
    pub fn new(context: (GlutinFacade, Program), data: Vec<(f64, f64)>, line_width: f32, point_size: f32) -> Plotter<'a> {
        let parameters = DrawParameters {
            line_width: Some(line_width),
            point_size: Some(point_size),
            blend: glium::Blend::alpha_blending(),
            .. Default::default()    
        };
        
        let len = data.len();
        
        let mut p = Plotter {
            buffer: (VertexBuffer::empty_dynamic(&context.0, data.len()).unwrap(), IndexBuffer::empty_dynamic(&context.0, PrimitiveType::LineStrip, data.len()).unwrap()),
            context: context,
            data: data.clone(),
            parameters: parameters,
            x_range: (0.0, len as f64),
            y_range: (0.0, 1.0)
        };
        p.update(data);
        p
    }
    
    pub fn x_range(mut self, range: (f64, f64)) -> Self { self.x_range = range; self }
    pub fn y_range(mut self, range: (f64, f64)) -> Self { self.y_range = range; self }
    
    pub fn update(&mut self, data: Vec<(f64, f64)>) {
        self.data = data.clone();
            
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        
        for (i, &(x, y)) in self.data.iter().enumerate() {
            let px = scale(x as f64, [self.x_range.0, self.x_range.1 - 1.0], [-1.0, 1.0]) as f32;
            let py = scale(y as f64, [self.y_range.0, self.y_range.1], [-1.0, 1.0]) as f32;
                        
            vertices.push( Vertex {
                position: [ px, py ],
                colour: [ 0.0, 1.0, 1.0, 1.0 ],
                radius: 0.0
            } );
            indices.push(i as u16);
        }
        self.buffer.0.write(&*vertices);
        self.buffer.1.write(&*indices);
    } 
    
    pub fn plot(&self) {
        let mut target = self.context.0.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
                
        target.draw(&self.buffer.0, &self.buffer.1, &self.context.1, &glium::uniforms::EmptyUniforms, &self.parameters).unwrap();
        target.draw(&self.buffer.0, NoIndices(PrimitiveType::Points), &self.context.1, &glium::uniforms::EmptyUniforms, &self.parameters).unwrap();

        target.finish().unwrap();        
    }
}

pub struct Particles<'a> {
    context: (GlutinFacade, Program),
    buffer: VertexBuffer<Vertex>,
    data: Vec<(Vector, (f32, f32, f32), f32)>,
    parameters: DrawParameters<'a>,
    uniforms: glium::uniforms::UniformsStorage<'a, f32, glium::uniforms::EmptyUniforms>,
    width: f64,
    height: f64
}

impl<'a> Particles<'a> {
    pub fn new(context: (GlutinFacade, Program), data: Vec<(Vector, (f32, f32, f32), f32)>, width: f64, height: f64) -> Particles<'a> {
        let parameters = DrawParameters {
            blend: glium::Blend::alpha_blending(),
            .. Default::default()    
        };
        let uniforms = uniform! {
            u_Aspect: height as f32 / width as f32,
        };
                
        let mut p = Particles {
            buffer: VertexBuffer::empty_dynamic(&context.0, data.len()).unwrap(),
            context: context,
            data: data.clone(),
            parameters: parameters,
            uniforms: uniforms,
            width: width,
            height: height,
        };
        p.update(data); 
        p
    }
        
    pub fn update(&mut self, data: Vec<(Vector, (f32, f32, f32), f32)>) {
        self.data = data.clone();
            
        let mut vertices = Vec::new();
        
        for (_, &(v, (r, g, b), radius)) in self.data.iter().enumerate() {
            let px = scale(v.x as f64, [0.0, self.width], [-1.0, 1.0]) as f32;
            let py = scale(v.y as f64, [0.0, self.height], [-1.0, 1.0]) as f32;
                        
            vertices.push( Vertex {
                position: [ px, py ],
                colour: [ r, g, b, 1.0 ],
                radius: radius * 2.0 / self.height as f32
            } );
        }
        self.buffer.write(&*vertices);
    } 
    
    pub fn draw(&self) {
        let mut target = self.context.0.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
                
        target.draw(&self.buffer, NoIndices(PrimitiveType::Points), &self.context.1, &self.uniforms, &self.parameters).unwrap();

        target.finish().unwrap();        
    }
}


pub struct Density<'a> {
    context: (GlutinFacade, Program),
    buffer: VertexBuffer<Vertex>,
    data: Vec<(f32, f32, f32)>,
    parameters: DrawParameters<'a>,
    uniforms: glium::uniforms::UniformsStorage<'a, f32, glium::uniforms::UniformsStorage<'a, f32, glium::uniforms::EmptyUniforms>>,
    radius: f64,
    number_of_rows: usize,
    number_of_columns: usize
}

impl<'a> Density<'a> {
    pub fn new(context: (GlutinFacade, Program), data: Vec<(f32, f32, f32)>, number_of_rows: usize, number_of_columns: usize) -> Density<'a> {
        let parameters = DrawParameters {
            blend: glium::Blend::alpha_blending(),
            .. Default::default()    
        };
        let uniforms = uniform! {
            u_Aspect: number_of_rows as f32 / number_of_columns as f32,
            radius: 1.0 / (number_of_columns as f32)
        };
         
        let mut p = Density {
            buffer: VertexBuffer::empty_dynamic(&context.0, data.len()).unwrap(),
            context: context,
            data: data.clone(),
            parameters: parameters,
            uniforms: uniforms,
            radius: 1.0 / (number_of_columns as f64),
            number_of_rows: number_of_rows,
            number_of_columns: number_of_columns
        };
        p.update(data); 
        p
    }
    
    pub fn update(&mut self, data: Vec<(f32, f32, f32)>) {
        self.data = data.clone();
            
        let mut vertices = Vec::new();
        
        for (i, &(red, green, blue)) in self.data.iter().enumerate() {
            let r = i / self.number_of_columns;
            let c = i % self.number_of_columns;
                                 
            let px = scale(c as f64, [0.0, self.number_of_columns as f64-1.0], [-1.0 + self.radius as f64, 1.0 - self.radius as f64]) as f32;
            let py = scale(r as f64, [0.0, self.number_of_rows as f64-1.0], [-1.0 + self.radius as f64, 1.0 - self.radius as f64]) as f32;
                        
            vertices.push( Vertex {
                position: [ px, py ],
                colour: [ red, green, blue, 1.0 ],
                radius: 0.0
            } );
        }
        self.buffer.write(&*vertices);
    } 
    
    pub fn draw(&self) {
        
        let mut target = self.context.0.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
                
        target.draw(&self.buffer, NoIndices(PrimitiveType::Points), &self.context.1, &self.uniforms, &self.parameters).unwrap();

        target.finish().unwrap(); 
    }
}
