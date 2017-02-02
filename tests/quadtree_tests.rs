

extern crate boltzmann;
extern crate rand;

use boltzmann::quadtree::Quadtree;
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
fn quadtree_test_collisions_2x2_non_random() {
    let mut quadtree = Quadtree::new(10.0, 10.0, 2.0);

    let mut particles = Vec::new();
    
    
    // 0
    let p = Vector::new(2.5, 2.5);
    particles.push( p );
    quadtree.insert( 0, p );
    
    // 1
    let p = Vector::new(7.5, 2.5);
    particles.push( p );
    quadtree.insert( 1, p );
    
    // 2
    let p = Vector::new(2.5, 7.5);
    particles.push( p );
    quadtree.insert( 2, p );
    
    // 3
    let p = Vector::new(7.5, 7.5);
    particles.push( p );
    quadtree.insert( 3, p );
    
    // 0
    let p = Vector::new(2.5, 5.0);
    particles.push( p );
    quadtree.insert( 4, p );
    
    // 0
    let p = Vector::new(5.0, 2.5);
    particles.push( p );
    quadtree.insert( 5, p );
    
    // 0
    let p = Vector::new(5.0, 5.0);
    particles.push( p );
    quadtree.insert( 6, p );
    
    // 0
    let p = Vector::new(5.0, 5.0);
    particles.push( p );
    quadtree.insert( 7, p );
    
    // 0
    let p = Vector::new(0.0, 0.0);
    particles.push( p );
    quadtree.insert( 8, p );
    
    // 3
    let p = Vector::new(10.0, 10.0);
    particles.push( p );
    quadtree.insert( 9, p );
    
    // 3
    let p = Vector::new(10.0, 10.0);
    particles.push( p );
    quadtree.insert( 10, p );
    
    // 0
    let p = Vector::new(4.8, 5.2);
    particles.push( p );
    quadtree.insert( 11, p );
    
    // 0
    let p = Vector::new(5.2, 4.8);
    particles.push( p );
    quadtree.insert( 12, p );
    
    let c_o = quadtree.collision_check();
    
    let (c_n_o, c_n_m) = collision_check(2.0, &particles);
    
    let mut v1 = collision_pairs(c_o);
    let mut v2 = collision_pairs(c_n_o);
    
    // let mut v3 = collision_pairs2(c_m);
    // let mut v4 = collision_pairs2(c_n_m);
    
    v1.sort();
    v2.sort();
    
    let e = equal_sets(v1, v2);
    
    assert_eq!(e, true)
}

#[test]
fn test_collisions_2x2() {    
    let mut quadtree = Quadtree::new(10.0, 10.0, 2.0);

    
    let mut particles = Vec::new();
    
    for i in 0..100 {
        let x = rand::random::<f64>() * 9.0;
        let y = rand::random::<f64>() * 9.0;
        let p = Vector::new(x, y);
        particles.push( p );

        quadtree.insert( i, p );
    }
    let c_o = quadtree.collision_check();
    
    let (c_n_o, c_n_m) = collision_check(2.0, &particles);
    
    let mut v1 = collision_pairs(c_o);
    let mut v2 = collision_pairs(c_n_o);
    
    v1.sort();
    v2.sort();
        
    let e = equal_sets(v1, v2);
    
    assert_eq!(e, true)
}

#[test]
fn test_collisions_3x3() {
    let mut quadtree = Quadtree::new(30.0, 30.0, 2.0);

    let mut particles = Vec::new();
    
    for i in 0..100 {
        let x = rand::random::<f64>() * 30.0;
        let y = rand::random::<f64>() * 30.0;
        let p = Vector::new(x, y);
        particles.push( p );

        quadtree.insert( i, p );
    }
    let (c_o, c_m) = quadtree.collision_check_with_comparisons();
    
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
    let mut quadtree = Quadtree::new(100.0, 100.0, 2.0);

    let mut particles = Vec::new();
    
    for i in 0..100 {
        let x = rand::random::<f64>() * 100.0;
        let y = rand::random::<f64>() * 100.0;
        let p = Vector::new(x, y);
        particles.push( p );

        quadtree.insert( i, p );
    }
    let (c_o, c_m) = quadtree.collision_check_with_comparisons();
    
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
    let mut quadtree = Quadtree::new(2048.0, 1536.0, 20.0);

    let mut particles = Vec::new();
    
    for i in 0..100 {
        let x = rand::random::<f64>() * 2048.0;
        let y = rand::random::<f64>() * 1536.0;
        let p = Vector::new(x, y);
        particles.push( p );

        quadtree.insert( i, p );
    }
    let (c_o, c_m) = quadtree.collision_check_with_comparisons();
    
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
