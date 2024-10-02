use crate::backgrounds::systems::spawn_background;
use bevy::prelude::*;

mod systems;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_background);
    }
}
