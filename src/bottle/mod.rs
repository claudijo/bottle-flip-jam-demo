use crate::bottle::systems::spawn_bottle;
use bevy::prelude::*;

mod systems;

pub mod components;

pub struct BottlePlugin;

impl Plugin for BottlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bottle);
    }
}
