use crate::game_instance::GameInstance;

pub struct GameState {
    game_instances: Vec<GameInstance>,
}

impl GameState {
    pub fn new(game_instances: Vec<GameInstance>) -> Self {
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
