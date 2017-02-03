use std::cmp;

use collision::*;
use vector::Vector;

pub struct SpatialHash {
    pub cells: Vec<Vec<(usize, Vector)>>,
    pub number_of_columns: usize,
    pub number_of_rows: usize,
    cell_width: f64,
    cell_height: f64,
    radius: f64
}

impl SpatialHash {
    pub fn new(width: f64, height: f64, number_of_columns: usize, number_of_rows: usize, radius: f64) -> SpatialHash {
        SpatialHash {
            cells: vec![vec![]; (number_of_columns+2)*(number_of_rows+2)],
            number_of_columns: number_of_columns,
            number_of_rows: number_of_rows,
            cell_width: width / number_of_columns as f64,
            cell_height: height / number_of_rows as f64,
            radius: radius
        }
    }
    fn in_cell(&self, v: Vector) -> (i32, i32) {
        let c = (v.x / self.cell_width) as i32;
        let r = (v.y / self.cell_height) as i32;
        
        (r, c)
    }
    
    fn get_cell_index(&self, r: i32, c: i32) -> usize {
        (r+1) as usize * (self.number_of_columns+2) + (c+1) as usize
    }
}

impl SpatialPartition for SpatialHash {
    fn insert(&mut self, index: usize, v: Vector) {
        
        let (row, column) = self.in_cell(v);
        
        let (r, c) = (cmp::min(row, self.number_of_rows as i32 -1), cmp::min(column, self.number_of_columns as i32 -1));
        
        for i in -1..2 {
            for j in -1..2 {
                let cell_index = self.get_cell_index(r+i as i32, c+j as i32);
                self.cells[cell_index].push((index, v));
            }
        }
    }

    fn clear(&mut self) {
        for c in &mut self.cells {
            c.clear();
        }
    }

    fn collision_check(&self) -> Vec<Collision> {
        let mut collisions = Vec::new();

        for row in 0..self.number_of_rows {
           for col in 0..self.number_of_columns {
                let c = &self.cells[self.get_cell_index(row as i32, col as i32)];
        // for c in &self.cells {
            for i in 0..c.len() {
                let (index1, p_position) = c[i];

                for j in (i+1)..c.len() {
                    let (index2, q_position) =  c[j];

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
        }
        
        
        collisions.sort();
        collisions.dedup();

        collisions
    }
    
    fn collision_check_with_comparisons(&self) -> (Vec<Collision>, Vec<(usize, usize)>) {
        let mut collisions = Vec::new();
        let mut comparisons = Vec::new();

        for c in &self.cells {
            for i in 0..c.len() {
                let (index1, p_position) = c[i];

                for j in (i+1)..c.len() {
                    let (index2, q_position) =  c[j];

                    let normal = (q_position - p_position).normalise();
                    let penetration = 2.0*self.radius - p_position.distance( q_position );

                    comparisons.push((i, j));

                    // if circles are overlapping
                    if penetration > 0.0 {
                        // add collision
                        collisions.push( Collision::new(index1, index2, penetration, normal) );
                    }
                }
            }
        }
        
        collisions.sort();
        collisions.dedup();

        (collisions, comparisons)
    }
}
