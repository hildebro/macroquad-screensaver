use std::fs;

use serde::{Deserialize, Serialize};

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
    pub pathfinder: Pathfinder,
}

impl GameConfig {
    pub fn get_config() -> GameConfig {
        // Create game config from default, if it doesn't exist yet.
        if !fs::try_exists("game_config.yaml").unwrap() {
            fs::copy("game_config.default.yaml", "game_config.yaml")
                .expect("Unable to initialize game config.");
        }

        // Read game config.
        let game_config_yaml = fs::read_to_string("game_config.yaml")
            .expect("Should have been able to read the config file");

        serde_yaml::from_str(game_config_yaml.as_str()).unwrap()
    }
}
