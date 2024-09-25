use crate::level::systems::spawn_background;
use bevy::prelude::*;

mod systems;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LevelState {
    FirstWave,
    SecondWave,
    ThirdWave,
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(LevelState::FirstWave);
        app.add_systems(Startup, spawn_background);
    }
}
