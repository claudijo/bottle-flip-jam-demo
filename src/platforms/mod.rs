use crate::platforms::systems::{
    animate_fan, spawn_bench, spawn_cardboard_boxes, spawn_fan, spawn_ground,
};
use bevy::prelude::*;
use crate::progression::ProgressionWaveState;

mod components;
mod systems;

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ProgressionWaveState::FirstWave), spawn_bench);
        app.add_systems(OnEnter(ProgressionWaveState::SecondWave), spawn_cardboard_boxes);
        app.add_systems(OnEnter(ProgressionWaveState::ThirdWave), spawn_fan);

        app.add_systems(Startup, spawn_ground);
        app.add_systems(Update, animate_fan);
    }
}
