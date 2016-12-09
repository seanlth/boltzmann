extern crate boltzmann;
extern crate piston_window;
extern crate rand;

use boltzmann::simulator::Simulator;

fn main() {

    let mut s = Simulator::new(20, 10.0, -9.0, 200, 200, 0.01);

    loop {
        s.update();
    };

}
