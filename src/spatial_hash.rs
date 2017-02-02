use std::cmp;

use collision::*;
use vector::Vector;
use particle::Particle;

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

        //for row in 0..self.number_of_rows {
        //    for col in 0..self.number_of_columns {
                // let c = &self.cells[self.get_cell_index(row as i32, col as i32)];
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
                        collisions.push( Collision::new(index1, index2, penetration, normal) );
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

// use std::cmp;
// 
// use collision::*;
// use vector::Vector;
// use particle::Particle;
// 
// pub struct SpatialHash {
//     pub cells: Vec<Vec<(usize, Vector)>>,
//     pub number_of_columns: usize,
//     pub number_of_rows: usize,
//     cell_width: f64,
//     cell_height: f64,
//     radius: f64
// }
// 
// impl SpatialHash {
//     pub fn new(width: f64, height: f64, number_of_columns: usize, number_of_rows: usize, radius: f64) -> SpatialHash {
//         SpatialHash {
//             cells: vec![vec![]; number_of_columns*number_of_rows],
//             number_of_columns: number_of_columns,
//             number_of_rows: number_of_rows,
//             cell_width: width / number_of_columns as f64,
//             cell_height: height / number_of_rows as f64,
//             radius: radius
//         }
//     }
// 
//     // pub fn reset(&mut self) {
//     //     for c in &mut self.cells {
//     //         c.clear();
//     //     }
//     // }
// }
// 
// impl SpatialPartition for SpatialHash {
//     fn insert(&mut self, index: usize, v: Vector) {
//         let row = (v.y / self.cell_height) as usize;
//         let column = (v.x / self.cell_width) as usize;
//         
//         let (r, c) = (cmp::min(row, self.number_of_rows-1), cmp::min(column, self.number_of_columns-1));
// 
//         let cell_index = r*self.number_of_columns + c;
//         self.cells[cell_index].push((index, v));
//         
//         if r > 0 && c > 0 {
//             let cell_index = (r-1)*self.number_of_columns + c-1;
//             self.cells[cell_index].push((index, v));
//         }
//         if r > 0 {
//             let cell_index = (r-1)*self.number_of_columns + c;
//             self.cells[cell_index].push((index, v));
//         }
//         if r > 0 && c < self.number_of_columns-1 {
//             let cell_index = (r-1)*self.number_of_columns + c+1;
//             self.cells[cell_index].push((index, v));
//         }
//         if c < self.number_of_columns-1 {
//             let cell_index = r*self.number_of_columns + c+1;
//             self.cells[cell_index].push((index, v));
//         }
//         if r < self.number_of_rows-1 && c < self.number_of_columns-1 {
//             let cell_index = (r+1)*self.number_of_columns + c+1;
//             self.cells[cell_index].push((index, v));
//         }
//         if r < self.number_of_rows-1 {
//             let cell_index = (r+1)*self.number_of_columns + c;
//             self.cells[cell_index].push((index, v));
//         }
//         if r < self.number_of_rows-1 && r > 0 {
//             let cell_index = (r+1)*self.number_of_columns + c-1;
//             self.cells[cell_index].push((index, v));
//         }
//         if c > 0 {
//             let cell_index = r*self.number_of_columns + c-1;
//             self.cells[cell_index].push((index, v));
//         }
//     }
// 
//     fn clear(&mut self) {
//         for c in &mut self.cells {
//             c.clear();
//         }
//     }
// 
//     fn collision_check(&self) -> Vec<Collision> {
//         let mut collisions = Vec::new();
// 
//         for c in &self.cells {
//             for i in 0..c.len() {
//                 let (index1, p_position) = c[i];
// 
//                 for j in (i+1)..c.len() {
//                     let (index2, q_position) =  c[j];
// 
//                     let normal = (q_position - p_position).normalise();
//                     let penetration = 2.0*self.radius - p_position.distance( q_position );
// 
//                     // if circles are overlapping
//                     if penetration > 0.0 {
//                         // add collision
//                         collisions.push( Collision::new(index1, index2, penetration, normal) );
//                     }
//                 }
//             }
//         }
//         
//         collisions.sort();
//         collisions.dedup();
// 
//         collisions
//     }
//     
//     fn collision_check_with_comparisons(&self) -> (Vec<Collision>, Vec<(usize, usize)>) {
//         let mut collisions = Vec::new();
//         let mut comparisons = Vec::new();
// 
//         for c in &self.cells {
//             for i in 0..c.len() {
//                 let (index1, p_position) = c[i];
// 
//                 for j in (i+1)..c.len() {
//                     let (index2, q_position) =  c[j];
// 
//                     let normal = (q_position - p_position).normalise();
//                     let penetration = 2.0*self.radius - p_position.distance( q_position );
// 
//                     comparisons.push((i, j));
// 
//                     // if circles are overlapping
//                     if penetration > 0.0 {
//                         // add collision
//                         collisions.push( Collision::new(index1, index2, penetration, normal) );
//                     }
//                 }
//             }
//         }
//         
//         collisions.sort();
//         collisions.dedup();
// 
//         (collisions, comparisons)
//     }
// }


// use collision::*;
// use vector::Vector;
// use std::cmp;
// 
// // -------------------------
// // |x |x |x |x \x |x |x |x |
// // |--|--|--|--|--|--|--|--|
// // |x |00|  |  |  |  |  |x |
// // |--|--|--|--|--|--|--|--|
// // |x |  |  |  |  |  |  |x |
// // |--|--|--|--|--|--|--|--|
// // |x |  |  |  |  |  |nn|x |
// // |--|--|--|--|--|--|--|--|
// // |x |x |x |x |x |x |x |x |
// // -------------------------
// 
// 
// 
// pub struct SpatialHash {
//     pub cells: Vec<Vec<(usize, Vector)>>,
//     pub number_of_columns: usize,
//     pub number_of_rows: usize,
//     cell_width: f64,
//     cell_height: f64,
//     radius: f64
// }
// 
// impl SpatialHash {
//     pub fn new(width: f64, height: f64, number_of_columns: usize, number_of_rows: usize, radius: f64) -> SpatialHash {
//         SpatialHash {
//             cells: vec![vec![]; (number_of_columns+2)*(number_of_rows+2)],
//             number_of_columns: number_of_columns,
//             number_of_rows: number_of_rows,
//             cell_width: width / (number_of_columns as f64),
//             cell_height: height / (number_of_rows as f64),
//             radius: radius
//         }
//     }
// 
//     pub fn reset(&mut self) {
//         for c in &mut self.cells {
//             c.clear();
//         }
//     }
//     
//     fn in_cell(&self, v: Vector) -> (i32, i32) {
//         let c = (v.x / self.cell_width) as i32;
//         let r = (v.y / self.cell_height) as i32;
//         
//         (r, c)
//     }
//     
//     fn get_cell_index(&self, r: i32, c: i32) -> usize {
//         // println!("{}, {}", r, c);
//         (r+1) as usize * (self.number_of_columns+2) + (c+1) as usize
//     }
//     
//     fn in_left_set(&self, c: i32, v: Vector) -> bool {
//         let left = c as f64 * self.cell_width;
//         f64::abs( left - v.x ) < self.radius && c > 0
//     }
//     
//     fn in_top_set(&self, r: i32, v: Vector) -> bool {        
//         let up = r as f64 * self.cell_height;        
//         f64::abs( up - v.y ) < self.radius && r > 0
//     }
//     
//     fn check_particle_against_cell(&self, collisions: &mut Vec<Collision>, index: usize, position: Vector, cell: usize, start: usize ) -> Vec<(usize, usize)> {
//         let mut comparisons = Vec::new();
//         
//         let c = &self.cells[cell];
//                 
//         for j in start..c.len() {
//             let (index2, q_position) =  c[j];
// 
//             let normal = (q_position - position).normalise();
//             let penetration = 2.0*self.radius - position.distance( q_position );
// 
//             comparisons.push((index, index2));
// 
//             // if circles are overlapping
//             if penetration > 0.0 {
//                 // add collision
//                 collisions.push( Collision::new(index, index2, penetration, normal) );
//             }
//         }
//         
//         comparisons
//     }
//     
//     pub fn print(&self) {
//         for i in &self.cells {
//             print!("{{ ");
//             for &(ref i, _) in i {
//                 print!("{} ", i);
//             }
//             println!("}}");
//         }
//     }
//     
//     
// }
// 
// impl SpatialPartition for SpatialHash {
//     
//     //  | r |
//     //  
//     //  -------------------     ---
//     //  |   |   top       |      r
//     //  |---|-------------|     ---
//     //  | l |             |
//     //  | e |             | 
//     //  | f |             |
//     //  | t |             |
//     //  ------------------|
//     
//     fn insert(&mut self, index: usize, v: Vector) {        
//         let (r, c) = self.in_cell(v);
//         
//         let (r, c) = (cmp::min(r, self.number_of_rows as i32-1), cmp::min(c, self.number_of_columns as i32-1));
//         
//         let cell_index = if self.in_left_set(c, v) && self.in_top_set(r, v) {
//             self.get_cell_index(r-1, c-1)
//         }
//         else if self.in_left_set(c, v) {
//             self.get_cell_index(r, c-1)
//         }
//         else if self.in_top_set(r, v) {
//             self.get_cell_index(r-1, c)
//         }
//         else {
//             self.get_cell_index(r, c)
//         };
// 
//         self.cells[cell_index].push((index, v))
//     }
// 
//     fn clear(&mut self) {
//         self.reset();
//     }
//     
//     fn collision_check(&self) -> Vec<Collision> {
//         let mut collisions = Vec::new();
//         let mut comparisons = Vec::new();
// 
//         for r in 0..self.number_of_rows {
//             for c in 0..self.number_of_columns {
//                 let r = r as i32;
//                 let c = c as i32;
//                 let cell_index = self.get_cell_index(r, c);
//                 let cell = &self.cells[cell_index];
//                 
//                 let right_index = self.get_cell_index(r, c+1);
//                 let bottom_left_index = self.get_cell_index(r+1, c-1);
//                 let bottom_index = self.get_cell_index(r+1, c);
//                 let bottom_right_index = self.get_cell_index(r+1, c+1);
// 
//                 for i in 0..cell.len() {
//                     let (index, position) = cell[i];
//                     comparisons.append( &mut self.check_particle_against_cell(&mut collisions, index, position, cell_index, i+1 ) );
//                     comparisons.append( &mut self.check_particle_against_cell(&mut collisions, index, position, right_index, 0 ) );
//                     comparisons.append( &mut self.check_particle_against_cell(&mut collisions, index, position, bottom_left_index, 0 ) );
//                     comparisons.append( &mut self.check_particle_against_cell(&mut collisions, index, position, bottom_index, 0 ) );
//                     comparisons.append( &mut self.check_particle_against_cell(&mut collisions, index, position, bottom_right_index, 0 ) );
//                 }
//             }
//         }
// 
//         collisions
//     }    
//     
//     fn collision_check_with_comparisons(&self) -> (Vec<Collision>, Vec<(usize, usize)>) {
//         let mut collisions = Vec::new();
//         let mut comparisons = Vec::new();
// 
//         for r in 0..self.number_of_rows {
//             for c in 0..self.number_of_columns {
//                 let r = r as i32;
//                 let c = c as i32;
// 
//                 let cell_index = self.get_cell_index(r, c);
//                 let cell = &self.cells[cell_index];
//             
//                 let right_index = self.get_cell_index(r, c+1);
//                 let bottom_left_index = self.get_cell_index(r+1, c-1);
//                 let bottom_index = self.get_cell_index(r+1, c);
//                 let bottom_right_index = self.get_cell_index(r+1, c+1);
// 
//                 for i in 0..cell.len() {
//                     let (index, position) = cell[i];
//                     comparisons.append( &mut self.check_particle_against_cell(&mut collisions, index, position, cell_index, i+1 ) );
//                     comparisons.append( &mut self.check_particle_against_cell(&mut collisions, index, position, right_index, 0 ) );
//                     comparisons.append( &mut self.check_particle_against_cell(&mut collisions, index, position, bottom_left_index, 0 ) );
//                     comparisons.append( &mut self.check_particle_against_cell(&mut collisions, index, position, bottom_index, 0 ) );
//                     comparisons.append( &mut self.check_particle_against_cell(&mut collisions, index, position, bottom_right_index, 0 ) );
//                 }
//                 
//                 
//             }
//         }
//         
//         (collisions, comparisons)
//     }
// }
