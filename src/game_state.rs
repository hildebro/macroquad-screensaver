use crate::game_config::GameConfig;
use crate::game_instance::GameInstance;

pub struct GameState {
    game_instances: Vec<GameInstance>,
}

impl GameState {
    pub fn new() -> Self {
        let game_config = GameConfig::get_config();

        // Build game instances based on the specified amount from the game config.
        let mut game_instances = Vec::new();
        for _ in 0..game_config.instance_count {
            game_instances.push(GameInstance::new(game_config));
        }

        GameState { game_instances }
    }

    pub fn update(&mut self) {
        for game_instance in &mut self.game_instances {
            game_instance.update();
        }
    }

    pub fn draw(&self) {
        for game_instance in &self.game_instances {
            game_instance.draw();
        }
    }
}
