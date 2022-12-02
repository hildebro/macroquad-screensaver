use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum StartingPosition {
    Corner,
    Center,
    Random,
}
