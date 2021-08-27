use macroquad::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use constants::*;
use game_state::GameState;

mod game_state;
mod constants;

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
        // Update width and height in case the user resized the window.
        game_state.update_absolute_size();

        // Switch direction on key input.
        if macroquad::input::is_key_down(KeyCode::W) {
            game_state.set_direction(Direction::NORTH);
        } else if macroquad::input::is_key_down(KeyCode::D) {
            game_state.set_direction(Direction::EAST);
        } else if macroquad::input::is_key_down(KeyCode::S) {
            game_state.set_direction(Direction::SOUTH);
        } else if macroquad::input::is_key_down(KeyCode::A) {
            game_state.set_direction(Direction::WEST);
        }

        // Move the player.
        game_state.move_player();

        // Check collision.
        game_state.collision_check();

        // Draw fps.
        // draw_text(&macroquad::time::get_fps().to_string(), 50.0, 50.0, FONT_SIZE, WHITE);

        // Draw player size (remove later).
        draw_text(
            &game_state.player_size.to_string(),
            50.0,
            game_state.height - 50.0,
            FONT_SIZE,
            WHITE,
        );

        // Draw the char.
        draw_text(
            game_state.char_to_render(),
            game_state.char_x_pos,
            game_state.char_y_pos,
            FONT_SIZE,
            WHITE,
        );

        // Draw the player.
        game_state.draw_player();

        next_frame().await
    }
}
