use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::prelude::rand;

use crate::game_instance::GameInstance;
use crate::game_state::GameState;
use crate::game_state_config::GameStateConfig;
use crate::pathfinder::Pathfinder;

pub fn build_game_state(game_state_config: GameStateConfig) -> GameState {
    // Set seed for randomness.
    let current_millisecond = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Unable to read system time.");
    rand::srand(current_millisecond.as_secs());

    // Pick a random pathfinder to start with.
    let pathfinder = if rand::gen_range(0, 2) == 0 {
        Pathfinder::LazyWalker
    } else {
        Pathfinder::StepWalker
    };

    let mut game_instances = Vec::new();
    for _ in 0..game_state_config.instance_count {
        game_instances.push(GameInstance::new(pathfinder));
    }

    GameState::new(game_instances)
}
