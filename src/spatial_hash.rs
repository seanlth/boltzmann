use std::cmp::min;
use std::cmp::max;

use collision::*;
use vector::Vector;
use particle::Particle;

#[derive(Clone)]
pub struct Partition {
    pub position: Vector,
    pub radius: f64,
    pub width: f64,
    pub height: f64,
    pub particles: Vec<(usize, Vector)>,
    pub border_in_0: Vec<(usize, Vector)>,
    pub border_in_1: Vec<(usize, Vector)>,
    pub border_in_2: Vec<(usize, Vector)>,
    pub border_in_3: Vec<(usize, Vector)>,
    pub border_out_0: Vec<(usize, Vector)>,
    pub border_out_1: Vec<(usize, Vector)>,
    pub border_out_2: Vec<(usize, Vector)>,
    pub border_out_3: Vec<(usize, Vector)>,
}

impl Partition {
    fn new(position: Vector, radius: f64, width: f64, height: f64) -> Partition {
        Partition {
            position: position,
            radius: radius,
            width: width,
            height: height,
            particles: vec![],
            border_in_0: vec![],
            border_in_1: vec![],
            border_in_2: vec![],
            border_in_3: vec![],
            border_out_0: vec![],
            border_out_1: vec![],
            border_out_2: vec![],
            border_out_3: vec![],
        }
    }

    // does the circle fit within the rect completely
    fn in_set_c(&self, p: Vector) -> bool {
        let b1 = p.x-self.radius >= self.position.x - self.width/2.0;
        let b2 = p.x+self.radius <= self.position.x + self.width/2.0;
        let b3 = p.y-self.radius >= self.position.y - self.height/2.0;
        let b4 = p.y+self.radius <= self.position.y + self.height/2.0;

        b1 && b2 && b3 && b4
    }

    // left boundary
    fn in_set_b0(&self, p: Vector) -> bool {
        f64::abs( (self.position.x - self.width/2.0) - p.x ) < self.radius
    }

    // right boundary
    fn in_set_b2(&self, p: Vector) -> bool {
        f64::abs( (self.position.x + self.width/2.0) - p.x ) < self.radius
    }

    // top boundary
    fn in_set_b1(&self, p: Vector) -> bool {
        f64::abs( (self.position.x + self.width/2.0) - p.y ) < self.radius
    }

    // bottom boundary
    fn in_set_b3(&self, p: Vector) -> bool {
        f64::abs( (self.position.y - self.width/2.0) - p.y ) < self.radius
    }

}


pub struct SpatialHash {
    pub partitions: Vec<Partition>,
    // pub cells: Vec<Vec<(usize, Vector)>>,
    // pub border_in_0: Vec<Vec<(usize, Vector)>>,
    // pub border_in_1: Vec<Vec<(usize, Vector)>>,
    // pub border_in_2: Vec<Vec<(usize, Vector)>>,
    // pub border_in_3: Vec<Vec<(usize, Vector)>>,
    //
    // pub border_out_0: Vec<Vec<(usize, Vector)>>,
    // pub border_out_1: Vec<Vec<(usize, Vector)>>,
    // pub border_out_2: Vec<Vec<(usize, Vector)>>,
    // pub border_out_3: Vec<Vec<(usize, Vector)>>,
    pub width_cells: usize,
    pub height_cells: usize,
    cell_width: f64,
    cell_height: f64,
    radius: f64
}

impl SpatialHash {
    pub fn new(width: f64, height: f64, width_cells: usize, height_cells: usize, radius: f64) -> SpatialHash {
        let mut partitions = vec![];

        let cell_width = width / width_cells as f64;
        let cell_height = height / height_cells as f64;

        for j in 0..height_cells {
            for i in 0..width_cells {
                let p = Vector::new( 0.5*cell_width + j as f64 + cell_width, 0.5*cell_width + i as f64 + cell_height );
                partitions.push( Partition::new( p, radius, cell_width, cell_height ) );
            }
        }

        SpatialHash {
            partitions: partitions,
            width_cells: width_cells,
            height_cells: height_cells,
            cell_width: cell_width,
            cell_height: cell_height,
            radius: radius
        }
    }
}

impl SpatialPartition for SpatialHash {
    fn insert(&mut self, index: usize, p: Vector) {
        let mut i = (p.x / self.cell_width) as usize;
        let mut j = (p.y / self.cell_height) as usize;

        i = max(min(i, self.width_cells-1), 0);
        j = max(min(j, self.height_cells-1), 0);

        // println!("i: {}", i);
        // println!("j: {}", j);

        let partition_index = j*self.width_cells + i;
        if self.partitions[partition_index].in_set_c(p) {
            self.partitions[partition_index].particles.push((index, p));
        }
        else {
            if self.partitions[partition_index].in_set_b0(p) {
                self.partitions[partition_index].border_out_0.push((index, p));
                if i > 0 {
                    let partition_index = j*self.width_cells + i-1;
                    self.partitions[partition_index].border_in_2.push((index, p));
                }
            }
            if self.partitions[partition_index].in_set_b1(p) {
                self.partitions[partition_index].border_out_1.push((index, p));
                if j < self.height_cells-1 {
                    let partition_index = (j+1)*self.width_cells + i;
                    self.partitions[partition_index].border_in_3.push((index, p));
                }
            }
            if self.partitions[partition_index].in_set_b2(p) {
                self.partitions[partition_index].border_out_2.push((index, p));
                if i < self.width_cells-1 {
                    let partition_index = j*self.width_cells + i+1;
                    self.partitions[partition_index].border_in_1.push((index, p));
                }
            }
            if self.partitions[partition_index].in_set_b3(p) {
                self.partitions[partition_index].border_out_3.push((index, p));
                if j > 0 {
                    let partition_index = (j-1)*self.width_cells + i;
                    self.partitions[partition_index].border_in_1.push((index, p));
                }
            }
        }

        // self.cells[cell_index].push((index, p));


        // if j > 0 {
        //     let cell_index = (j-1)*self.width_cells + i;
        //     self.border_cells[cell_index].push((index, p));
        // }
        // if j < self.height_cells-1 {
        //     let cell_index = (j+1)*self.width_cells + i;
        //     self.border_cells[cell_index].push((index, p));
        // }
        // if i > 0 {
        //     let cell_index = j*self.width_cells + i-1;
        //     self.border_cells[cell_index].push((index, p));
        // }
        // if i < self.width_cells-1 {
        //     let cell_index = j*self.width_cells + i+1;
        //     self.border_cells[cell_index].push((index, p));
        // }
        //
        // if j > 0 && i > 0 {
        //     let cell_index = (j-1)*self.width_cells + i-1;
        //     self.border_cells[cell_index].push((index, p));
        // }
        // if j < self.height_cells-1 && i > 0 {
        //     let cell_index = (j+1)*self.width_cells + i-1;
        //     self.border_cells[cell_index].push((index, p));
        // }
        // if i < self.width_cells-1 && j > 0 {
        //     let cell_index = (j-1)*self.width_cells + i+1;
        //     self.border_cells[cell_index].push((index, p));
        // }
        // if i < self.width_cells-1 && j < self.height_cells-1 {
        //     let cell_index = (j+1)*self.width_cells + i+1;
        //     self.border_cells[cell_index].push((index, p));
        // }

    }

    fn clear(&mut self) {
        self.partitions.clear();
    }

    fn collision_check(&self, particles: &Vec<Particle>) -> Vec<Collision> {
        let mut collisions = Vec::new();

        collisions
    }

    // fn collision_check(&self, particles: &Vec<Particle>) -> Vec<Collision> {
    //     let mut collisions = Vec::new();
    //
    //     for (cell_index, c) in self.partitions.iter().enumerate() {
    //         for i in 0..c.len() {
    //             let (index1, _) = c[i];
    //             let p_position = particles[index1].get_position();
    //
    //             for j in (i+1)..c.len() {
    //                 let (index2, _) =  c[j];
    //                 let q_position = particles[index2].get_position();
    //
    //                 let normal = (q_position - p_position).normalise();
    //                 let penetration = 2.0*self.radius - p_position.distance( q_position );
    //
    //                 // if circles are overlapping
    //                 if penetration > 0.0 {
    //                     // add collision
    //                     collisions.push( Collision::new(index1, index2, penetration, normal) );
    //                 }
    //             }
    //
    //
    //             for j in 0..self.border_cells[cell_index].len() {
    //                 let (index2, _) =  self.border_cells[cell_index][j];
    //                 let q_position = particles[index2].get_position();
    //
    //                 let normal = (q_position - p_position).normalise();
    //                 let penetration = 2.0*self.radius - p_position.distance( q_position );
    //
    //                 // if circles are overlapping
    //                 if penetration > 0.0 {
    //                     // add collision
    //                     collisions.push( Collision::new(index1, index2, penetration, normal) );
    //                 }
    //             }
    //         }
    //
    //
    //     }
    //
    //     collisions
    // }
}
