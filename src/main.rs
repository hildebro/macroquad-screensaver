use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::prelude::*;

use game_state::GameState;

mod game_state;
mod constants;
mod player_state;

fn window_conf() -> Conf {
    Conf {
        window_title: "Roflcopter".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Set seed for randomness.
    let current_millisecond = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Unable to read systemt time.");
    rand::srand(current_millisecond.as_secs());

    let mut game_state = GameState::new();

    loop {
        game_state.update();
        game_state.draw();

        next_frame().await
    }
}
