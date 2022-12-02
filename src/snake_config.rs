use std::fs;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::constants::StartingPosition;
use crate::player::Pathfinder;

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone)]
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

lazy_static! {
    pub static ref CONFIG: SnakeConfig = {
        // Create the config from default, if it doesn't exist yet.
        if !fs::try_exists("config.yaml").unwrap() {
            fs::copy("config.default.yaml", "config.yaml").expect("Unable to initialize config.");
        }

        // Read the config.
        let snake_config_yaml = fs::read_to_string("config.yaml").expect("Unable to read config.");

        serde_yaml::from_str(snake_config_yaml.as_str()).unwrap()
    };
}
