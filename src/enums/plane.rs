use crate::enums::Direction;

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
