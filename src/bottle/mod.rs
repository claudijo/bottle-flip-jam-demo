use crate::bottle::systems::{adjust_angular_damping, adjust_linear_damping, debug_translation, spawn_bottle};
use crate::progression::GameState;
use bevy::prelude::*;
use crate::bottle::resources::SpawnLocation;
use crate::config::ASSETS_SCALE_FACTOR;

mod systems;

pub mod components;
mod resources;

pub struct BottlePlugin;

impl Plugin for BottlePlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, debug_translation);
        app.insert_resource(SpawnLocation(Vec3::new(-90. * ASSETS_SCALE_FACTOR, 0. * ASSETS_SCALE_FACTOR, 10.)));
        app.add_systems(OnEnter(GameState::InGame), spawn_bottle);
        app.add_systems(
            Update,
            (adjust_angular_damping, adjust_linear_damping).run_if(in_state(GameState::InGame)),
        );
    }
}
