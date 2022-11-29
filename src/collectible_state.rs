use macroquad::prelude::*;

use crate::constants::ROFLCOPTER;
use crate::snake_config::SnakeConfig;

pub struct CollectibleState {
    pub snake_config: SnakeConfig,
    pub x_position: i32,
    pub y_position: i32,
    pub index: usize,
}

impl CollectibleState {
    pub fn new(snake_config: SnakeConfig) -> CollectibleState {
        CollectibleState {
            snake_config,
            x_position: rand::gen_range(0, snake_config.horizontal_slots),
            y_position: rand::gen_range(0, snake_config.vertical_slots),
            // Starts at 1 instead of 0, because the player starts with one collectible already.
            index: 1,
        }
    }

    pub fn set_new_character(&mut self) {
        // New location.
        self.x_position = rand::gen_range(0, self.snake_config.horizontal_slots);
        self.y_position = rand::gen_range(0, self.snake_config.vertical_slots);

        // New character to render.
        if self.index < 9 {
            self.index += 1;
        } else {
            self.index = 0;
        }
    }

    pub fn draw(&self, font_size: f32, color: Color) {
        // get absolute positions for the collectible.
        let abs_x_position = self.x_position as f32 * font_size;
        let abs_y_position = self.y_position as f32 * font_size;

        draw_text(
            self.char_to_render(),
            abs_x_position,
            abs_y_position,
            font_size,
            color,
        );
    }

    pub fn char_to_render(&self) -> &str {
        ROFLCOPTER[self.index]
    }
}
