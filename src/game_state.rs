use crate::game_instance::GameInstance;

pub struct GameState {
    game_instances: Vec<GameInstance>,
}

impl GameState {
    pub fn new(instance_count: usize) -> Self {
        let mut game_instances = Vec::new();
        for _ in 0..instance_count {
            game_instances.push(GameInstance::new());
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
