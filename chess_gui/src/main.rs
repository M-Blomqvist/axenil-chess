extern crate glutin_window;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::{
    event_loop::{EventLoop, EventSettings, Events},
    input::RenderEvent,
    window::WindowSettings,
};
use std::{
    collections::HashMap,
    env,
    path::Path,
    sync::mpsc::{Receiver, Sender},
};

use crate::chess_controller::ChessController;
use crate::chess_gui_view::{ChessView, ViewSettings};
use multiplayer::start_multiplayer;
use rust_chess::{units::Color, *};
mod chess_controller;
mod chess_gui_view;
fn main() {
    let opengl = OpenGL::V3_2;

    let settings = WindowSettings::new("chessGUI", [600; 2]).exit_on_esc(true); //Only supports square resolutions
    let mut window: GlutinWindow = settings.build().expect("Error building Glutin_window!");

    let mut events = Events::new(EventSettings::new().lazy(false));
    let mut gl = GlGraphics::new(opengl);

    let mut chess_board = init_chess();
    chess_board.fill_board("./rust_chess/data/board.txt");
    chess_board.print_board();

    let imgs = load_imgs();

    let args: Vec<String> = env::args().collect();
    println!("Running {}...", args[0]);
    let mut online_connection: Option<(Sender<[u8; 5]>, Receiver<[u8; 5]>)> = None;
    let mut player_color = None;
    if args.contains(&"host".to_string()) {
        if let Ok((connection, handle)) = start_multiplayer(&args[1], &args[2]) {
            online_connection = Some(connection);
            player_color = Some(Color::White);
        }
    }
    if args.contains(&"connect".to_string()) {
        if let Ok((connection, handle)) = start_multiplayer(&args[1], &args[2]) {
            online_connection = Some(connection);
            player_color = Some(Color::Black);
        }
    }

    let mut chess_controller = ChessController::new(chess_board, online_connection, player_color);
    let chess_view = ChessView::new(ViewSettings::default_view(settings.get_size().width, imgs));

    while let Some(event) = events.next(&mut window) {
        chess_controller.event(
            chess_view.settings.position,
            chess_view.settings.size,
            chess_view.settings.grid_width,
            &event,
        );
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |context, g| {
                use graphics::clear;

                clear([0.0; 4], g);
                chess_view.draw(&chess_controller, &context, g);
            });
        }
    }
}

fn load_imgs() -> HashMap<String, Texture> {
    let mut imgs: HashMap<String, Texture> = HashMap::new();
    let path = "./chess_gui/Chess pieces/";
    let piecetypes = ["pawn", "king", "rook", "knight", "bishop", "queen"];
    let color = "white_";
    for piece in piecetypes.iter() {
        let image = graphics::Image::new().rect(graphics::rectangle::square(0.0, 0.0, 200.0));
        let p: String = path.to_string() + color + piece + ".png";
        let texture = Texture::from_path(Path::new(&p), &TextureSettings::new())
            .expect("error loading image");
        imgs.insert(color.to_string() + piece, texture);
    }
    let color = "black_";
    for piece in piecetypes.iter() {
        let image = graphics::Image::new().rect(graphics::rectangle::square(0.0, 0.0, 200.0));
        let mut p: String = path.to_string() + color + piece + ".png";
        let texture = Texture::from_path(Path::new(&p), &TextureSettings::new())
            .expect("error loading image");
        imgs.insert(color.to_string() + piece, texture);
    }

    imgs
}

fn init_chess() -> rust_chess::board::Board {
    let mut chess_board = board::Board::init();
    chess_board.fill_board("rust_chess/data/board.txt");
    chess_board
}
