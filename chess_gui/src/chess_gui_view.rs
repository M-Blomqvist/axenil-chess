use std::collections::HashMap;

use graphics::{types::Color, Context, Graphics, Image};
use opengl_graphics::Texture;

use super::chess_controller::ChessController;
pub struct ChessView {
    pub settings: ViewSettings,
}

impl ChessView {
    pub fn new(settings: ViewSettings) -> ChessView {
        ChessView { settings }
    }

    pub fn draw<G: Graphics<Texture = Texture>>(
        &self,
        controller: &ChessController,
        context: &Context,
        g: &mut G,
    ) {
        use graphics::{rectangle, Rectangle};

        let settings = &self.settings;
        let board_area = [
            settings.position[0],
            settings.position[1],
            settings.size,
            settings.size,
        ];

        Rectangle::new(settings.background_color).draw(
            board_area,
            &context.draw_state,
            context.transform,
            g,
        );

        let black_space = Rectangle::new(settings.black_color);
        let white_space = Rectangle::new(settings.white_color);
        let space_size = (settings.size - settings.grid_width) / 8.0 - settings.grid_width;
        let mut draw_black = false;

        for y in 0..8 {
            for x in 0..8 {
                let space = rectangle::square(
                    x as f64 * (space_size + settings.grid_width)
                        + settings.position[0]
                        + settings.grid_width,
                    y as f64 * (space_size + settings.grid_width)
                        + settings.position[0]
                        + settings.grid_width,
                    space_size,
                );
                if draw_black {
                    black_space.draw(space, &context.draw_state, context.transform, g);
                } else {
                    white_space.draw(space, &context.draw_state, context.transform, g);
                }
                draw_black = !draw_black;
            }
            draw_black = !draw_black;
        }
        if let Some(space_coord) = controller.selected_space {
            let selected_space = rectangle::square(
                space_coord.0 as f64 * (space_size + settings.grid_width)
                    + settings.position[0]
                    + settings.grid_width,
                (7 - space_coord.1) as f64 * (space_size + settings.grid_width)
                    + settings.position[0]
                    + settings.grid_width,
                space_size,
            );

            Rectangle::new(settings.selection_color).draw(
                selected_space,
                &context.draw_state,
                context.transform,
                g,
            );
        }
        if let Some(highligted_space) = &controller.highlighted_spaces {
            for space in highligted_space {
                let space_area = rectangle::square(
                    space.0 as f64 * (space_size + settings.grid_width)
                        + settings.position[0]
                        + settings.grid_width,
                    space.1 as f64 * (space_size + settings.grid_width)
                        + settings.position[0]
                        + settings.grid_width,
                    space_size,
                );

                Rectangle::new(settings.selection_color).draw(
                    space_area,
                    &context.draw_state,
                    context.transform,
                    g,
                );
            }
        }
        for y in 0..8 {
            for x in 0..8 {
                let piece_str = &controller.board_string[y][x];
                if let Some(texture) = self.settings.piece_imgs.get(piece_str) {
                    let img = Image::new().rect(rectangle::square(
                        x as f64 * (space_size + settings.grid_width)
                            + settings.position[0]
                            + settings.grid_width,
                        y as f64 * (space_size + settings.grid_width)
                            + settings.position[0]
                            + settings.grid_width,
                        space_size,
                    ));

                    img.draw(texture, &context.draw_state, context.transform, g);
                }
            }
        }
    }
}

pub struct ViewSettings {
    pub piece_imgs: HashMap<String, Texture>,
    pub position: [f64; 2],
    pub size: f64,
    pub background_color: Color,
    pub white_color: Color,
    pub black_color: Color,
    pub selection_color: Color,
    pub highlight_color: Color,
    pub grid_width: f64,
}

impl ViewSettings {
    pub fn default_view(size: f64, piece_imgs: HashMap<String, Texture>) -> ViewSettings {
        ViewSettings {
            piece_imgs,
            position: [0.0; 2],
            size,
            background_color: [0.313, 0.2, 0.007, 1.0],
            white_color: [0.882, 0.878, 0.858, 1.0],
            black_color: [0.415, 0.364, 0.223, 1.0],
            selection_color: [0.133, 0.725, 0.694, 0.5],
            highlight_color: [0.8, 0.909, 0.552, 0.8],
            grid_width: size / 100.0,
        }
    }
}
