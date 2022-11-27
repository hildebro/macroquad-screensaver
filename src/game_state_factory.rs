use macroquad::prelude::rand;

use crate::game_config::GameConfig;
use crate::game_instance::GameInstance;
use crate::game_state::GameState;
use crate::pathfinder::Pathfinder;

pub fn build_game_state(game_config: GameConfig) -> GameState {
    // Pick a random pathfinder to start with.
    let pathfinder = if rand::gen_range(0, 2) == 0 {
        Pathfinder::LazyWalker
    } else {
        Pathfinder::StepWalker
    };

    let mut game_instances = Vec::new();
    for _ in 0..game_config.instance_count {
        game_instances.push(GameInstance::new(pathfinder, game_config));
    }

    GameState::new(game_instances)
}
