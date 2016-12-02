extern crate boltzmann;
extern crate piston_window;
extern crate rand;


use boltzmann::vector;
use boltzmann::simulator::Simulator;

use piston_window::*;

fn main() {

    let mut s = Simulator::new(2, 10.0, 400, 400, 0.01);

    let mut i = 0;
    loop { s.draw(); if i % 20 == 0 {  } i += 1; } ;

    // let mut window: PistonWindow =
    //     WindowSettings::new("Hello Piston!", [640, 480])
    //     .exit_on_esc(true).build().unwrap();
    // while let Some(e) = window.next() {
    //     window.draw_2d(&e, |c, g| {
    //         clear([1.0; 4], g);
    //
    //         // piston_window::ellipse::<R: Into<types::Rectangle>, G>(undefined)
    //
    //         ellipse([1.0, 0.0, 0.0, 1.0], // red
    //                   [100.0, 100.0, 10.0, 10.0],
    //                   c.transform, g);
    //
    //                   ellipse([1.0, 0.0, 0.0, 1.0], // red
    //                             [10.0, 100.0, 10.0, 10.0],
    //                             c.transform, g);
    //     });
    // }
}
