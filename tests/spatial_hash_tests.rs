
extern crate boltzmann;
extern crate rand;

use boltzmann::spatial_hash::SpatialHash;
use boltzmann::collision::*;
use boltzmann::vector::Vector;

use std::cmp::Ordering;


struct P(usize, usize);

impl PartialEq for P {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 ||
        self.1 == other.0 && self.0 == other.1
    }
}
impl Eq for P {} 

impl Ord for P {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 < other.0 { Ordering::Less }
        else if self.0 > other.0 { Ordering::Greater }
        else {
            if self.1 < other.1 { Ordering::Less }
            else if self.1 > other.1 { Ordering::Greater }
            else { Ordering::Equal }
        }
    }
}

impl PartialOrd for P {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn collision_check(radius: f64, particles: &Vec<Vector>) -> (Vec<Collision>, Vec<(usize, usize)>) {
    let mut collisions = Vec::new();
    let mut comparisons = Vec::new();

    for i in 0..particles.len() {
        let p = particles[i];
        let p_position = p;

        for j in (i+1)..particles.len() {
            let q = particles[j];
            let q_position = q;

            let normal = (q_position - p_position).normalise();
            let penetration = 2.0*radius - p_position.distance( q_position );

            comparisons.push((i, j));

            // if circles are overlapping
            if penetration > 0.0 {

                // add collision
                collisions.push( Collision::new(i, j, penetration, normal) );
            }
        }
    }

    (collisions, comparisons)
}

fn equal_sets(set1: Vec<P>, set2: Vec<P>) -> Option<(usize, usize)> {

    for (p1, p2) in set1.iter().zip( set2.iter() ) {
        if p1 != p2 {
            println!("{{ {}, {} }}  {{ {}, {} }}", p1.0, p1.1, p2.0, p2.1);

            return Some((p2.0, p2.1))
        }
    }
    None
}

fn collision_pairs(collisions: Vec<Collision>) -> Vec<P> {
    let mut v = Vec::new();
    for c in collisions {
        if c.p1 < c.p2 {
            v.push( P(c.p1, c.p2) );
        }
        else {
            v.push( P(c.p2, c.p1) );
        }
    }
    v
}

fn print(p: &Vec<P>) {
    for i in p {
        println!("{}, {}", i.0, i.1);
    }
}


fn collision_pairs2(comparisons: Vec<(usize, usize)>) -> Vec<P> {
    let mut v = Vec::new();
    for c in comparisons {
        if c.1 < c.0 {
            v.push( P(c.1, c.0) );
        }
        else {
            v.push( P(c.0, c.1) );
        }
    }
    v
}

#[test]
fn test_map_to_cell() {
    let mut hash = SpatialHash::new(100.0, 100.0, 10, 10);

    let (r, c) = hash.map_to_cell(Vector::new(11.0, 11.0));
    
    assert!(r == 1 && c == 1);
}

#[test]
fn test_insert() {
    let mut hash = SpatialHash::new(100.0, 100.0, 10, 10);

    //hash.insert(0, Vector::new(10.0, 10.0), 10.0);
    //hash.insert(1, Vector::new(45.0, 25.0), 5.0);
    //hash.insert(2, Vector::new(45.0, 45.0), 5.0);
    //hash.insert(3, Vector::new(35.0, 75.0), 15.0);
    hash.insert(4, Vector::new(86.60802197907876, 80.45208446109477), 10.0);
    //hash.insert(5, Vector::new(93.07088587906382, 66.96573438766336), 10.0);
    
    //assert!(hash.cells[0][0].0 == 0); 
    //assert!(hash.cells[1][0].0 == 0); 
    //assert!(hash.cells[10][0].0 == 0); 
    //assert!(hash.cells[11][0].0 == 0); 
   
    //assert!(hash.cells[24][0].0 == 1); 
    //assert!(hash.cells[44][0].0 == 2); 
    
    //assert!(hash.cells[62][0].0 == 3);         
    //assert!(hash.cells[63][0].0 == 3);     
    //assert!(hash.cells[64][0].0 == 3);     
    //assert!(hash.cells[72][0].0 == 3);     
    //assert!(hash.cells[73][0].0 == 3);     
    //assert!(hash.cells[74][0].0 == 3);     
    //assert!(hash.cells[82][0].0 == 3);     
    //assert!(hash.cells[83][0].0 == 3);     
    //assert!(hash.cells[84][0].0 == 3);     

    assert!(hash.cells[77][0].0 == 4);         
    assert!(hash.cells[78][0].0 == 4);     
    assert!(hash.cells[79][0].0 == 4);     
    //assert!(hash.cells[87][0].0 == 4);     
    //assert!(hash.cells[88][0].0 == 4);     
    //assert!(hash.cells[89][0].0 == 4);     
    //assert!(hash.cells[98][0].0 == 4);     
}

#[test]
fn test_collisions() {
    let mut hash = SpatialHash::new(100.0, 100.0, 10, 10);

    let mut particles = Vec::new();

    particles.push( Vector::new(86.60802197907876, 80.45208446109477) );
    hash.insert( 0, *particles.last().unwrap(), 10.0 );
    particles.push( Vector::new(93.07088587906382, 66.96573438766336) );
    hash.insert( 1, *particles.last().unwrap(), 10.0 );
    
    let (c_o, c_m) = hash.collision_check_with_comparisons().clone();
    let (c_n_o, c_n_m) = collision_check(10.0, &particles);
    
    let mut v1 = collision_pairs(c_o.clone());
    let mut v2 = collision_pairs(c_n_o);
    v1.sort();
    v2.sort();
     
    let e = equal_sets(v1, v2);
    
    if let Some((p0, p1)) = e {
        println!("{}, {}", particles[p0], particles[p1]);
    }

    assert!(e == None)
}

#[test]
fn test_collisions_random() {
    let mut hash = SpatialHash::new(100.0, 100.0, 10, 10);

    let mut particles = Vec::new();
    
    for i in 0..100 {
        let x = rand::random::<f64>() * 100.0;
        let y = rand::random::<f64>() * 100.0;
        let p = Vector::new(x, y);
        particles.push( p );

        hash.insert( i, p, 10.0 );
    }
    let (c_o, c_m) = hash.collision_check_with_comparisons().clone();
    
    let (c_n_o, c_n_m) = collision_check(10.0, &particles);
    
    let mut v1 = collision_pairs(c_o.clone());
    let mut v2 = collision_pairs(c_n_o);
    v1.sort();
    v2.sort();
    
    let e = equal_sets(v1, v2);

    if let Some((p0, p1)) = e {
        println!("{}, {}", particles[p0], particles[p1]);
    }

    assert!(e == None)
}


/*
#[test]
fn test_collisions_2x2_non_random() {
    let mut spatial_hash = SpatialHash::new(10.0, 10.0, 2, 2, 1.0).unwrap();
    
    let mut particles = Vec::new();
    
    
    // 0
    let p = Vector::new(2.5, 2.5);
    particles.push( p );
    spatial_hash.insert( 0, p );
    
    // 1
    let p = Vector::new(7.5, 2.5);
    particles.push( p );
    spatial_hash.insert( 1, p );
    
    // 2
    let p = Vector::new(2.5, 7.5);
    particles.push( p );
    spatial_hash.insert( 2, p );
    
    // 3
    let p = Vector::new(7.5, 7.5);
    particles.push( p );
    spatial_hash.insert( 3, p );
    
    // 0
    let p = Vector::new(2.5, 5.0);
    particles.push( p );
    spatial_hash.insert( 4, p );
    
    // 0
    let p = Vector::new(5.0, 2.5);
    particles.push( p );
    spatial_hash.insert( 5, p );
    
    // 0
    let p = Vector::new(5.0, 5.0);
    particles.push( p );
    spatial_hash.insert( 6, p );
    
    // 0
    let p = Vector::new(5.0, 5.0);
    particles.push( p );
    spatial_hash.insert( 7, p );
    
    // 0
    let p = Vector::new(0.0, 0.0);
    particles.push( p );
    spatial_hash.insert( 8, p );
    
    // 3
    let p = Vector::new(10.0, 10.0);
    particles.push( p );
    spatial_hash.insert( 9, p );
    
    // 3
    let p = Vector::new(10.0, 10.0);
    particles.push( p );
    spatial_hash.insert( 10, p );
    
    // 0
    let p = Vector::new(4.8, 5.2);
    particles.push( p );
    spatial_hash.insert( 11, p );
    
    // 0
    let p = Vector::new(5.2, 4.8);
    particles.push( p );
    spatial_hash.insert( 12, p );
    
    let (c_o, c_m) = spatial_hash.collision_check_with_comparisons();
    
    let (c_n_o, c_n_m) = collision_check(1.0, &particles);
    
    let mut v1 = collision_pairs(c_o);
    let mut v2 = collision_pairs(c_n_o);
    
    let mut v3 = collision_pairs2(c_m);
    let mut v4 = collision_pairs2(c_n_m);
    
    v1.sort();
    v2.sort();
    
    let e = equal_sets(v1, v2);
    
    assert_eq!(e, true)
}

#[test]
fn test_collisions_2x2() {
    let mut spatial_hash = SpatialHash::new(10.0, 10.0, 2, 2, 1.0).unwrap();
    
    let mut particles = Vec::new();
    
    for i in 0..100 {
        let x = rand::random::<f64>() * 9.0;
        let y = rand::random::<f64>() * 9.0;
        let p = Vector::new(x, y);
        particles.push( p );

        spatial_hash.insert( i, p );
    }
    let (c_o, c_m) = spatial_hash.collision_check_with_comparisons();
    
    let (c_n_o, c_n_m) = collision_check(1.0, &particles);
    
    let mut v1 = collision_pairs(c_o);
    let mut v2 = collision_pairs(c_n_o);
    
    let mut v3 = collision_pairs2(c_m);
    v1.sort();
    v2.sort();
    v3.sort();
        
    let e = equal_sets(v1, v2);
    
    assert_eq!(e, true)
}

#[test]
fn test_collisions_3x3() {
    let mut spatial_hash = SpatialHash::new(30.0, 30.0, 3, 3, 1.0).unwrap();
    
    let mut particles = Vec::new();
    
    for i in 0..100 {
        let x = rand::random::<f64>() * 30.0;
        let y = rand::random::<f64>() * 30.0;
        let p = Vector::new(x, y);
        particles.push( p );

        spatial_hash.insert( i, p );
    }
    let (c_o, c_m) = spatial_hash.collision_check_with_comparisons();
    
    let (c_n_o, c_n_m) = collision_check(1.0, &particles);
    
    let mut v1 = collision_pairs(c_o);
    let mut v2 = collision_pairs(c_n_o);
    v1.sort();
    v2.sort();
    
    print(&v1);
    println!("");
    print(&v2);
    
    let e = equal_sets(v1, v2);
    
    assert_eq!(e, true)
}

#[test]
fn test_collisions_10x10() {
    let mut spatial_hash = SpatialHash::new(100.0, 100.0, 10, 10, 2.0).unwrap();
    
    let mut particles = Vec::new();
    
    for i in 0..100 {
        let x = rand::random::<f64>() * 100.0;
        let y = rand::random::<f64>() * 100.0;
        let p = Vector::new(x, y);
        particles.push( p );

        spatial_hash.insert( i, p );
    }
    let (c_o, c_m) = spatial_hash.collision_check_with_comparisons();
    
    let (c_n_o, c_n_m) = collision_check(2.0, &particles);
    
    let mut v1 = collision_pairs(c_o);
    let mut v2 = collision_pairs(c_n_o);
    v1.sort();
    v2.sort();
    
    print(&v1);
    println!("");
    print(&v2);
    
    let e = equal_sets(v1, v2);
    
    assert_eq!(e, true)
}

#[test]
fn test_collisions_512x512() {
    let mut spatial_hash = SpatialHash::new(512.0, 512.0, 32, 32, 2.0).unwrap();
    
    let mut particles = Vec::new();
    
    for i in 0..10000 {
        let x = rand::random::<f64>() * 512.0;
        let y = rand::random::<f64>() * 512.0;
        let p = Vector::new(x, y);
        particles.push( p );

        spatial_hash.insert( i, p );
    }
    let (c_o, c_m) = spatial_hash.collision_check_with_comparisons();
    
    println!("{}", c_m.len());
    
    let (c_n_o, c_n_m) = collision_check(2.0, &particles);
    
    let mut v1 = collision_pairs(c_o);
    let mut v2 = collision_pairs(c_n_o);
    v1.sort();
    v2.sort();
        
    let e = equal_sets(v1, v2);
    
    assert_eq!(e, true)
}

#[test]
fn test_collisions_parallel_512x512() {
    let mut spatial_hash = SpatialHash::new(512.0, 512.0, 32, 32, 2.0).unwrap();
    
    let mut particles = Vec::new();
    
    for i in 0..10000 {
        let x = rand::random::<f64>() * 512.0;
        let y = rand::random::<f64>() * 512.0;
        let p = Vector::new(x, y);
        particles.push( p );

        spatial_hash.insert( i, p );
    }
    let c_o = spatial_hash.collision_check_parallel();
        
    let (c_n_o, _) = collision_check(2.0, &particles);
    
    let mut v1 = collision_pairs(c_o);
    let mut v2 = collision_pairs(c_n_o);
    v1.sort();
    v2.sort();
        
    let e = equal_sets(v1, v2);
    
    assert_eq!(e, true)
}

*/
