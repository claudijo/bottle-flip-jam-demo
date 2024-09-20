use bevy::prelude::*;
use crate::bottle::systems::spawn_bottle;

mod systems;

pub struct BottlePlugin;

impl Plugin for BottlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bottle);
    }
}