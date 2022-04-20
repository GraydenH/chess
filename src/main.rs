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
use ndarray::Array2;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Piece {
    BlackPawn = 1,
    BlackRook = 2,
    BlackKnight = 3,
    BlackBishop = 4,
    BlackQueen = 5,
    BlackKing = 6,

    WhitePawn = 7,
    WhiteRook = 8,
    WhiteKnight = 9,
    WhiteBishop = 10,
    WhiteQueen = 11,
    WhiteKing = 12,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const LIGHT: [f32; 4] = [0.84, 0.71, 0.55, 1.0]; // D7B68B
        const DARK: [f32; 4] = [0.16, 0.11, 0.05, 1.0]; // 2A1D0C

        let white_king_image= Image::new().src_rect([0.0, 0.0, 333.0, 333.0]).rect(square(0.0, 0.0, 128.0));
        let white_queen_image= Image::new().src_rect([333.0, 0.0, 333.0, 333.0]).rect(square(0.0, 0.0, 128.0));
        let white_bishop_image= Image::new().src_rect([2.0 * 333.0, 0.0, 333.0, 333.0]).rect(square(0.0, 0.0, 128.0));
        let white_knight_image= Image::new().src_rect([3.0 * 333.0, 0.0, 333.0, 333.0]).rect(square(0.0, 0.0, 128.0));
        let white_rook_image= Image::new().src_rect([4.0 * 333.0, 0.0, 333.0, 333.0]).rect(square(0.0, 0.0, 128.0));
        let white_pawn_image= Image::new().src_rect([5.0 * 333.0, 0.0, 333.0, 333.0]).rect(square(0.0, 0.0, 128.0));

        let black_king_image= Image::new().src_rect([0.0, 333.0, 333.0, 333.0]).rect(square(0.0, 0.0, 128.0));
        let black_queen_image= Image::new().src_rect([333.0, 333.0, 333.0, 333.0]).rect(square(0.0, 0.0, 128.0));
        let black_bishop_image= Image::new().src_rect([2.0 * 333.0, 333.0, 333.0, 333.0]).rect(square(0.0, 0.0, 128.0));
        let black_knight_image= Image::new().src_rect([3.0 * 333.0, 333.0, 333.0, 333.0]).rect(square(0.0, 0.0, 128.0));
        let black_rook_image= Image::new().src_rect([4.0 * 333.0, 333.0, 333.0, 333.0]).rect(square(0.0, 0.0, 128.0));
        let black_pawn_image= Image::new().src_rect([5.0 * 333.0, 333.0, 333.0, 333.0]).rect(square(0.0, 0.0, 128.0));

        let pieces_texture = Texture::from_path(Path::new("assets/textures/pieces.png"), &TextureSettings::new()).unwrap();

        let square = rectangle::square(0.0, 0.0, 128.0);

        let mut array = Array2::zeros((8, 8));
        array[[1, 0]] = Piece::BlackPawn as u8;
        array[[1, 1]] = Piece::BlackPawn as u8;
        array[[1, 2]] = Piece::BlackPawn as u8;
        array[[1, 3]] = Piece::BlackPawn as u8;
        array[[1, 4]] = Piece::BlackPawn as u8;
        array[[1, 5]] = Piece::BlackPawn as u8;
        array[[1, 6]] = Piece::BlackPawn as u8;
        array[[1, 7]] = Piece::BlackPawn as u8;
        array[[0, 0]] = Piece::BlackRook as u8;
        array[[0, 1]] = Piece::BlackKnight as u8;
        array[[0, 2]] = Piece::BlackBishop as u8;
        array[[0, 3]] = Piece::BlackQueen as u8;
        array[[0, 4]] = Piece::BlackKing as u8;
        array[[0, 5]] = Piece::BlackBishop as u8;
        array[[0, 6]] = Piece::BlackKnight as u8;
        array[[0, 7]] = Piece::BlackRook as u8;

        array[[6, 0]] = Piece::WhitePawn as u8;
        array[[6, 1]] = Piece::WhitePawn as u8;
        array[[6, 2]] = Piece::WhitePawn as u8;
        array[[6, 3]] = Piece::WhitePawn as u8;
        array[[6, 4]] = Piece::WhitePawn as u8;
        array[[6, 5]] = Piece::WhitePawn as u8;
        array[[6, 6]] = Piece::WhitePawn as u8;
        array[[6, 7]] = Piece::WhitePawn as u8;
        array[[7, 0]] = Piece::WhiteRook as u8;
        array[[7, 1]] = Piece::WhiteKnight as u8;
        array[[7, 2]] = Piece::WhiteBishop as u8;
        array[[7, 3]] = Piece::WhiteQueen as u8;
        array[[7, 4]] = Piece::WhiteKing as u8;
        array[[7, 5]] = Piece::WhiteBishop as u8;
        array[[7, 6]] = Piece::WhiteKnight as u8;
        array[[7, 7]] = Piece::WhiteRook as u8;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            for i in 0..8 {
                for j in 0..8 {
                    let y = (i as f64) * 128.0;
                    let x = (j as f64) * 128.0;
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
                    if array[[i, j]] == Piece::BlackPawn as u8 {
                        black_pawn_image.draw(&pieces_texture, &c.draw_state, transform_piece, gl);
                    } else if array[[i, j]] == Piece::BlackRook as u8 {
                        black_rook_image.draw(&pieces_texture, &c.draw_state, transform_piece, gl);
                    } else if array[[i, j]] == Piece::BlackKnight as u8 {
                        black_knight_image.draw(&pieces_texture, &c.draw_state, transform_piece, gl);
                    } else if array[[i, j]] == Piece::BlackBishop as u8 {
                        black_bishop_image.draw(&pieces_texture, &c.draw_state, transform_piece, gl);
                    } else if array[[i, j]] == Piece::BlackQueen as u8 {
                        black_queen_image.draw(&pieces_texture, &c.draw_state, transform_piece, gl);
                    } else if array[[i, j]] == Piece::BlackKing as u8 {
                        black_king_image.draw(&pieces_texture, &c.draw_state, transform_piece, gl);
                    } else if array[[i, j]] == Piece::WhitePawn as u8 {
                        white_pawn_image.draw(&pieces_texture, &c.draw_state, transform_piece, gl);
                    } else if array[[i, j]] == Piece::WhiteRook as u8 {
                        white_rook_image.draw(&pieces_texture, &c.draw_state, transform_piece, gl);
                    } else if array[[i, j]] == Piece::WhiteKnight as u8 {
                        white_knight_image.draw(&pieces_texture, &c.draw_state, transform_piece, gl);
                    } else if array[[i, j]] == Piece::WhiteBishop as u8 {
                        white_bishop_image.draw(&pieces_texture, &c.draw_state, transform_piece, gl);
                    } else if array[[i, j]] == Piece::WhiteQueen as u8 {
                        white_queen_image.draw(&pieces_texture, &c.draw_state, transform_piece, gl);
                    } else if array[[i, j]] == Piece::WhiteKing as u8 {
                        white_king_image.draw(&pieces_texture, &c.draw_state, transform_piece, gl);
                    }
                }
            }
        });
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("chess", [1024, 1024])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
    }
}