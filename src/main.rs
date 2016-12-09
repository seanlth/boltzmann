extern crate boltzmann;
extern crate piston_window;
extern crate rand;

use boltzmann::simulator::Simulator;

fn main() {

    let mut s = Simulator::new(40, 5.0, 0.0, 200, 200, 0.01);

    let mut t = 0;
    loop {
        s.update();
        let vs = s.velociies();
        for v in vs { if t % 10 == 0 { println!("{}", v); } }
        t += 1;
    };

}
