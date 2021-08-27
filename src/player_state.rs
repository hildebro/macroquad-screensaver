use crate::constants::*;
use macroquad::prelude::*;

pub struct PlayerState {
    pub width: f32,
    pub height: f32,
    pub player_pos: [(f32, f32); 10],
    pub player_direction: Direction,
    pub player_last_move_time: f64,
    pub player_size: usize,
}

impl PlayerState {
    pub fn update(&mut self) {
        // Switch direction on key input.
        if macroquad::input::is_key_down(KeyCode::W) {
            self.set_direction(Direction::NORTH);
        } else if macroquad::input::is_key_down(KeyCode::D) {
            self.set_direction(Direction::EAST);
        } else if macroquad::input::is_key_down(KeyCode::S) {
            self.set_direction(Direction::SOUTH);
        } else if macroquad::input::is_key_down(KeyCode::A) {
            self.set_direction(Direction::WEST);
        }

        self.move_player();
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.player_direction = direction;
    }

    pub fn move_player(&mut self) {
        let loop_time = macroquad::time::get_time();
        if loop_time - self.player_last_move_time <= PLAYER_MOVE_INTERVAL {
            // Don't move unless a bit of time has passed since the last move.
            return;
        }

        // Reset the compare time.
        self.player_last_move_time = loop_time;

        // Move all body parts one step closer to the head.
        for i in (0..self.player_pos.len() - 1).rev() {
            self.player_pos[i + 1] = self.player_pos[i];
        }

        // Move the head to a new position.
        match self.player_direction {
            Direction::WEST => self.set_player_x_pos(self.player_x_pos() - FONT_SIZE / 2.0),
            Direction::EAST => self.set_player_x_pos(self.player_x_pos() + FONT_SIZE / 2.0),
            Direction::NORTH => self.set_player_y_pos(self.player_y_pos() - FONT_SIZE / 2.0),
            Direction::SOUTH => self.set_player_y_pos(self.player_y_pos() + FONT_SIZE / 2.0),
        }

        // Jump to the other side, if the player hits the edge.
        if self.player_x_pos() >= self.width {
            self.set_player_x_pos(0.0);
        }
        if self.player_x_pos() < 0.0 {
            self.set_player_x_pos(self.width - FONT_SIZE / 2.0);
        }
        if self.player_y_pos() >= self.height {
            self.set_player_y_pos(0.0);
        }
        if self.player_y_pos() < 0.0 {
            self.set_player_y_pos(self.height - FONT_SIZE / 2.0);
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
        for i in 0..=self.player_size {
            draw_text(
                ROFLCOPTER[i],
                self.player_pos[i].0,
                self.player_pos[i].1,
                FONT_SIZE,
                WHITE,
            );
        }
    }

    pub fn new(width: f32, height: f32) -> PlayerState {
        PlayerState {
            width,
            height,
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
            player_direction: Direction::EAST,
            player_last_move_time: macroquad::time::get_time(),
            player_size: 0,
        }
    }
}
