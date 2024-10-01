use bevy::prelude::*;

#[derive(Event)]
pub struct ScoreEvent {
    pub value: u32,
    pub round_id: u64,
}

impl ScoreEvent {
    pub fn new(value: u32, round_id: u64) -> Self {
        Self { value, round_id }
    }
}

// #[derive(Event)]
// pub struct BonusEvent {
//     pub value: u32,
//     pub round_id: u64,
// }
//
// impl BonusEvent {
//     pub fn new(value: u32, round_id: u64) -> Self {
//         Self { value, round_id }
//     }
// }
