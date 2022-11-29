use macroquad::prelude::*;

use crate::constants::*;
use crate::snake_config::SnakeConfig;

pub struct PlayerState {
    pub snake_config: SnakeConfig,
    pub player_pos: [(i32, i32); 10],
    pub player_direction: Direction,
    pub direction_switch_since_move: bool,
    pub player_last_move_time: f64,
    pub collected_letters: usize,
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
        for i in (0..self.player_pos.len() - 1).rev() {
            self.player_pos[i + 1] = self.player_pos[i];
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
        if self.collected_letters < 9 {
            self.collected_letters += 1;
        } else {
            self.collected_letters = 0;
        }
    }

    pub fn player_x_pos(&self) -> i32 {
        self.player_pos[0].0
    }

    pub fn set_player_x_pos(&mut self, pos: i32) {
        self.player_pos[0].0 = pos;
    }

    pub fn player_y_pos(&self) -> i32 {
        self.player_pos[0].1
    }

    pub fn set_player_y_pos(&mut self, pos: i32) {
        self.player_pos[0].1 = pos;
    }

    pub fn draw(&self, font_size: f32, color: Color) {
        // draw every collected letter at their current position.
        for i in 0..=self.collected_letters {
            let abs_letter_x_pos = self.player_pos[i].0 as f32 * font_size;
            let abs_letter_y_pos = self.player_pos[i].1 as f32 * font_size;

            draw_text(
                ROFLCOPTER[i],
                abs_letter_x_pos,
                abs_letter_y_pos,
                font_size,
                color,
            );
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
        PlayerState {
            snake_config,
            player_pos: [
                // Start the snake `expanded` even though the characters aren't visible yet.
                // In the future, the should all be on the head position and only expand once
                // they are actually collected and visible.
                (PLAYER_START_X_POS, PLAYER_START_Y_POS),
                (PLAYER_START_X_POS, PLAYER_START_Y_POS + 1),
                (PLAYER_START_X_POS, PLAYER_START_Y_POS + 2),
                (PLAYER_START_X_POS, PLAYER_START_Y_POS + 3),
                (PLAYER_START_X_POS, PLAYER_START_Y_POS + 4),
                (PLAYER_START_X_POS, PLAYER_START_Y_POS + 5),
                (PLAYER_START_X_POS, PLAYER_START_Y_POS + 6),
                (PLAYER_START_X_POS, PLAYER_START_Y_POS + 7),
                (PLAYER_START_X_POS, PLAYER_START_Y_POS + 8),
                (PLAYER_START_X_POS, PLAYER_START_Y_POS + 9),
            ],
            player_direction: Direction::East,
            direction_switch_since_move: false,
            player_last_move_time: get_time(),
            collected_letters: 0,
        }
    }
}
