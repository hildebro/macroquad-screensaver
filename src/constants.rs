// Characters to render.
pub const ROFLCOPTER: [&'static str; 10] = ["R", "O", "F", "L", "C", "O", "P", "T", "E", "R"];

pub const PLAYER_START_X_POS: f32 = 120.0;
pub const PLAYER_START_Y_POS: f32 = 120.0;

pub const PLAYER_MOVE_INTERVAL: f64 = 0.25;

pub const CHAR_LIFETIME: f64 = 5.0;

pub const COLLISION_RANGE: f32 = 20.0;

pub const FONT_SIZE: f32 = 60.0;

pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}