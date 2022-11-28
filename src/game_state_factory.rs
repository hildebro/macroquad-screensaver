use macroquad::prelude::rand;
use std::fs;

use crate::game_config::GameConfig;
use crate::game_instance::GameInstance;
use crate::game_state::GameState;
use crate::pathfinder::Pathfinder;

pub fn build_game_state() -> GameState {
    // Pick a random pathfinder to start with.
    let pathfinder = if rand::gen_range(0, 2) == 0 {
        Pathfinder::LazyWalker
    } else {
        Pathfinder::StepWalker
    };

    // Create game config from default, if it doesn't exist yet.
    if !fs::try_exists("game_config.yaml").unwrap() {
        fs::copy("game_config.default.yaml", "game_config.yaml")
            .expect("Unable to initialize game config.");
    }

    // Read game config.
    let game_config_yaml = fs::read_to_string("game_config.yaml")
        .expect("Should have been able to read the config file");
    let game_config: GameConfig = serde_yaml::from_str(game_config_yaml.as_str()).unwrap();

    // Build game instances based on the specified amount from the game config.
    let mut game_instances = Vec::new();
    for _ in 0..game_config.instance_count {
        game_instances.push(GameInstance::new(pathfinder, game_config));
    }

    GameState::new(game_instances)
}
