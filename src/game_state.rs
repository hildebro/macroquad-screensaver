use macroquad::prelude::rand;

// Characters to render.
const ROFLCOPTER: [&'static str; 10] = ["R", "O", "F", "L", "C", "O", "P", "T", "E", "R"];

const PLAYER_SPEED: f32 = 5.0;

pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST
}

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
    pub player_direction: Direction
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

    pub fn update_char(&mut self, loop_time: f64)
    {
        // New birthtime to compare against.
        self.char_birthtime = loop_time;

        // New location.
        self.char_x_pos = rand::gen_range(0.0, self.width);
        self.char_y_pos = rand::gen_range(0.0, self.height);

        // New char to render.
        if self.char_index < 9 {
            self.char_index += 1;
        } else {
            self.char_index = 0;
        }
    }

    pub fn move_player(&mut self) {
        // Actual move.
        match self.player_direction {
            Direction::NORTH => self.player_y_pos -= PLAYER_SPEED,
            Direction::EAST => self.player_x_pos += PLAYER_SPEED,
            Direction::SOUTH => self.player_y_pos += PLAYER_SPEED,
            Direction::WEST => self.player_x_pos -= PLAYER_SPEED,
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

    pub fn new(width: f32, height: f32, char_x_pos: f32, char_y_pos: f32, char_birthtime: f64) -> GameState {
        GameState {
            width,
            height,
            char_x_pos,
            char_y_pos,
            char_birthtime,
            char_index: 0,
            player_x_pos: width / 2.0,
            player_y_pos: height / 2.0,
            player_direction: Direction::NORTH
        }
    }
}
