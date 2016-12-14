#[macro_use] extern crate conrod;
extern crate boltzmann;
extern crate graphics;
extern crate rand;

use boltzmann::simulator::Simulator;
use boltzmann::quadtree::Quadtree;

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
    let b = graphics::rectangle::Border { color: [1.0, 1.0, 1.0, 1.0], radius: 1.0 };
    let mut rect = Rectangle::new([0.0, 0.0, 0.0, 0.0]);
    rect.border = Some( b );
    rect.draw([quadtree.position.x - quadtree.width/2.0, quadtree.position.y - quadtree.height/2.0, quadtree.width, quadtree.height], &c.draw_state, c.transform, g);

    let colour: f32 = (rand::random::<u32>()) as f32 / std::u32::MAX as f32;

    for &(_, p) in &quadtree.objects {
        // let (red, green, blue) = grey_to_jet(p.get_velocity().magnitude(), 0.0, 100.0);
           graphics::ellipse([colour, 1.0, colour, 1.0],
                   [p.x - quadtree.radius, 200.0 - p.y - quadtree.radius, 2.0*quadtree.radius, 2.0*quadtree.radius],
                   c.transform, g);
    }

    if let Some((ref c1, ref c2, ref c3, ref c4)) = quadtree.children {
        draw_quad_tree(c1, c, g);
        // draw_quad_tree(c2, c, g);
        // draw_quad_tree(c3, c, g);
        // draw_quad_tree(c4, c, g);
    }
}

fn main() {
    let mut s = Simulator::new(60, 5.0, 0.0, 400.0, 200.0, 0.01);

    let mut t = 0;

    let mut window: Window =
       piston::window::WindowSettings::new("boltzmann", [400, 400])
           .opengl(OpenGL::V3_2)
           .samples(4)
           .exit_on_esc(true)
           .build()
           .unwrap();

    let mut events = WindowEvents::new();
    let mut ui = conrod::UiBuilder::new([400.0 as f64, 400.0 as f64]).build();

    // A unique identifier for each widget.
    let ids = Ids::new(ui.widget_id_generator());

    // No text to draw, so we'll just create an empty text texture cache.
    let mut text_texture_cache = piston::window::GlyphCache::new(&mut window, 0, 0);

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::new();

    // Poll events from the window.
    while let Some(event) = window.next_event(&mut events) {

        let ps = s.particles.clone();
        let vs: Vec<f64> = s.velociies();
        let mut density = vec![0; 10];

        for v in vs {
            let i = v as usize / 10;

            density[ std::cmp::min(i, 9) ] += 1;
        }

        let r = s.radius;
        let h = s.height as f64;

        let f = |x: f64| -> f64 {
            let i = x as usize;
            let j = i + 1;
            let h = if i > 0 {i - 1} else { 0 };
            let k = i + 2;
            let a = density[ std::cmp::max(h, 0) ] as f64;
            let b = density[ std::cmp::min(i, 9) ] as f64;
            let c = density[ std::cmp::min(j, 9) ] as f64;
            let d = density[ std::cmp::min(k, 9) ] as f64;

            let v = x - i as f64;

            cubic_interpolate(a, b, c, d, v)
        };

        // Convert the piston event to a conrod event.
        if let Some(e) = piston::window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }

        event.update(|_| {
            use conrod::{color, widget, Colorable, Positionable, Sizeable, Widget};

            let ui = &mut ui.set_widgets();

            widget::Canvas::new().color(color::DARK_CHARCOAL).set(ids.canvas, ui);

            let min_x = 0.0;
            let max_x = density.len() as f64;
            let min_y = 0.0;
            let max_y = 5.0;
            widget::PlotPath::new(min_x, max_x, min_y, max_y, &f)
                .color(color::LIGHT_BLUE)
                .w_h(400.0, 50.0)
                .bottom_left_with_margins_on(ids.canvas, 0.0, 0.0)
                .set(ids.plot, ui);
        });
        if t == 0 {
        window.draw_2d(&event, |c, g| {

            let primitives = ui.draw();
            //if let Some(primitives) = ui.draw_if_changed() {
            graphics::clear([0.1, 0.1, 0.1, 1.0], g);

            fn texture_from_image<T>(img: &T) -> &T { img };
            piston::window::draw(c, g, primitives,
                                 &mut text_texture_cache,
                                 &image_map,
                                 texture_from_image);

            draw_quad_tree(&s.quadtree, &c, g);

            // for p in ps {
            //     let (red, green, blue) = grey_to_jet(p.get_velocity().magnitude(), 0.0, 100.0);
            //     graphics::ellipse([red, green, blue, 1.0],
            //             [p.get_position().x - r, h - p.get_position().y - s.radius, 2.0*r, 2.0*r],
            //             c.transform, g);
            //
            //
            //
            //     // let mut rect = graphics::Rectangle::new([0.0, 0.0, 0.0, 0.0]);
            //     // rect.border = Some(b);
            //     // rect.draw( [p.get_position().x - r, h - p.get_position().y - s.radius, 2.0*r, 2.0*r],  )
            //     // let square = graphics::rectangle::Border::
            //     // let red = [1.0, 0.0, 0.0, 1.0];
            //     // rectangle(red, square, view.trans(self.x, self.y).trans(-50.0, -50.0), g);
            // }
        });
        }
        if t == 0 {
            s.update();
        }
        t += 1;

    }
}
