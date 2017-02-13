use std::cmp;

use collision::*;
use vector::Vector;
use scoped_pool::{Pool, Scope};

pub struct SpatialHash {
    pub cells: Vec<Vec<(usize, Vector)>>,
    pub number_of_columns: usize,
    pub number_of_rows: usize,
    cell_width: f64,
    cell_height: f64,
    radius: f64,
    collisions: Vec<Collision>,
    pool: Option<Pool>,
}

impl SpatialHash {
    pub fn new(width: f64, height: f64, number_of_columns: usize, number_of_rows: usize, radius: f64) -> Option<SpatialHash> {
        let max_number_of_columns = (width / radius.ceil()) as usize;
        let max_number_of_rows = (height / radius.ceil()) as usize;
        
        if number_of_rows > max_number_of_rows || number_of_columns > max_number_of_columns {
            None
        }
        else {
            Some(SpatialHash {
                cells: vec![vec![]; (number_of_columns+2)*(number_of_rows+2)],
                number_of_columns: number_of_columns,
                number_of_rows: number_of_rows,
                cell_width: width / number_of_columns as f64,
                cell_height: height / number_of_rows as f64,
                radius: radius,
                collisions: Vec::with_capacity(10000),
                pool: Some(Pool::new(4))
            })
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
    
    fn within(&self, r: i32, c: i32, p: Vector) -> bool {
        let cell_position = Vector::new((c as f64 + 0.5) * self.cell_width, (r as f64 + 0.5) * self.cell_height);  
        
        let b1 = p.x+self.radius >= cell_position.x - self.cell_width/2.0;
        let b2 = p.x-self.radius <= cell_position.x + self.cell_width/2.0;
        let b3 = p.y+self.radius >= cell_position.y - self.cell_height/2.0;
        let b4 = p.y-self.radius <= cell_position.y + self.cell_height/2.0;

        b1 && b2 && b3 && b4
    }
    
    fn contained(&self, r: i32, c: i32, p: Vector) -> bool {
        let cell_position = Vector::new((c as f64 + 0.5) * self.cell_width, (r as f64 + 0.5) * self.cell_height);  
        
        let b1 = p.x-self.radius >= cell_position.x - self.cell_width/2.0;
        let b2 = p.x+self.radius <= cell_position.x + self.cell_width/2.0;
        let b3 = p.y-self.radius >= cell_position.y - self.cell_height/2.0;
        let b4 = p.y+self.radius <= cell_position.y + self.cell_height/2.0;

        b1 && b2 && b3 && b4
    }
    
    fn check_collisions_in_quadrant(&self, row: i32, column: i32, rows: usize, columns: usize) -> Vec<Collision> {
        let mut collisions = Vec::with_capacity(1000);
        for r in 0..rows {
            for c in 0..columns {
                let c = &self.cells[self.get_cell_index(r as i32 + row, c as i32 + column)];
                
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
    

}

impl SpatialPartition for SpatialHash {
    fn insert(&mut self, index: usize, v: Vector) {
        let (row, column) = self.in_cell(v);
        let (r, c) = (cmp::min(row, self.number_of_rows as i32 -1), cmp::min(column, self.number_of_columns as i32 -1));
        
        if self.contained(r, c, v) {
            let cell_index = self.get_cell_index(r as i32, c as i32);
            self.cells[cell_index].push((index, v));
        }
        else {
            for i in -1..2 {
                for j in -1..2 {
                    if self.within(r+i, c+j, v) {
                        let cell_index = self.get_cell_index(r+i as i32, c+j as i32);
                        self.cells[cell_index].push((index, v));
                    }
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
                let (index1, p_position) = c[i];
                
                for j in (i+1)..c.len() {
                    let (index2, q_position) =  c[j];
                    
                    let normal = (q_position - p_position).normalise();
                    let penetration = 2.0*self.radius - p_position.distance( q_position );
                    
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
                scoped.execute(|| { c1 = self.check_collisions_in_quadrant(0, 0, self.number_of_rows/2, self.number_of_columns/2) });
                scoped.execute(|| { c2 = self.check_collisions_in_quadrant(self.number_of_rows as i32/2, 0, self.number_of_rows/2, self.number_of_columns/2) });
                scoped.execute(|| { c3 = self.check_collisions_in_quadrant(0, self.number_of_columns as i32/2, self.number_of_rows/2, self.number_of_columns/2) });
                scoped.execute(|| { c4 = self.check_collisions_in_quadrant(self.number_of_rows as i32/2, self.number_of_columns as i32/2, self.number_of_rows/2, self.number_of_columns/2) });
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
                let (index1, p_position) = c[i];

                for j in (i+1)..c.len() {
                    let (index2, q_position) =  c[j];

                    let normal = (q_position - p_position).normalise();
                    let penetration = 2.0*self.radius - p_position.distance( q_position );

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
