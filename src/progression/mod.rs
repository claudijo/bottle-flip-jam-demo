use crate::progression::systems::check_waypoint_completed;
use bevy::prelude::*;

pub mod components;
mod systems;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProgressionWaveState {
    First,
    Second,
    Third,
}

pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(ProgressionWaveState::First);
        app.add_systems(Update, check_waypoint_completed);
    }
}
