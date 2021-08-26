use macroquad::prelude::*;

use constants::*;
use game_state::GameState;

mod game_state;
mod constants;

#[macroquad::main("Roflcopter")]
async fn main() {
    let mut game_state = GameState::new();

    loop {
        // Update width and height in case the user resized the window.
        game_state.update_absolute_size();

        // Update char position, if necessary.
        game_state.update_char();

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

        // Draw window size.
        draw_text(
            &format!("{} x {}", game_state.width, game_state.height),
            game_state.width - 300.0,
            50.0,
            FONT_SIZE,
            WHITE,
        );

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
        draw_text(
            "R",
            game_state.player_x_pos,
            game_state.player_y_pos,
            FONT_SIZE,
            WHITE,
        );

        next_frame().await
    }
}
