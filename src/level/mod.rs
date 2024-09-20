use bevy::prelude::*;
use crate::level::systems::spawn_background;

mod systems;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_background);
    }
}