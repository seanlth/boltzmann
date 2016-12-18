use collision::*;
use vector::Vector;
use particle::Particle;

pub struct SpatialHash {
    pub cells: Vec<Vec<(usize, Vector)>>,
    pub width_cells: usize,
    pub height_cells: usize,
    cell_width: f64,
    cell_height: f64,
    radius: f64
}

impl SpatialHash {
    pub fn new(width: f64, height: f64, width_cells: usize, height_cells: usize, radius: f64) -> SpatialHash {
        SpatialHash {
            cells: vec![vec![]; width_cells*height_cells],
            width_cells: width_cells,
            height_cells: height_cells,
            cell_width: width / width_cells as f64,
            cell_height: height / height_cells as f64,
            radius: radius
        }
    }

    pub fn reset(&mut self) {
        for c in &mut self.cells {
            c.clear();
        }
    }

    pub fn add_object(&mut self, index: usize, p: Vector) {
        let i = (p.x / self.cell_width) as usize;
        let j = (p.y / self.cell_height) as usize;

        let cell_index = j*self.width_cells + i;
        self.cells[cell_index].push((index, p));

        if j > 0 {
            let cell_index = (j-1)*self.width_cells + i;
            self.cells[cell_index].push((index, p));
        }
        if j < self.height_cells-1 {
            let cell_index = (j+1)*self.width_cells + i;
            self.cells[cell_index].push((index, p));
        }
        if i > 0 {
            let cell_index = j*self.width_cells + i-1;
            self.cells[cell_index].push((index, p));
        }
        if i < self.width_cells-1 {
            let cell_index = j*self.width_cells + i+1;
            self.cells[cell_index].push((index, p));
        }
    }
}

impl SpatialPartition for SpatialHash {
    fn insert(&mut self, index: usize, p: Vector) {
        self.add_object(index, p);
    }

    fn clear(&mut self) {
        self.reset();
    }

    fn collision_check(&self, particles: &Vec<Particle>) -> Vec<Collision> {
        let mut collisions = Vec::new();

        for c in &self.cells {
            for i in 0..c.len() {
                let (index1, _) = c[i];
                let p_position = particles[index1].get_position();

                for j in (i+1)..c.len() {
                    let (index2, _) =  c[j];
                    let q_position = particles[index2].get_position();

                    let normal = (q_position - p_position).normalise();
                    let penetration = 2.0*self.radius - p_position.distance( q_position );

                    // if circles are overlapping
                    if penetration > 0.0 {
                        // add collision
                        collisions.push( Collision::new(index1, index2, penetration, normal) );
                    }
                }
            }
        }

        collisions
    }
}
