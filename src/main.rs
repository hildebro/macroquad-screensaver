use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::prelude::*;

use roflcopter_snake_lib::snake_game_collection::SnakeGameCollection;

fn window_conf() -> Conf {
    Conf {
        window_title: "Roflcopter Snake".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Set seed for randomness.
    let current_millisecond = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Unable to read system time.");
    rand::srand(current_millisecond.as_secs());

    // Generate a new collection of snake games.
    let mut snake_game_collection = SnakeGameCollection::new();

    loop {
        snake_game_collection.update();
        snake_game_collection.draw();
        next_frame().await
    }
}
