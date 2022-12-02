use crate::enums::Direction;
use crate::player_state::PlayerState;
use crate::snake_config::CONFIG;

pub fn move_player(player_state: &mut PlayerState, direction: Direction) {
    // Move all body parts one step closer to the head.
    for i in (0..player_state.player_parts.len() - 1).rev() {
        player_state.player_parts[i + 1] = player_state.player_parts[i];
    }

    // Move the head to a new position.
    match direction {
        Direction::West => player_state.set_player_x_pos(player_state.player_x_pos() - 1),
        Direction::East => player_state.set_player_x_pos(player_state.player_x_pos() + 1),
        Direction::North => player_state.set_player_y_pos(player_state.player_y_pos() - 1),
        Direction::South => player_state.set_player_y_pos(player_state.player_y_pos() + 1),
    }

    // Jump to the other side, if the player hits the edge.
    if player_state.player_x_pos() >= CONFIG.horizontal_slots {
        player_state.set_player_x_pos(0);
    }
    if player_state.player_x_pos() < 0 {
        player_state.set_player_x_pos(CONFIG.horizontal_slots - 1);
    }
    if player_state.player_y_pos() >= CONFIG.vertical_slots {
        player_state.set_player_y_pos(0);
    }
    if player_state.player_y_pos() < 0 {
        player_state.set_player_y_pos(CONFIG.vertical_slots - 1);
    }
}
