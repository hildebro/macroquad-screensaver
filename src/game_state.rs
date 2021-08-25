use macroquad::prelude::rand;

// Characters to render.
const ROFLCOPTER: [&'static str; 10] = ["R", "O", "F", "L", "C", "O", "P", "T", "E", "R"];

pub struct GameState {
    pub width: f32,
    pub height: f32,
    pub char_x_pos: f32,
    pub char_y_pos: f32,
    // The time at which the current char started to appear.
    pub char_birthtime: f64,
    // The char index of ROFLCOPTER to render.
    pub char_index: usize,
}

impl GameState {
    pub(crate) fn update_absolute_size(&mut self) {
        self.width = macroquad::window::screen_width();
        self.height = macroquad::window::screen_height();
    }

    pub fn char_to_render(&self) -> &str {
        ROFLCOPTER[self.char_index]
    }

    pub fn set_char_birthtime(&mut self, char_birthtime: f64) {
        self.char_birthtime = char_birthtime
    }

    pub fn update_char(&mut self, loop_time: f64)
    {
        // New birthtime to compare against.
        self.set_char_birthtime(loop_time);

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

    pub fn new(width: f32, height: f32, char_x_pos: f32, char_y_pos: f32, char_birthtime: f64) -> GameState {
        GameState {
            width,
            height,
            char_x_pos,
            char_y_pos,
            char_birthtime,
            char_index: 0,
        }
    }
}
