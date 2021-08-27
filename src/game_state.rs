use crate::constants::*;
use macroquad::prelude::*;

pub struct GameState {
    pub width: f32,
    pub height: f32,
    pub char_x_pos: f32,
    pub char_y_pos: f32,
    // The char index of ROFLCOPTER to render.
    pub char_index: usize,
    pub player_pos: [(f32, f32); 10],
    pub player_direction: Direction,
    pub player_last_move_time: f64,
    pub player_size: usize,
}

impl GameState {
    pub fn set_direction(&mut self, direction: Direction) {
        self.player_direction = direction;
    }

    pub fn update_absolute_size(&mut self) {
        self.width = macroquad::window::screen_width();
        self.height = macroquad::window::screen_height();
    }

    pub fn char_to_render(&self) -> &str {
        ROFLCOPTER[self.char_index]
    }

    pub fn collision_check(&mut self) {
        let x_distance = self.player_x_pos() - self.char_x_pos;
        let y_distance = self.player_y_pos() - self.char_y_pos;

        if y_distance != 0.0 || x_distance != 0.0 {
            // Nothing to do, if there's no collision.
            return;
        }

        // Increase size of player.
        self.player_size += 1;
        // Force the char update
        self.update_char();
    }

    pub fn update_char(&mut self)
    {
        // New location.
        let (char_x_pos, char_y_pos) = new_char_pos(self.width, self.height);
        self.char_x_pos = char_x_pos;
        self.char_y_pos = char_y_pos;

        // New char to render.
        if self.char_index < 9 {
            self.char_index += 1;
        } else {
            self.char_index = 0;
        }
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

    pub fn draw_player(&self) {
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

    pub fn new() -> GameState {
        let width = macroquad::window::screen_width();
        let height = macroquad::window::screen_height();

        let (char_x_pos, char_y_pos) = new_char_pos(width, height);

        GameState {
            width,
            height,
            char_x_pos,
            char_y_pos,
            char_index: 1,
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
