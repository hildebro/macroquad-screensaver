use macroquad::prelude::*;

use crate::collectible_state::CollectibleState;
use crate::constants::PLAYER_MOVE_INTERVAL;
use crate::player::{find_path, Pathfinder};
use crate::player_state::PlayerState;
use crate::snake_config::SnakeConfig;

pub struct SnakeGame {
    pub snake_config: SnakeConfig,
    pub player_state: PlayerState,
    pub collectible_state: CollectibleState,
    pub pathfinder: Pathfinder,
    pub color: Color,
}

impl SnakeGame {
    pub fn update(&mut self) {
        // Always check for a pathfinder update.
        self.update_pathfinder();

        // Don't move unless a bit of time has passed since the last move.
        let loop_time = get_time();
        if loop_time - self.player_state.time_of_last_movement <= PLAYER_MOVE_INTERVAL {
            return;
        }

        // Find the new direction for the player.
        let direction = find_path(self);

        // Update the player location.
        self.player_state.update_location(direction);

        // Check, whether the new location matches the collectible.
        self.collision_check();
    }

    /// Updates the pathfinder, if the user clicks the left mouse button.
    fn update_pathfinder(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            match self.pathfinder {
                Pathfinder::LazyWalker => self.pathfinder = Pathfinder::StepWalker,
                Pathfinder::StepWalker => self.pathfinder = Pathfinder::LazyWalker,
            }
        }
    }

    pub fn draw(&self) {
        // get potential font sizes based on absolute size and slots.
        let font_size_by_width = screen_width() / self.snake_config.horizontal_slots as f32;
        let font_size_by_height = screen_height() / self.snake_config.vertical_slots as f32;
        // use the smaller option of the two to ensure it fits the screen.
        let font_size = font_size_by_width.min(font_size_by_height);

        self.player_state.draw(font_size, self.color);
        self.collectible_state.draw(font_size, self.color);
    }

    pub fn collision_check(&mut self) {
        if self.player_state.player_x_pos() == self.collectible_state.x_position
            && self.player_state.player_y_pos() == self.collectible_state.y_position
        {
            self.player_state.register_collision();
            self.collectible_state.set_new_character();
        }
    }

    pub fn new(snake_config: SnakeConfig) -> SnakeGame {
        let r = rand::gen_range(0.5, 1.0);
        let g = rand::gen_range(0.5, 1.0);
        let b = rand::gen_range(0.5, 1.0);

        SnakeGame {
            snake_config,
            player_state: PlayerState::new(snake_config),
            collectible_state: CollectibleState::new(snake_config),
            pathfinder: snake_config.pathfinder,
            color: Color::new(r, g, b, 1.0),
        }
    }
}
