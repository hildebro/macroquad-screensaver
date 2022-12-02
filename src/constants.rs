use serde::{Deserialize, Serialize};

// Characters to render.
pub const ROFLCOPTER: [&'static str; 10] = ["R", "O", "F", "L", "C", "O", "P", "T", "E", "R"];

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone)]
pub enum StartingPosition {
    Corner,
    Center,
    Random,
}

#[derive(Clone, Copy)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(PartialEq)]
pub enum Plane {
    Vertical,
    Horizontal,
}

pub fn plane_of_direction(direction: &Direction) -> Plane {
    match direction {
        Direction::North => Plane::Vertical,
        Direction::South => Plane::Vertical,
        Direction::West => Plane::Horizontal,
        Direction::East => Plane::Horizontal,
    }
}
