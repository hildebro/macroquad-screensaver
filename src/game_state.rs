use macroquad::window::{screen_height, screen_width};

use crate::game_instance::GameInstance;

pub struct GameState {
    game_instances: Vec<GameInstance>,
    window_width: f32,
    window_height: f32,
}

impl GameState {
    pub fn new(game_instances: Vec<GameInstance>) -> Self {
        GameState {
            game_instances,
            window_width: screen_width(),
            window_height: screen_height(),
        }
    }

    pub fn update(&mut self) {
        for game_instance in &mut self.game_instances {
            game_instance.update();
        }

        self.update_absolute_size()
    }

    pub fn draw(&self) {
        for game_instance in &self.game_instances {
            game_instance.draw();
        }
    }

    /// Update width and height in case the user resizes the window.
    fn update_absolute_size(&mut self) {
        self.window_width = screen_width();
        self.window_height = screen_height();
    }
}
