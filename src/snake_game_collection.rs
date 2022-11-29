use crate::snake_config::SnakeConfig;
use crate::snake_game::SnakeGame;

pub struct SnakeGameCollection {
    snake_games: Vec<SnakeGame>,
}

impl SnakeGameCollection {
    pub fn new() -> Self {
        let snake_config = SnakeConfig::get_config();

        // Build game instances based on the specified amount from the game config.
        let mut snake_games = Vec::new();
        for _ in 0..snake_config.instance_count {
            snake_games.push(SnakeGame::new(snake_config));
        }

        SnakeGameCollection { snake_games }
    }

    pub fn update(&mut self) {
        for snake_game in &mut self.snake_games {
            snake_game.update();
        }
    }

    pub fn draw(&self) {
        for snake_game in &self.snake_games {
            snake_game.draw();
        }
    }
}
