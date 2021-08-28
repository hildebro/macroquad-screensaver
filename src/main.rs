use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::prelude::*;

use game_state::GameState;

mod game_state;
mod constants;
mod player_state;
mod pathfinder;

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

    let mut game_instances: Vec<GameState> = Vec::new();
    for _ in 0..constants::INSTANCE_COUNT {
        game_instances.push(GameState::new())
    }

    loop {
        for game_state in &mut game_instances {
            game_state.update();
            game_state.draw();
        }

        next_frame().await
    }
}
