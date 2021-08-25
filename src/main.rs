mod game_state;

use macroquad::prelude::*;
use game_state::GameState;
use crate::game_state::Direction;

#[macroquad::main("Roflcopter")]
async fn main() {
    let width = screen_width();
    let height = screen_height();
    let mut game_state = GameState::new(
        width,
        height,
        // Get random coordinates for the char to appear at.
        rand::gen_range(0.0, width),
        rand::gen_range(0.0, height),
        macroquad::time::get_time());

    loop {
        // Update width and height in case the user resized the window.
        game_state.update_absolute_size();

        // Draw fps
        draw_text(&macroquad::time::get_fps().to_string(), 50.0, 50.0, 50.0, WHITE);

        // Draw the letter.
        draw_text(
            game_state.char_to_render(),
            game_state.char_x_pos,
            game_state.char_y_pos,
            50.0,
            WHITE,
        );

        // Switch direction on key input
        if macroquad::input::is_key_down(KeyCode::W) {
            game_state.set_direction(Direction::NORTH);
        } else if macroquad::input::is_key_down(KeyCode::D) {
            game_state.set_direction(Direction::EAST);
        } else if macroquad::input::is_key_down(KeyCode::S) {
            game_state.set_direction(Direction::SOUTH);
        } else if macroquad::input::is_key_down(KeyCode::A) {
            game_state.set_direction(Direction::WEST);
        }

        // Move the player
        game_state.move_player();

        // Draw the player
        draw_text(
            "R",
            game_state.player_x_pos,
            game_state.player_y_pos,
            50.0,
            WHITE,
        );

        let loop_time = macroquad::time::get_time();
        if loop_time - game_state.char_birthtime > 0.5 {
            game_state.update_char(loop_time);
        }

        next_frame().await
    }
}
