use crate::bottle::systems::{adjust_angular_damping, adjust_linear_damping, spawn_bottle};
use crate::progression::GameState;
use bevy::prelude::*;

mod systems;

pub mod components;

pub struct BottlePlugin;

impl Plugin for BottlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bottle);
        app.add_systems(
            Update,
            (adjust_angular_damping, adjust_linear_damping).run_if(in_state(GameState::InGame)),
        );
    }
}
