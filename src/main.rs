use macroquad::prelude::*;

const LETTER: [&'static str; 10] = ["R", "O", "F", "L", "C", "O", "P", "T", "E", "R"];

#[macroquad::main("Roflcopter")]
async fn main() {

    let mut iteration: usize = 0;
    loop {
        let height = screen_height();
        let width = screen_width();

        let y_pos = rand::gen_range(0.0, height);
        let x_pos = rand::gen_range(0.0, width);

        draw_text(LETTER[iteration], x_pos, y_pos, 30.0, PINK);

        iteration += 1;
        if iteration > 9 {
            iteration = 0;
        }

        next_frame().await
    }
}
