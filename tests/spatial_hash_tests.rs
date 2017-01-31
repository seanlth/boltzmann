
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

fn equal_sets(set1: Vec<P>, set2: Vec<P>) -> bool {
    // if set1.len() != set2.len() {
    //     if set1.len() > set2.len() {
    //         println!("Set1 more");
    //     }
    //     else {
    //         println!("Set2 more");
    //     }
    //     return false;
    // }
    
    for (p1, p2) in set1.iter().zip( set2.iter() ) {
        if p1 != p2 {
            println!("{{ {}, {} }}  {{ {}, {} }}", p1.0, p1.1, p2.0, p2.1);
            return false
        }
    }
    true
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
fn test_collisions_2x2_non_random() {
    let mut spatial_hash = SpatialHash::new(10.0, 10.0, 2, 2, 2.0);
    
    let mut particles = Vec::new();
    
    let p = Vector::new(2.5, 2.5);
    particles.push( p );
    spatial_hash.insert( 0, p );
    
    let p = Vector::new(7.5, 2.5);
    particles.push( p );
    spatial_hash.insert( 1, p );
    
    let p = Vector::new(2.5, 7.5);
    particles.push( p );
    spatial_hash.insert( 2, p );
    
    let p = Vector::new(7.5, 7.5);
    particles.push( p );
    spatial_hash.insert( 3, p );
    
    let p = Vector::new(2.5, 5.0);
    particles.push( p );
    spatial_hash.insert( 4, p );
    
    let p = Vector::new(5.0, 2.5);
    particles.push( p );
    spatial_hash.insert( 5, p );
    
    let p = Vector::new(5.0, 5.0);
    particles.push( p );
    spatial_hash.insert( 6, p );
    
    let p = Vector::new(5.0, 5.0);
    particles.push( p );
    spatial_hash.insert( 7, p );
    
    let p = Vector::new(0.0, 0.0);
    particles.push( p );
    spatial_hash.insert( 8, p );
    
    let p = Vector::new(10.0, 10.0);
    particles.push( p );
    spatial_hash.insert( 9, p );
    
    let p = Vector::new(10.0, 10.0);
    particles.push( p );
    spatial_hash.insert( 10, p );
    
    let p = Vector::new(4.8, 5.2);
    particles.push( p );
    spatial_hash.insert( 11, p );
    
    let p = Vector::new(5.2, 4.8);
    particles.push( p );
    spatial_hash.insert( 12, p );
    
    let (c_o, c_m) = spatial_hash.collision_check2();
    
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
fn test_collisions_2x2() {
    let mut spatial_hash = SpatialHash::new(10.0, 10.0, 2, 2, 2.0);
    
    let mut particles = Vec::new();
    
    for i in 0..100 {
        let x = rand::random::<f64>() * 9.0;
        let y = rand::random::<f64>() * 9.0;
        let p = Vector::new(x, y);
        particles.push( p );

        spatial_hash.insert( i, p );
    }
    let (c_o, c_m) = spatial_hash.collision_check2();
    
    let (c_n_o, c_n_m) = collision_check(2.0, &particles);
    
    let mut v1 = collision_pairs(c_o);
    let mut v2 = collision_pairs(c_n_o);
    
    let mut v3 = collision_pairs2(c_m);
    // let mut v4 = collision_pairs(c_n_o);
    v1.sort();
    v2.sort();
    v3.sort();
    
    spatial_hash.print();
    let asd= spatial_hash.cells[0].len() + spatial_hash.cells[1].len() + spatial_hash.cells[2].len() + spatial_hash.cells[3].len();
    // println!("{}", asd);
    // print(&v1);
    // println!("");
    // print(&v2);
    // println!("");
    print(&v3);
    
    let e = equal_sets(v1, v2);
    
    assert_eq!(e, true)
}

#[test]
fn test_collisions_3x3() {
    let mut spatial_hash = SpatialHash::new(30.0, 30.0, 3, 3, 2.0);
    
    let mut particles = Vec::new();
    
    for i in 0..100 {
        let x = rand::random::<f64>() * 30.0;
        let y = rand::random::<f64>() * 30.0;
        let p = Vector::new(x, y);
        particles.push( p );

        spatial_hash.insert( i, p );
    }
    let (c_o, c_m) = spatial_hash.collision_check2();
    
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
fn test_collisions_10x10() {
    let mut spatial_hash = SpatialHash::new(100.0, 100.0, 10, 10, 2.0);
    
    let mut particles = Vec::new();
    
    for i in 0..100 {
        let x = rand::random::<f64>() * 100.0;
        let y = rand::random::<f64>() * 100.0;
        let p = Vector::new(x, y);
        particles.push( p );

        spatial_hash.insert( i, p );
    }
    let (c_o, c_m) = spatial_hash.collision_check2();
    
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
fn test_collisions_2048x1536() {
    let mut spatial_hash = SpatialHash::new(2048.0, 1536.0, 10, 10, 20.0);
    
    let mut particles = Vec::new();
    
    for i in 0..100 {
        let x = rand::random::<f64>() * 2048.0;
        let y = rand::random::<f64>() * 1536.0;
        let p = Vector::new(x, y);
        particles.push( p );

        spatial_hash.insert( i, p );
    }
    let (c_o, c_m) = spatial_hash.collision_check2();
    
    let (c_n_o, c_n_m) = collision_check(20.0, &particles);
    
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
