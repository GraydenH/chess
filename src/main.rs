extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate ndarray;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use graphics::color::GREEN;
use graphics::rectangle::square;
use std::path::Path;
use opengl_graphics::TextureSettings;
use ndarray::{Array2, Axis};
use std::borrow::Borrow;
use piston::{MouseCursorEvent, MouseRelativeEvent, MouseScrollEvent, ButtonState, ButtonEvent, Button, Key, MouseButton, ButtonArgs};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    square: f64,
    grid: Array2<u8>,
    mousex: f64,
    mousey: f64,
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Piece {
    Empty = 0,

    Pawn = 1,
    Rook = 2,
    Knight = 3,
    Bishop = 4,
    Queen = 5,
    King = 6,
}

impl Piece {
    fn from_symbol(c: char) -> Piece {
        return match c.to_ascii_lowercase() {
            'k' => Piece::King,
            'p' => Piece::Pawn,
            'n' => Piece::Knight,
            'b' => Piece::Bishop,
            'r' => Piece::Rook,
            'q' => Piece::Queen,
            _ => Piece::Empty
        }
    }
}

fn load_fen(fen: &str) -> Array2<u8> {
    let mut sections = fen.split(" ");
    let first = sections.next().unwrap_or("");

    let mut result = Array2::zeros((8, 8));

    let mut file = 0usize;
    let mut rank = 7usize;
    for c in first.chars() {
        if c == '/' {
            file = 0;
            rank -= 1;
        } else if c.is_digit(10) {
            file += c.to_digit(10).unwrap_or(0) as usize;
        } else {
            let color = if c.is_uppercase() { 16u8 } else { 0u8 };
            result[[rank, file]] = Piece::from_symbol(c) as u8 | color;
            file += 1;
        }
    }
    result.invert_axis(Axis { 0: 0 });
    result.invert_axis(Axis { 0: 1});
    return result;
}

fn get_square(screenx: f64, screeny: f64, square_size: f64) -> u8 {
    let x = screenx / square_size;
    let y = (square_size * 8.0 - screeny) / square_size;

    let integerx = x as u8;
    let integery = y as u8;

    return integery * 8 + integerx;
}

impl App {
    fn render(&mut self, args: &RenderArgs, pieces_texture: &Texture) {
        use graphics::*;

        const LIGHT: [f32; 4] = [0.84, 0.71, 0.55, 1.0]; // D7B68B
        const DARK: [f32; 4] = [0.16, 0.11, 0.05, 1.0]; // 2A1D0C

        let white_king_image = Image::new().src_rect([0.0, 0.0, 333.0, 333.0]).rect(square(0.0, 0.0, self.square));
        let white_queen_image = Image::new().src_rect([333.0, 0.0, 333.0, 333.0]).rect(square(0.0, 0.0, self.square));
        let white_bishop_image = Image::new().src_rect([2.0 * 333.0, 0.0, 333.0, 333.0]).rect(square(0.0, 0.0, self.square));
        let white_knight_image = Image::new().src_rect([3.0 * 333.0, 0.0, 333.0, 333.0]).rect(square(0.0, 0.0, self.square));
        let white_rook_image = Image::new().src_rect([4.0 * 333.0, 0.0, 333.0, 333.0]).rect(square(0.0, 0.0, self.square));
        let white_pawn_image = Image::new().src_rect([5.0 * 333.0, 0.0, 333.0, 333.0]).rect(square(0.0, 0.0, self.square));

        let black_king_image = Image::new().src_rect([0.0, 333.0, 333.0, 333.0]).rect(square(0.0, 0.0, self.square));
        let black_queen_image = Image::new().src_rect([333.0, 333.0, 333.0, 333.0]).rect(square(0.0, 0.0, self.square));
        let black_bishop_image = Image::new().src_rect([2.0 * 333.0, 333.0, 333.0, 333.0]).rect(square(0.0, 0.0, self.square));
        let black_knight_image = Image::new().src_rect([3.0 * 333.0, 333.0, 333.0, 333.0]).rect(square(0.0, 0.0, self.square));
        let black_rook_image = Image::new().src_rect([4.0 * 333.0, 333.0, 333.0, 333.0]).rect(square(0.0, 0.0, self.square));
        let black_pawn_image = Image::new().src_rect([5.0 * 333.0, 333.0, 333.0, 333.0]).rect(square(0.0, 0.0, self.square));

        let square_length = self.square;
        let square = rectangle::square(0.0, 0.0, self.square);

        let array = &self.grid;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            for i in 0..8 {
                for j in 0..8 {
                    let y = (i as f64) * square_length;
                    let x = (j as f64) * square_length;
                    let transform = c
                        .transform
                        .trans(x, y);
                    if (i + j) % 2 == 0 {
                        rectangle(LIGHT, square, transform, gl);
                    } else {
                        rectangle(DARK, square, transform, gl);
                    }
                    
                    if array[[i, j]] & 16 == 0 {
                        if array[[i, j]] == Piece::Pawn as u8 {
                            black_pawn_image.draw(pieces_texture, &c.draw_state, transform, gl);
                        } else if array[[i, j]] == Piece::Rook as u8 {
                            black_rook_image.draw(pieces_texture, &c.draw_state, transform, gl);
                        } else if array[[i, j]] == Piece::Knight as u8 {
                            black_knight_image.draw(pieces_texture, &c.draw_state, transform, gl);
                        } else if array[[i, j]] == Piece::Bishop as u8 {
                            black_bishop_image.draw(pieces_texture, &c.draw_state, transform, gl);
                        } else if array[[i, j]] == Piece::Queen as u8 {
                            black_queen_image.draw(pieces_texture, &c.draw_state, transform, gl);
                        } else if array[[i, j]] == Piece::King as u8 {
                            black_king_image.draw(pieces_texture, &c.draw_state, transform, gl);
                        }
                    } else {
                        if array[[i, j]] & 15 == Piece::Pawn as u8 {
                            white_pawn_image.draw(pieces_texture, &c.draw_state, transform, gl);
                        } else if array[[i, j]] & 15 == Piece::Rook as u8 {
                            white_rook_image.draw(pieces_texture, &c.draw_state, transform, gl);
                        } else if array[[i, j]] & 15 == Piece::Knight as u8 {
                            white_knight_image.draw(pieces_texture, &c.draw_state, transform, gl);
                        } else if array[[i, j]] & 15 == Piece::Bishop as u8 {
                            white_bishop_image.draw(pieces_texture, &c.draw_state, transform, gl);
                        } else if array[[i, j]] & 15 == Piece::Queen as u8 {
                            white_queen_image.draw(pieces_texture, &c.draw_state, transform, gl);
                        } else if array[[i, j]] & 15 == Piece::King as u8 {
                            white_king_image.draw(pieces_texture, &c.draw_state, transform, gl);
                        }
                    }
                }
            }
        });
    }

    fn click(&mut self, args: &ButtonArgs) {
        let index = get_square(self.mousex, self.mousey, self.square);
        if args.state == ButtonState::Press {
            
        } else {

        }
    }

    fn set_mousex(&mut self, x: f64) {
        self.mousex = x;
    }

    fn set_mousey(&mut self, y: f64) {
        self.mousey = y;
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

    let grid = load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        square: 64.0,
        grid,
        mousex: 0.0,
        mousey: 0.0,
    };

    let pieces_texture = Texture::from_path(Path::new("assets/textures/pieces.png"), &TextureSettings::new()).unwrap();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &pieces_texture);
        }
        if let Some(args) = e.mouse_cursor_args() {
            let [x, y] = args;
            app.set_mousex(x);
            app.set_mousey(y);
        }
        if let Some(args) = e.button_args() {
            match args.button {
                Button::Mouse(MouseButton::Left) => app.click(&args),
                _ => (),
            }
        }
    }
}