use crate::bottle::resources::SpawnPoint;
use crate::bottle::systems::{
    adjust_angular_damping, adjust_linear_damping, despawn_bottle,
    filter_collisions_for_grabbed_bottle, set_spawn_point_1, set_spawn_point_2, set_spawn_point_3,
    set_spawn_point_4, spawn_bottle,
};
use crate::progression::states::{GameState, LevelState, RoundState};
use avian2d::prelude::*;
use bevy::prelude::*;

mod systems;

pub mod components;
mod resources;

pub struct BottlePlugin;

impl Plugin for BottlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnPoint(Vec3::ZERO));

        app.add_systems(
            OnEnter(RoundState::Start),
            spawn_bottle.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            OnExit(RoundState::Start),
            despawn_bottle.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            Update,
            (adjust_angular_damping, adjust_linear_damping).run_if(in_state(GameState::InGame)),
        );

        // https://docs.rs/avian2d/latest/avian2d/schedule/struct.PostProcessCollisions.html#example
        app.add_systems(
            PostProcessCollisions,
            filter_collisions_for_grabbed_bottle.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(OnExit(LevelState::Initial), set_spawn_point_1);
        app.add_systems(OnExit(LevelState::First), set_spawn_point_2);
        app.add_systems(OnExit(LevelState::Second), set_spawn_point_3);
        app.add_systems(OnExit(LevelState::Third), set_spawn_point_4);
    }
}
