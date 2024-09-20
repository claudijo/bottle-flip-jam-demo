use bevy::prelude::*;
use crate::platforms::systems::{spawn_bench, spawn_floor};

mod systems;

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_floor, spawn_bench));
    }
}