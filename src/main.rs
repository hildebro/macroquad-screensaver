use macroquad::prelude::*;

use roflcopter_snake_lib::game_state_config::GameStateConfig;
use roflcopter_snake_lib::game_state_factory::*;

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
    let mut game_state = build_game_state(GameStateConfig {
        instance_count: INSTANCE_COUNT,
    });

    loop {
        game_state.update();
        game_state.draw();

        // Draw fps here so that you don't see it, when crate is used as lib.
        draw_text(
            &get_fps().to_string(),
            50.0,
            50.0,
            40.0,
            WHITE,
        );

        next_frame().await
    }
}
