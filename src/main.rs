use macroquad::prelude::*;

// Characters to render.
const ROFLCOPTER: [&'static str; 10] = ["R", "O", "F", "L", "C", "O", "P", "T", "E", "R"];

#[macroquad::main("Roflcopter")]
async fn main() {
    // Denotes which ROFLCOPTER character to render per loop.
    let mut next_char_index: usize = 0;

    loop {
        // Get window size inside loop to handle resizing.
        let height = screen_height();
        let width = screen_width();

        // Get random coordinates for the current loop.
        let y_pos = rand::gen_range(0.0, height);
        let x_pos = rand::gen_range(0.0, width);

        // Draw the letter.
        draw_text(ROFLCOPTER[next_char_index], x_pos, y_pos, 30.0, WHITE);

        // Decide which character to render in the next loop.
        if next_char_index < 9 {
            next_char_index += 1;
        } else {
            next_char_index = 0;
        }

        // Draw fps
        draw_text(&macroquad::time::get_fps().to_string(), 50.0,50.0, 50.0, WHITE);

        next_frame().await
    }
}
