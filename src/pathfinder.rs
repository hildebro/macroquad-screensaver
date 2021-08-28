use crate::constants::*;
use crate::game_state::GameState;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Pathfinder {
    LazyWalker
}

pub enum PathfinderResult {
    KeepGoing,
    NewDirection(Direction),
}

type PathfinderFn = fn(&GameState) -> PathfinderResult;

pub static PATHFINDER_MAPPING: &'static [(Pathfinder, PathfinderFn)] = &[
    (Pathfinder::LazyWalker, lazy_walker_fn),
];

fn lazy_walker_fn(game_state: &GameState) -> PathfinderResult {
    let plane_of_direction = plane_of_direction(&game_state.player_state.player_direction);
    let player_x_pos = game_state.player_state.player_x_pos();
    let player_y_pos = game_state.player_state.player_y_pos();

    if player_x_pos != game_state.char_x_pos
        && plane_of_direction == Plane::Horizontal {
        // If we aren't above/below the collectible and already moving on the horizontal plane,
        // we just continue;
        return PathfinderResult::KeepGoing;
    }

    if player_y_pos != game_state.char_y_pos
        && plane_of_direction == Plane::Vertical {
        // If we aren't left/right the collectible and already moving on the vertical plane,
        // we just continue;
        return PathfinderResult::KeepGoing;
    }

    // At this point, we know that we need to change direction.
    return if player_x_pos == game_state.char_x_pos {
        // Horizontally aligned, so we need to either go north or south.
        if player_y_pos < game_state.char_y_pos {
            PathfinderResult::NewDirection(Direction::South)
        } else {
            PathfinderResult::NewDirection(Direction::North)
        }
    } else {
        // Vertically aligned, so we need to either go west or east.
        if player_x_pos < game_state.char_x_pos {
            PathfinderResult::NewDirection(Direction::East)
        } else {
            PathfinderResult::NewDirection(Direction::West)
        }
    };
}
