use bevy::prelude::*;
use crate::progression::systems::check_waypoint_completed;

mod systems;
pub mod components;


#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProgressionWaveState {
    FirstWave,
    SecondWave,
    ThirdWave,
}


pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(ProgressionWaveState::FirstWave);
        app.add_systems(Update, check_waypoint_completed);
    }
}