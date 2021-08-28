use macroquad::prelude::*;

use crate::constants::*;
use crate::pathfinder::*;
use crate::player_state::PlayerState;

pub struct GameState {
    pub width: f32,
    pub height: f32,
    pub char_x_pos: f32,
    pub char_y_pos: f32,
    // The char index of ROFLCOPTER to render.
    pub char_index: usize,
    pub player_state: PlayerState,
    pub pathfinder: Pathfinder,
}

impl GameState {
    pub fn update(&mut self) {
        self.update_absolute_size();
        self.update_player_direction();
        self.move_player();
        self.collision_check();
    }

    pub fn update_player_direction(&mut self) {
        for pathfinder_tuple in PATHFINDER_MAPPING {
            if self.pathfinder != pathfinder_tuple.0 {
                // Continue until the pathfinder matches.
                continue;
            }

            let pathfinder_fn = pathfinder_tuple.1;
            let pathfinder_result = pathfinder_fn(self);
            match pathfinder_result {
                PathfinderResult::KeepGoing => {}
                PathfinderResult::NewDirection(direction) => {
                    self.player_state.player_direction = direction;
                }
            }
        }
    }

    pub fn draw(&self) {
        // Draw the player.
        self.player_state.draw();

        // Draw the char.
        draw_text(
            self.char_to_render(),
            self.char_x_pos,
            self.char_y_pos,
            FONT_SIZE,
            self.player_state.color,
        );
    }

    /// Update width and height in case the user resizes the window.
    pub fn update_absolute_size(&mut self) {
        self.width = macroquad::window::screen_width();
        self.height = macroquad::window::screen_height();
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
        let (char_x_pos, char_y_pos) = new_char_pos(self.width, self.height);
        self.char_x_pos = char_x_pos;
        self.char_y_pos = char_y_pos;

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
        if self.player_state.player_x_pos() >= self.width {
            self.player_state.set_player_x_pos(0.0);
        }
        if self.player_state.player_x_pos() < 0.0 {
            self.player_state
                .set_player_x_pos(self.width - FONT_SIZE / 2.0);
        }
        if self.player_state.player_y_pos() >= self.height {
            self.player_state.set_player_y_pos(0.0);
        }
        if self.player_state.player_y_pos() < 0.0 {
            self.player_state
                .set_player_y_pos(self.height - FONT_SIZE / 2.0);
        }
    }

    pub fn new() -> GameState {
        let width = macroquad::window::screen_width();
        let height = macroquad::window::screen_height();

        let (char_x_pos, char_y_pos) = new_char_pos(width, height);

        GameState {
            width,
            height,
            char_x_pos,
            char_y_pos,
            char_index: 1,
            player_state: PlayerState::new(),
            pathfinder: Pathfinder::LazyWalker,
        }
    }
}
