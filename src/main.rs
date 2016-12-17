#[macro_use] extern crate conrod;
extern crate boltzmann;
extern crate graphics;
extern crate rand;

use boltzmann::simulator::Simulator;
use boltzmann::quadtree::Quadtree;
use boltzmann::vector::Vector;

use conrod::backend::piston::{self, Window, WindowEvents, OpenGL};
use conrod::backend::piston::event::UpdateEvent;
use graphics::*;

widget_ids! {
    struct Ids { canvas, plot }
}


fn grey_to_jet(mut v: f64, min: f64, max: f64) -> (f32, f32, f32)
{
    let mut c_r = 1.0;
    let mut c_g = 1.0;
    let mut c_b = 1.0;

    if v < min { v = min; }
    if v > max { v = max; }
    let dv = max - min;

    if v < (min + 0.25 * dv) {
      c_r = 0.0;
      c_g = 4.0 * (v - min) / dv;
    }
    else if v < (min + 0.5 * dv) {
      c_r = 0.0;
      c_b = 1.0 + 4.0 * (min + 0.25 * dv - v) / dv;
    }
    else if v < (min + 0.75 * dv) {
      c_r = 4.0 * (v - min - 0.5 * dv) / dv;
      c_b = 0.0;
    }
    else {
      c_g = 1.0 + 4.0 * (min + 0.75 * dv - v) / dv;
      c_b = 0.0;
    }

    (c_r as f32, c_g as f32, c_b as f32)
}

fn cubic_interpolate( a: f64, b: f64, c: f64, d: f64, w: f64 ) -> f64 {

    let a0 = d - c - a + b;
    let a1 = a - b - a0;
    let a2 = c - a;
    let a3 = b;

   f64::max(0.0, a0*w*w*w + a1*w*w + a2*w + a3)
}

pub fn linear_interpolate(a: f64, b: f64, w: f64) -> f64 {
	a * w + b * (1.0 - w)
}

fn draw_quad_tree(quadtree: &Quadtree, c: &Context, g: &mut piston::gfx::G2d) {
    if let Some((ref c1, ref c2, ref c3, ref c4)) = quadtree.children {
        draw_quad_tree(c1, c, g);
        draw_quad_tree(c2, c, g);
        draw_quad_tree(c3, c, g);
        draw_quad_tree(c4, c, g);
    }

    let (radius, colour) = if quadtree.level == 0 { (1.5, [1.0, 1.0, 1.0, 1.0]) }
                 else if quadtree.level == 1 { (1.1, [0.8, 1.0, 1.0, 1.0]) }
                 else if quadtree.level == 2 { (1.0, [0.7, 0.8, 1.0, 1.0]) }
                 else if quadtree.level == 3 { (0.8, [0.3, 1.0, 0.6, 1.0]) }
                 else if quadtree.level == 4 { (0.7, [0.2, 0.8, 0.8, 1.0]) }
                 else { (0.5, [0.2, 0.4, 0.3, 1.0]) };

    let b = graphics::rectangle::Border { color: colour, radius: radius };
    let mut rect = Rectangle::new([0.0, 0.0, 0.0, 0.0]);
    rect.border = Some( b );
    rect.draw([quadtree.position.x - quadtree.width/2.0, 200.0 - quadtree.position.y - quadtree.height/2.0, quadtree.width, quadtree.height], &c.draw_state, c.transform, g);
}

fn velocity_density(velocities: Vec<f64>, bins: usize) -> Box<Fn(f64) -> f64> {
    let mut density = vec![0.0; bins];
    let mut max = 0.0;

    for v in velocities {
        let i = v as usize / bins;
        density[ std::cmp::min(i, bins-1) ] += 1.0;
        if density[ std::cmp::min(i, bins-1) ] > max {
            max = density[ std::cmp::min(i, bins-1) ];
        }
    }

    let f = move |x: f64| -> f64 {
        let i = x as usize;
        let j = i + 1;
        let h = if i > 0 {i - 1} else { 0 };
        let k = i + 2;
        let a = density[ std::cmp::max(h, 0) ] as f64 / max;
        let b = density[ std::cmp::min(i, bins-1) ] as f64 / max;
        let c = density[ std::cmp::min(j, bins-1) ] as f64 / max;
        let d = density[ std::cmp::min(k, bins-1) ] as f64 / max;

        let v = x - i as f64;

        cubic_interpolate(a, b, c, d, v)
    };

    Box::new(f)
}

fn create_window(width: f64, height: f64) -> (Window, WindowEvents, conrod::Ui, Ids, piston::window::GlyphCache) {
    let mut window: Window = piston::window::WindowSettings::new("boltzmann", [400, 400])
                             .opengl(OpenGL::V3_2)
                             .samples(4)
                             .exit_on_esc(true)
                             .build()
                             .unwrap();

    let events = WindowEvents::new();
    let mut ui = conrod::UiBuilder::new([width as f64, 2.0*height as f64]).build();
    let ids = Ids::new(ui.widget_id_generator());
    let text_texture_cache = piston::window::GlyphCache::new(&mut window, 0, 0);

    (window, events, ui, ids, text_texture_cache)
}

fn main_loop(mut s: Simulator, width: f64, height: f64, window_info: (Window, WindowEvents, conrod::Ui, Ids, piston::window::GlyphCache), draw_tree: bool) {
    let (mut window, mut events, mut ui, ids, mut text_texture_cache) = window_info;
    let image_map = conrod::image::Map::new();

    let mut mouse_position: (f64, f64) = (0.0, 0.0);
    let mut holding_f = false;

    while let Some(event) = window.next_event(&mut events) {

        let f = velocity_density(s.velocities(), 10);

        if let Some(e) = piston::window::convert_event(event.clone(), &window) {
            if let conrod::event::Input::Move( m ) = e {
                if let conrod::event::Motion::MouseCursor(x, y) = m {
                    mouse_position = (x + height, y);
                }
            }

            if let conrod::event::Input::Press( b ) = e {
                if let conrod::input::Button::Mouse(mb) = b {
                    if conrod::input::state::mouse::Button::Left == mb {
                        if holding_f {
                            let (x, y) = mouse_position;
                            for p in &mut s.particles {
                                let position = p.get_position();
                                let d = position.distance(Vector::new(x, y));
                                if d < 100.0 {
                                    let f = 500000.0*(position - Vector::new(x, y) ).normalise() / (d*d);
                                    p.verlet( f );
                                }
                            }
                        }
                        else {
                            let (x, y) = mouse_position;
                            let v_x = (rand::random::<u32>() % 50) as f64 - 25.0;
                            let v_y = (rand::random::<u32>() % 50) as f64 - 25.0;

                            s.insert_particle( Vector::new(x, y) , Vector::new(v_x, v_y));
                        }
                    }
                }
                else if let conrod::input::Button::Keyboard(key) = b {
                    if conrod::input::Key::F == key {
                        holding_f = true
                    }
                }
            }
            if let conrod::event::Input::Release( b ) = e {
                if let conrod::input::Button::Keyboard(key) = b {
                    if conrod::input::Key::F == key {
                        holding_f = false;
                    }
                }
            }
            ui.handle_event(e);
        }

        event.update(|_| {
            use conrod::{color, widget, Colorable, Positionable, Sizeable, Widget};

            let ui = &mut ui.set_widgets();

            widget::Canvas::new().color(color::DARK_CHARCOAL).set(ids.canvas, ui);

            let min_x = 0.0;
            let max_x = 10.0;
            let min_y = 0.0;
            let max_y = 1.0;
            widget::PlotPath::new(min_x, max_x, min_y, max_y, &*f)
                .color( color::Color::Rgba(0.44, 0.67, 0.89, 1.0) )
                .w_h(width, height / 2.0)
                .bottom_left_with_margins_on(ids.canvas, 0.0, 0.0)
                .set(ids.plot, ui);
        });
        window.draw_2d(&event, |c, g| {

            let primitives = ui.draw();
            graphics::clear([0.1, 0.1, 0.1, 1.0], g);

            fn texture_from_image<U>(img: &U) -> &U { img };
            piston::window::draw(c, g, primitives,
                                 &mut text_texture_cache,
                                 &image_map,
                                 texture_from_image);

            if draw_tree { draw_quad_tree(&s.quadtree, &c, g) };

            for p in &s.particles {
                let (red, green, blue) = grey_to_jet(p.get_velocity().magnitude(), 0.0, 100.0);
                graphics::ellipse([red, green, blue, 1.0],
                        [p.get_position().x - s.radius, s.height - p.get_position().y - s.radius, 2.0*s.radius, 2.0*s.radius],
                        c.transform, g);
            }

        });

        s.update();
    }
}


fn main() {
    let width = 400.0;
    let height = 200.0;

    let s = Simulator::new(800, 2.5, 0.0, 1.0, width, height, 0.01);
    let w = create_window(width, height);
    main_loop(s, width, height, w, false);
}
