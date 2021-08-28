use crate::constants::*;
use crate::game_state::GameState;

#[derive(PartialEq)]
pub enum Pathfinder {
    // Keeps going in one direction until aligned with the collectible. Then turn towards it.
    LazyWalker,
    // Keep switching planes in the most direct way towards the collectible.
    StepWalker,
}

pub enum PathfinderResult {
    KeepGoing,
    NewDirection(Direction),
}

type PathfinderFn = fn(&GameState) -> PathfinderResult;

pub static PATHFINDER_MAPPING: &'static [(Pathfinder, PathfinderFn)] = &[
    (Pathfinder::LazyWalker, lazy_walker_fn),
    (Pathfinder::StepWalker, step_walker_fn),
];

fn lazy_walker_fn(game_state: &GameState) -> PathfinderResult {
    let plane_of_direction = plane_of_direction(&game_state.player_state.player_direction);
    let player_x_pos = game_state.player_state.player_x_pos();
    let player_y_pos = game_state.player_state.player_y_pos();

    if player_x_pos != game_state.char_x_pos && plane_of_direction == Plane::Horizontal {
        // If we aren't aligned with the collectible horizontally while traversing the horizontal
        // plane, just keep going.
        return PathfinderResult::KeepGoing;
    }

    if player_y_pos != game_state.char_y_pos && plane_of_direction == Plane::Vertical {
        // If we aren't aligned with the collectible vertically while traversing the vertical plane,
        // just keep going.
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

fn step_walker_fn(game_state: &GameState) -> PathfinderResult {
    let player_x_pos = game_state.player_state.player_x_pos();
    let player_y_pos = game_state.player_state.player_y_pos();

    return if player_x_pos == game_state.char_x_pos {
        // The player is aligned horizontally, so just pick the correct vertical direction.
        if player_y_pos < game_state.char_y_pos {
            PathfinderResult::NewDirection(Direction::South)
        } else {
            PathfinderResult::NewDirection(Direction::North)
        }
    } else if player_y_pos == game_state.char_y_pos {
        // The player is aligned vertically, so just pick the correct horizontal direction.
        if player_x_pos < game_state.char_x_pos {
            PathfinderResult::NewDirection(Direction::East)
        } else {
            PathfinderResult::NewDirection(Direction::West)
        }
    } else {
        // Not aligned at all, so there are two valid directions to take at this point. We don't
        // want to continue on the current direction, so we switch based on that.
        let plane = plane_of_direction(&game_state.player_state.player_direction);
        match plane {
            Plane::Vertical => {
                if player_x_pos < game_state.char_x_pos {
                    PathfinderResult::NewDirection(Direction::East)
                } else {
                    PathfinderResult::NewDirection(Direction::West)
                }
            }
            Plane::Horizontal => {
                if player_y_pos < game_state.char_y_pos {
                    PathfinderResult::NewDirection(Direction::South)
                } else {
                    PathfinderResult::NewDirection(Direction::North)
                }
            }
        }
    };
}
