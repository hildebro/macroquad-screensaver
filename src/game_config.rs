#[derive(Copy, Clone)]
pub struct GameConfig {
    // How many games will run consecutively.
    pub instance_count: i32,
    // The amount of horizontal slots that can be occupied by game objects.
    pub horizontal_slots: i32,
    // The amount of vertical slots that can be occupied by game objects.
    pub vertical_slots: i32
}
