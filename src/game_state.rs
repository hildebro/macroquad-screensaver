use macroquad::prelude::*;

use crate::game_instance::GameInstance;
use crate::pathfinder::Pathfinder;

pub struct GameState {
    game_instances: Vec<GameInstance>,
}

impl GameState {
    pub fn new(instance_count: usize) -> Self {
        // Pick a random pathfinder to start with.
        let pathfinder = if rand::gen_range(0, 2) == 0 {
            Pathfinder::LazyWalker
        } else {
            Pathfinder::StepWalker
        };

        let mut game_instances = Vec::new();
        for _ in 0..instance_count {
            game_instances.push(GameInstance::new(pathfinder));
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
