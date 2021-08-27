use crate::constants::*;
use macroquad::prelude::*;

pub struct PlayerState {
    pub player_pos: [(f32, f32); 10],
    pub player_direction: Direction,
    pub player_last_move_time: f64,
    pub collected_letters: usize,
    pub color: Color,
}

impl PlayerState {
    pub fn attempt_move(&mut self) -> bool {
        let loop_time = macroquad::time::get_time();
        if loop_time - self.player_last_move_time <= PLAYER_MOVE_INTERVAL {
            // Don't move unless a bit of time has passed since the last move.
            return false;
        }

        // Reset the compare time.
        self.player_last_move_time = loop_time;

        // Move all body parts one step closer to the head.
        for i in (0..self.player_pos.len() - 1).rev() {
            self.player_pos[i + 1] = self.player_pos[i];
        }

        // Move the head to a new position.
        match self.player_direction {
            Direction::West => self.set_player_x_pos(self.player_x_pos() - FONT_SIZE / 2.0),
            Direction::East => self.set_player_x_pos(self.player_x_pos() + FONT_SIZE / 2.0),
            Direction::North => self.set_player_y_pos(self.player_y_pos() - FONT_SIZE / 2.0),
            Direction::South => self.set_player_y_pos(self.player_y_pos() + FONT_SIZE / 2.0),
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

    pub fn player_x_pos(&self) -> f32 {
        self.player_pos[0].0
    }

    pub fn set_player_x_pos(&mut self, pos: f32) {
        self.player_pos[0].0 = pos;
    }

    pub fn player_y_pos(&self) -> f32 {
        self.player_pos[0].1
    }

    pub fn set_player_y_pos(&mut self, pos: f32) {
        self.player_pos[0].1 = pos;
    }

    pub fn draw(&self) {
        for i in 0..=self.collected_letters {
            draw_text(
                ROFLCOPTER[i],
                self.player_pos[i].0,
                self.player_pos[i].1,
                FONT_SIZE,
                self.color,
            );
        }
    }

    pub fn new() -> PlayerState {
        let r = rand::gen_range(0.5, 1.0);
        let g = rand::gen_range(0.5, 1.0);
        let b = rand::gen_range(0.5, 1.0);

        PlayerState {
            player_pos: [
                // Start the snake `expanded` even though the characters aren't visible yet.
                // In the future, the should all be on the head position and only expand once
                // they are actually collected and visible.
                (PLAYER_START_X_POS, PLAYER_START_Y_POS),
                (PLAYER_START_X_POS, PLAYER_START_Y_POS + FONT_SIZE / 2.0),
                (PLAYER_START_X_POS, PLAYER_START_Y_POS + FONT_SIZE),
                (PLAYER_START_X_POS, PLAYER_START_Y_POS + FONT_SIZE * 1.5),
                (PLAYER_START_X_POS, PLAYER_START_Y_POS + FONT_SIZE * 2.0),
                (PLAYER_START_X_POS, PLAYER_START_Y_POS + FONT_SIZE * 2.5),
                (PLAYER_START_X_POS, PLAYER_START_Y_POS + FONT_SIZE * 3.0),
                (PLAYER_START_X_POS, PLAYER_START_Y_POS + FONT_SIZE * 3.5),
                (PLAYER_START_X_POS, PLAYER_START_Y_POS + FONT_SIZE * 4.0),
                (PLAYER_START_X_POS, PLAYER_START_Y_POS + FONT_SIZE * 4.5),
            ],
            player_direction: Direction::East,
            player_last_move_time: macroquad::time::get_time(),
            collected_letters: 0,
            color: Color::new(r, g, b, 1.0),
        }
    }
}
