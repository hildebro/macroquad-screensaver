use macroquad::prelude::*;

use crate::constants::*;
use crate::enums::{Direction, StartingPosition};
use crate::player::move_player;
use crate::snake_config::CONFIG;

pub struct PlayerState {
    // x and y position of every part of the snake.
    pub player_parts: Vec<(i32, i32)>,
    pub last_move_direction: Direction,
    pub last_move_time: f64,
}

impl PlayerState {
    pub fn register_collision(&mut self) {
        if self.player_parts.len() >= ROFLCOPTER.len() {
            // If the player already collected all parts of ROFLCOPTER, the player is reverted back
            // to a single part by creating a new Vec that contains only the head of the snake.
            self.player_parts = self.player_parts.drain(..1).collect();
        } else {
            // As long as the max length hasn't been reached, we just add a new part.
            let last_part = self.player_parts.last().unwrap();
            self.player_parts.push((last_part.0, last_part.1));
        }
    }

    pub fn player_x_pos(&self) -> i32 {
        self.player_parts[0].0
    }

    pub fn set_player_x_pos(&mut self, pos: i32) {
        self.player_parts[0].0 = pos;
    }

    pub fn player_y_pos(&self) -> i32 {
        self.player_parts[0].1
    }

    pub fn set_player_y_pos(&mut self, pos: i32) {
        self.player_parts[0].1 = pos;
    }

    pub fn draw(&self, font_size: f32, color: Color) {
        // Draw every player part. In order to know, which letter of ROFLCOPTER to draw per part,
        // an index variable is used that increments after each draw.
        let mut index = 0;
        for player_part in self.player_parts.iter() {
            let abs_letter_x_pos = player_part.0 as f32 * font_size;
            let abs_letter_y_pos = player_part.1 as f32 * font_size;

            draw_text(
                ROFLCOPTER[index],
                abs_letter_x_pos,
                abs_letter_y_pos,
                font_size,
                color,
            );

            index += 1;
        }
    }

    pub fn update_location(&mut self, direction: Direction) {
        // Move the player
        move_player(self, direction);
        // Set last move direction
        self.last_move_direction = direction;
        // Reset the compare time.
        self.last_move_time = get_time();
    }

    pub fn new() -> PlayerState {
        let starting_position = match CONFIG.starting_position {
            StartingPosition::Corner => (0, 0),
            StartingPosition::Center => (CONFIG.horizontal_slots / 2, CONFIG.vertical_slots / 2),
            StartingPosition::Random => (
                rand::gen_range(0, CONFIG.horizontal_slots),
                rand::gen_range(0, CONFIG.vertical_slots),
            ),
        };
        let player_parts = vec![starting_position];

        PlayerState {
            player_parts,
            // Setting an arbitrary direction to begin with.
            last_move_direction: Direction::East,
            last_move_time: get_time(),
        }
    }
}
