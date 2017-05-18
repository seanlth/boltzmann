//! Spatial hash implementation.

use common::{scale, bound};
use collision::*;
use vector::Vector;
use scoped_pool::Pool;

pub struct SpatialHash {
    pub cells: Vec<Vec<(usize, f64, Vector)>>,
    pub number_of_columns: usize,
    pub number_of_rows: usize,
    cell_width: f64,
    cell_height: f64,
    width: f64,
    height: f64,
    collisions: Vec<Collision>,
    pool: Option<Pool>,
}

impl SpatialHash {


    /// Construct a new SpatialHash.
    /// Maps a simulation domain with width and height to 
    /// a grid of cells.   

    pub fn new(width: f64, height: f64, number_of_columns: usize, number_of_rows: usize) -> SpatialHash {
        SpatialHash {
            cells: vec![vec![]; (number_of_columns)*(number_of_rows)],
            number_of_columns: number_of_columns,
            number_of_rows: number_of_rows,
            cell_width: width / number_of_columns as f64,
            cell_height: height / number_of_rows as f64,
            width: width,
            height: height,
            collisions: Vec::with_capacity(10000),
            pool: Some(Pool::new(4))
        }
    }
    
    /// Map a position to a cell in the grid.
    /// The boundary cells are ignored 
    ///
    /// Where: 
    ///     (x, y) -> [r, c] 
    ///     (0, 0) -> [1, 1]
    ///     (width, height) -> [#Rows-1, #Cols-1]
    ///
    /// The resulting row and column are guaranteed 
    /// to be bounded by the grid size

    pub fn map_to_cell(&self, v: Vector) -> (i32, i32) {
        
        // map position to rows and columns
        let r = scale(v.y, [0.0, self.height], [0.0, self.number_of_rows as f64]);
        let c = scale(v.x, [0.0, self.width], [0.0, self.number_of_columns as f64]);

        // bound the row and column
        let b_r = bound(r, [0.0, self.number_of_rows as f64 - 1.0]) as i32;
        let b_c = bound(c, [0.0, self.number_of_columns as f64 - 1.0]) as i32;

        (b_r, b_c)
    }

    /// Get the (x,y) position of the cell. 

    fn get_cell_position(&self, row: i32, column: i32) -> Vector {
        Vector::new((column as f64 + 0.5) * self.cell_width, 
                    (row as f64 + 0.5) * self.cell_height)
    }
    
    /// Get the index of the cell.

    fn get_cell_index(&self, row: i32, column: i32) -> usize {
        row as usize * (self.number_of_columns) + column as usize
    }

    /// Is the particle completely contained within the grid cell. 
       
    fn is_contained(&self, row: i32, column: i32, v: Vector, radius: f64) -> bool {
        let cell_position = self.get_cell_position(row, column);
        
        // bounds 
        let b1 = v.x-radius >= cell_position.x - self.cell_width/2.0;
        let b2 = v.x+radius <= cell_position.x + self.cell_width/2.0;
        let b3 = v.y-radius >= cell_position.y - self.cell_height/2.0;
        let b4 = v.y+radius <= cell_position.y + self.cell_height/2.0;

        b1 && b2 && b3 && b4
    }

    /// Get the min and max points of a particle.

    fn get_min_max(v: Vector, radius: f64) -> (Vector, Vector) {
        let r = Vector::new(radius, radius);
        let min = v - r;
        let max = v + r;
        
        (min, max)
    }

    /// Get all collision in a subgrid. 
    
    fn check_collisions_in_subgrid(&self, row: i32, column: i32, 
                                   rows: usize, columns: usize) -> Vec<Collision> {
        let mut collisions = Vec::with_capacity(1000);
        for r in 0..rows {
            for c in 0..columns {
                let c = &self.cells[self.get_cell_index(r as i32 + row, c as i32 + column)];
                
                for (i, &(index1, radius1, p_position)) in c.iter().enumerate() {
                    
                    for &(index2, radius2, q_position) in c.iter().skip((i+1)) {
                        //let (index2, radius2, q_position) =  c[j];
                        
                        let normal = (q_position - p_position).normalise();
                        let penetration = (radius1 + radius2) - p_position.distance( q_position );
                        
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
    

}

impl SpatialPartition for SpatialHash {

    // Insert the particle into the grid. 
    // If the particle is completely enclosed by a cell 
    // it is only added to that cell, otherwise the particle
    // gets added to the cells that it overlaps. 

    fn insert(&mut self, index: usize, v: Vector, radius: f64) {
        let (r, c) = self.map_to_cell(v);

        // particle fits in the cell completely
        if self.is_contained(r, c, v, radius) {
            let cell_index = self.get_cell_index(r as i32, c as i32);
            self.cells[cell_index].push((index, radius, v));
        }
        else {
            
            let (min, max) = Self::get_min_max(v, radius);
            let ((r1, c1), (r2, c2)) = (self.map_to_cell(min), self.map_to_cell(max));
            
            for i in r1..r2+1 {
                for j in c1..c2+1 {
                    let cell_index = self.get_cell_index(i as i32, j as i32);
                    self.cells[cell_index].push((index, radius, v));
                }
            }
        }
    }

    fn clear(&mut self) {
        self.collisions.clear();
        for c in &mut self.cells {
            c.clear();
        }
    }

    fn collision_check(&mut self) -> &Vec<Collision> {
        for c in &self.cells {
            for i in 0..c.len() {
                let (index1, radius1, p_position) = c[i];
                
                for j in (i+1)..c.len() {
                    let (index2, radius2, q_position) =  c[j];
                    
                    let normal = (q_position - p_position).normalise();
                    let penetration = (radius1 + radius2) - p_position.distance( q_position );
                    
                    // if circles are overlapping
                    if penetration > 0.0 {
                        // add collision
                        self.collisions.push( Collision::new(index1, index2, penetration, normal) );
                    }
                }
            }
        }
        
        
        self.collisions.sort();
        self.collisions.dedup();

        &self.collisions
    }
    
    fn collision_check_parallel(&mut self) -> &Vec<Collision> {
        
        let mut c1 = Vec::new();
        let mut c2 = Vec::new();
        let mut c3 = Vec::new();
        let mut c4 = Vec::new();

        if let Some(ref p) = self.pool {
            p.scoped(|scoped| {
                scoped.execute(|| { c1 = self.check_collisions_in_subgrid(0, 0, self.number_of_rows/2, self.number_of_columns/2) });
                scoped.execute(|| { c2 = self.check_collisions_in_subgrid(self.number_of_rows as i32/2, 0, self.number_of_rows/2, self.number_of_columns/2) });
                scoped.execute(|| { c3 = self.check_collisions_in_subgrid(0, self.number_of_columns as i32/2, self.number_of_rows/2, self.number_of_columns/2) });
                scoped.execute(|| { c4 = self.check_collisions_in_subgrid(self.number_of_rows as i32/2, self.number_of_columns as i32/2, self.number_of_rows/2, self.number_of_columns/2) });
            });
            
            self.collisions.append( &mut c1 );
            self.collisions.append( &mut c2 );
            self.collisions.append( &mut c3 );
            self.collisions.append( &mut c4 );
        }
        
        self.collisions.sort();
        self.collisions.dedup();

        &self.collisions
    }
    
    fn collision_check_with_comparisons(&mut self) -> (&Vec<Collision>, Vec<(usize, usize)>) {
        let mut comparisons = Vec::new();

        for c in &self.cells {
            for i in 0..c.len() {
                let (index1, radius1, p_position) = c[i];

                for j in (i+1)..c.len() {
                    let (index2, radius2, q_position) =  c[j];

                    let normal = (q_position - p_position).normalise();
                    let penetration = (radius1 + radius2) - p_position.distance( q_position );

                    comparisons.push((i, j));

                    // if circles are overlapping
                    if penetration > 0.0 {
                        // add collision
                        self.collisions.push( Collision::new(index1, index2, penetration, normal) );
                    }
                }
            }
        }
        
        self.collisions.sort();
        self.collisions.dedup();

        (&self.collisions, comparisons)
    }
}
