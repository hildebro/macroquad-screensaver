use macroquad::prelude::*;

use crate::constants::*;
use crate::game_config::GameConfig;
use crate::pathfinder::*;
use crate::player_state::PlayerState;

pub struct GameInstance {
    pub game_config: GameConfig,
    // The position of the char to get next.
    pub char_x_pos: i32,
    pub char_y_pos: i32,
    // The char index of `ROFLCOPTER` to render.
    pub char_index: usize,
    pub player_state: PlayerState,
    pub pathfinder: Pathfinder,
}

impl GameInstance {
    pub fn update(&mut self) {
        self.update_pathfinder();
        self.update_player_direction();
        self.move_player();
        self.collision_check();
    }

    fn update_pathfinder(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            match self.pathfinder {
                Pathfinder::LazyWalker => self.pathfinder = Pathfinder::StepWalker,
                Pathfinder::StepWalker => self.pathfinder = Pathfinder::LazyWalker,
            }
        }
    }

    pub fn update_player_direction(&mut self) {
        if self.player_state.direction_switch_since_move {
            // Switch direction at most once between moves.
            return;
        }

        self.player_state.player_direction = find_path(self);
        self.player_state.direction_switch_since_move = true;
    }

    pub fn draw(&self) {
        // get potential font sizes based on absolute size and slots.
        let font_size_by_width = screen_width() / self.game_config.horizontal_slots as f32;
        let font_size_by_height = screen_height() / self.game_config.vertical_slots as f32;
        // use the smaller option of the two to ensure it fits the screen.
        let font_size = font_size_by_width.min(font_size_by_height);

        // get absolute positions for the char.
        let abs_char_x_pos = self.char_x_pos as f32 * font_size;
        let abs_char_y_pos = self.char_y_pos as f32 * font_size;

        // Draw the player.
        self.player_state.draw(font_size);

        // Draw the char.
        draw_text(
            self.char_to_render(),
            abs_char_x_pos,
            abs_char_y_pos,
            font_size,
            self.player_state.color,
        );
    }

    pub fn char_to_render(&self) -> &str {
        ROFLCOPTER[self.char_index]
    }

    pub fn collision_check(&mut self) {
        if self.player_state.player_x_pos() != self.char_x_pos
            || self.player_state.player_y_pos() != self.char_y_pos
        {
            // Nothing to do, if there's no collision.
            return;
        }

        self.player_state.register_collision();
        // Force the char update
        self.update_char();
    }

    pub fn update_char(&mut self) {
        // New location.
        self.char_x_pos = rand::gen_range(0, self.game_config.horizontal_slots);
        self.char_y_pos = rand::gen_range(0, self.game_config.vertical_slots);

        // New char to render.
        if self.char_index < 9 {
            self.char_index += 1;
        } else {
            self.char_index = 0;
        }
    }

    pub fn move_player(&mut self) {
        let player_moved = self.player_state.attempt_move();
        if !player_moved {
            // No need to check for out-of-bounds movement, when no move happened.
            return;
        }

        // Jump to the other side, if the player hits the edge.
        if self.player_state.player_x_pos() >= self.game_config.horizontal_slots {
            self.player_state.set_player_x_pos(0);
        }
        if self.player_state.player_x_pos() < 0 {
            self.player_state
                .set_player_x_pos(self.game_config.horizontal_slots - 1);
        }
        if self.player_state.player_y_pos() >= self.game_config.vertical_slots {
            self.player_state.set_player_y_pos(0);
        }
        if self.player_state.player_y_pos() < 0 {
            self.player_state
                .set_player_y_pos(self.game_config.vertical_slots - 1);
        }
    }

    pub fn new(pathfinder: Pathfinder, game_config: GameConfig) -> GameInstance {
        let char_x_pos = rand::gen_range(0, game_config.horizontal_slots);
        let char_y_pos = rand::gen_range(0, game_config.vertical_slots);

        GameInstance {
            game_config,
            char_x_pos,
            char_y_pos,
            char_index: 1,
            player_state: PlayerState::new(),
            pathfinder,
        }
    }
}
