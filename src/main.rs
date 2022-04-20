extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use graphics::color::{WHITE, BLACK, GREEN};
use graphics::rectangle::square;
use std::path::Path;
use opengl_graphics::TextureSettings;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const LIGHT: [f32; 4] = [0.84, 0.71, 0.55, 1.0]; // D7B68B
        const DARK: [f32; 4] = [0.16, 0.11, 0.05, 1.0]; // 2A1D0C

        // Create the image object and attach a square Rectangle object inside.
        let image= Image::new().rect(square(0.0, 0.0, 64.0));
        // A texture to use with the image
        let texture = Texture::from_path(Path::new("black_pawn.png"), &TextureSettings::new()).unwrap();

        let square = rectangle::square(0.0, 0.0, 64.0);
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            for i in 0..8 {
                for j in 0..8 {
                    let y = (i as f64) * 64.0;
                    let x = (j as f64) * 64.0;
                    let transform = c
                        .transform
                        .trans(x, y);
                    if (i + j) % 2 == 0 {
                        rectangle(LIGHT, square, transform, gl);
                    } else {
                        rectangle(DARK, square, transform, gl);
                    }

                    let transform_piece = c
                        .transform
                        .trans(x, y);
                    image.draw(&texture, &c.draw_state, transform_piece, gl);
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("chess", [512, 512])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}