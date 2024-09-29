use crate::bottle::resources::SpawnLocation;
use crate::bottle::systems::{
    adjust_angular_damping, adjust_linear_damping, debug_physics_values, spawn_bottle,
};
use crate::config::ASSETS_SCALE_FACTOR;
use crate::progression::GameState;
use bevy::prelude::*;

mod systems;

pub mod components;
mod resources;

pub struct BottlePlugin;

impl Plugin for BottlePlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, debug_physics_values);
        app.insert_resource(SpawnLocation(Vec3::new(
            -90. * ASSETS_SCALE_FACTOR,
            0. * ASSETS_SCALE_FACTOR,
            10.,
        )));
        app.add_systems(OnEnter(GameState::InGame), spawn_bottle);
        app.add_systems(
            Update,
            (adjust_angular_damping, adjust_linear_damping).run_if(in_state(GameState::InGame)),
        );
    }
}
