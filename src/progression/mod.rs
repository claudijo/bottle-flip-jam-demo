use crate::progression::systems::check_waypoint_completed;
use bevy::prelude::*;

pub mod components;
mod systems;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProgressionWaveState {
    Zero,
    First,
    Second,
    Third,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
}

pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::MainMenu);
        app.insert_state(ProgressionWaveState::Zero);
        app.add_systems(Update, check_waypoint_completed);
    }
}
