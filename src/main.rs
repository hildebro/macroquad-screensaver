use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::prelude::*;

use roflcopter_snake_lib::game_config::GameConfig;
use roflcopter_snake_lib::game_state_factory::*;

pub const INSTANCE_COUNT: i32 = 150;
pub const HORIZONTAL_SLOTS: i32 = 80;
pub const VERTICAL_SLOTS: i32 = 45;

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

    let mut game_state = build_game_state(GameConfig {
        instance_count: INSTANCE_COUNT,
        horizontal_slots: HORIZONTAL_SLOTS,
        vertical_slots: VERTICAL_SLOTS,
    });

    loop {
        game_state.update();
        game_state.draw();

        // Draw fps here so that you don't see it, when crate is used as lib.
        draw_text(&get_fps().to_string(), 50.0, 50.0, 40.0, WHITE);

        next_frame().await
    }
}
