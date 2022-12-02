use crate::enums::Plane;

#[derive(Clone, Copy)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn get_plane(direction: &Direction) -> Plane {
        match direction {
            Direction::North => Plane::Vertical,
            Direction::South => Plane::Vertical,
            Direction::West => Plane::Horizontal,
            Direction::East => Plane::Horizontal,
        }
    }
}
