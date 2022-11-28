use serde::{Serialize, Deserialize};
use crate::pathfinder::Pathfinder;

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone)]
pub struct GameConfig {
    // How many games will run consecutively.
    pub instance_count: i32,
    // The amount of horizontal slots that can be occupied by game objects.
    pub horizontal_slots: i32,
    // The amount of vertical slots that can be occupied by game objects.
    pub vertical_slots: i32,
    // Which pathfinder to use for the snake.
    pub pathfinder: Pathfinder
}
