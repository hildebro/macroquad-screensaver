use macroquad::prelude::*;

use crate::constants::*;
use crate::snake_config::SnakeConfig;

pub struct PlayerState {
    pub snake_config: SnakeConfig,
    // x and y position of every part of the snake.
    pub player_parts: Vec<(i32, i32)>,
    pub player_direction: Direction,
    pub direction_switch_since_move: bool,
    pub player_last_move_time: f64,
}

impl PlayerState {
    pub fn move_player(&mut self) {
        let player_moved = self.attempt_move();
        if !player_moved {
            // No need to check for out-of-bounds movement, when no move happened.
            return;
        }

        // Jump to the other side, if the player hits the edge.
        if self.player_x_pos() >= self.snake_config.horizontal_slots {
            self.set_player_x_pos(0);
        }
        if self.player_x_pos() < 0 {
            self.set_player_x_pos(self.snake_config.horizontal_slots - 1);
        }
        if self.player_y_pos() >= self.snake_config.vertical_slots {
            self.set_player_y_pos(0);
        }
        if self.player_y_pos() < 0 {
            self.set_player_y_pos(self.snake_config.vertical_slots - 1);
        }
    }

    pub fn attempt_move(&mut self) -> bool {
        let loop_time = get_time();
        if loop_time - self.player_last_move_time <= PLAYER_MOVE_INTERVAL {
            // Don't move unless a bit of time has passed since the last move.
            return false;
        }

        // Reset the compare time.
        self.player_last_move_time = loop_time;
        // Reset direction switcher.
        self.direction_switch_since_move = false;

        // Move all body parts one step closer to the head.
        for i in (0..self.player_parts.len() - 1).rev() {
            self.player_parts[i + 1] = self.player_parts[i];
        }

        // Move the head to a new position.
        match self.player_direction {
            Direction::West => self.set_player_x_pos(self.player_x_pos() - 1),
            Direction::East => self.set_player_x_pos(self.player_x_pos() + 1),
            Direction::North => self.set_player_y_pos(self.player_y_pos() - 1),
            Direction::South => self.set_player_y_pos(self.player_y_pos() + 1),
        }

        return true;
    }

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

    pub fn update(&mut self, new_direction: Direction) {
        if !self.direction_switch_since_move {
            self.player_direction = new_direction;
            self.direction_switch_since_move = true;
        }

        self.move_player();
    }

    pub fn new(snake_config: SnakeConfig) -> PlayerState {
        let mut player_parts = Vec::new();
        player_parts.push((PLAYER_START_X_POS, PLAYER_START_Y_POS));

        PlayerState {
            snake_config,
            player_parts,
            player_direction: Direction::East,
            direction_switch_since_move: false,
            player_last_move_time: get_time(),
        }
    }
}
