use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::prelude::*;
use roflcopter_snake_lib::game_state::GameState;

pub const INSTANCE_COUNT: usize = 500;

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

    let mut game_state = GameState::new(INSTANCE_COUNT);

    loop {
        game_state.update();
        game_state.draw();

        // Draw fps here so that you don't see it, when crate is used as lib.
        draw_text(
            &macroquad::time::get_fps().to_string(),
            50.0,
            50.0,
            40.0,
            WHITE,
        );

        next_frame().await
    }
}
