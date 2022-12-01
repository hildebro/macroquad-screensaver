use std::fs;

use serde::{Deserialize, Serialize};

use crate::constants::StartingPosition;
use crate::pathfinder::Pathfinder;

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone)]
pub struct SnakeConfig {
    // How many games will run consecutively.
    pub instance_count: i32,
    // The amount of horizontal slots that can be occupied by game objects.
    pub horizontal_slots: i32,
    // The amount of vertical slots that can be occupied by game objects.
    pub vertical_slots: i32,
    // Which pathfinder to use for the snake.
    pub pathfinder: Pathfinder,
    // Where the player will start from.
    pub starting_position: StartingPosition,
    // Whether to draw the fps counter in the corner of the screen
    pub draw_fps: bool,
}

impl SnakeConfig {
    pub fn get_config() -> SnakeConfig {
        // Create the config from default, if it doesn't exist yet.
        if !fs::try_exists("config.yaml").unwrap() {
            fs::copy("config.default.yaml", "config.yaml").expect("Unable to initialize config.");
        }

        // Read the config.
        let snake_config_yaml = fs::read_to_string("config.yaml").expect("Unable to read config.");

        serde_yaml::from_str(snake_config_yaml.as_str()).unwrap()
    }
}
