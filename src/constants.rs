use macroquad::prelude::rand;

// Characters to render.
pub const ROFLCOPTER: [&'static str; 10] = ["R", "O", "F", "L", "C", "O", "P", "T", "E", "R"];

pub const PLAYER_START_X_POS: f32 = 120.0;
pub const PLAYER_START_Y_POS: f32 = 120.0;

pub const PLAYER_MOVE_INTERVAL: f64 = 0.15;

pub const FONT_SIZE: f32 = 120.0;

pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

/// The position of an element needs to be adjusted in order to fit neatly into the grid.
pub fn fix_to_grid(num: f32) -> f32 {
    let remainder = num % FONT_SIZE;

    num - remainder
}

pub fn new_char_pos(width: f32, height: f32) -> (f32, f32) {
    (
        fix_to_grid(rand::gen_range(0.0, width)),
        fix_to_grid(rand::gen_range(0.0, height)),
    )
}
