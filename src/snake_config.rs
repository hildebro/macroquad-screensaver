use std::{fs, path::Path};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::enums::{Pathfinder, StartingPosition};

#[derive(Serialize, Deserialize)]
pub struct SnakeConfig {
    // Amount of snake games running consecutively.
    pub instance_count: i32,
    // Amount of horizontal slots that can be occupied by game objects.
    pub horizontal_slots: i32,
    // Amount of vertical slots that can be occupied by game objects.
    pub vertical_slots: i32,
    // Time in seconds that needs to pass between snakes moving to a new position.
    pub move_interval: f64,
    // Algorithm used to determine the route of the snakes.
    pub pathfinder: Pathfinder,
    // Location from which the snakes will start from.
    pub starting_position: StartingPosition,
    // Whether to draw the fps counter in the corner of the screen.
    pub draw_fps: bool,
}

impl Default for SnakeConfig {
    fn default() -> Self {
        Self {
            instance_count: 100,
            horizontal_slots: 80,
            vertical_slots: 45,
            move_interval: 0.05,
            pathfinder: Pathfinder::LazyWalker,
            starting_position: StartingPosition::Center,
            draw_fps: false,
        }
    }
}

lazy_static! {
    pub static ref CONFIG: SnakeConfig = {
        // Parse the config if it exist.
        if Path::new("config.yaml").exists() {
            let snake_config_yaml = fs::read_to_string("config.yaml").expect("Unable to read config.");
            serde_yaml::from_str(snake_config_yaml.as_str()).expect("Unable to deserialize config.")
        } else {
            SnakeConfig::default()
        }
    };
}
