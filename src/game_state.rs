use crate::constants::*;

pub struct GameState {
    pub width: f32,
    pub height: f32,
    pub char_x_pos: f32,
    pub char_y_pos: f32,
    // The time at which the current char started to appear.
    pub char_birthtime: f64,
    // The char index of ROFLCOPTER to render.
    pub char_index: usize,
    pub player_x_pos: f32,
    pub player_y_pos: f32,
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
        let y_distance = self.player_y_pos - self.char_y_pos;
        let x_distance = self.player_x_pos - self.char_x_pos;

        if y_distance.abs() < COLLISION_RANGE && x_distance.abs() < COLLISION_RANGE {
            self.player_size += 1;
        }
    }

    pub fn update_char(&mut self)
    {
        let loop_time = macroquad::time::get_time();
        if loop_time - self.char_birthtime <= CHAR_LIFETIME {
            // Don't update, if the char is too young.
            return;
        }

        // New birthtime to compare against.
        self.char_birthtime = loop_time;

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

        self.player_last_move_time = loop_time;

        // Actual move.
        match self.player_direction {
            Direction::NORTH => self.player_y_pos -= FONT_SIZE / 2.0,
            Direction::EAST => self.player_x_pos += FONT_SIZE / 2.0,
            Direction::SOUTH => self.player_y_pos += FONT_SIZE / 2.0,
            Direction::WEST => self.player_x_pos -= FONT_SIZE / 2.0,
        }

        // Jump to the other side, if the player hits the edge.
        if self.player_x_pos > self.width {
            self.player_x_pos = 0.0;
        }
        if self.player_x_pos < 0.0 {
            self.player_x_pos = self.width;
        }
        if self.player_y_pos > self.height {
            self.player_y_pos = 0.0;
        }
        if self.player_y_pos < 0.0 {
            self.player_y_pos = self.height
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
            char_birthtime: macroquad::time::get_time(),
            char_index: 0,
            player_x_pos: PLAYER_START_X_POS,
            player_y_pos: PLAYER_START_Y_POS,
            player_direction: Direction::EAST,
            player_last_move_time: macroquad::time::get_time(),
            player_size: 0,
        }
    }
}
