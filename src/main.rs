use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::prelude::*;

use roflcopter_snake_lib::game_state_factory::*;

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
        .expect("Unable to read system time.");
    rand::srand(current_millisecond.as_secs());

    let mut game_state = build_game_state();

    loop {
        game_state.update();
        game_state.draw();

        // Draw fps outside of game state so that it's not rendered, when the crate is used as lib.
        draw_text(&get_fps().to_string(), 50.0, 50.0, 40.0, WHITE);

        next_frame().await
    }
}
