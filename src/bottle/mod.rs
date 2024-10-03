use crate::bottle::resources::SpawnPoint;
use crate::bottle::systems::{
    adjust_angular_damping, adjust_linear_damping, despawn_bottle, despawn_bottle_content,
    filter_collisions_for_grabbed_bottle, set_spawn_point_1, set_spawn_point_2, set_spawn_point_3,
    set_spawn_point_4, spawn_bottle, spawn_bottle_content,
};
use crate::config::ASSETS_SCALE_FACTOR;
use crate::progression::states::{GameState, LevelState, RoundState};
use avian2d::prelude::*;
use bevy::prelude::*;

pub mod systems;

pub mod components;
mod resources;

pub const BOTTLE_BODY_SIZE: Vec2 = Vec2::new(8. * ASSETS_SCALE_FACTOR, 15. * ASSETS_SCALE_FACTOR);
pub const BOTTLE_NECK_HEIGHT: f32 = 3. * ASSETS_SCALE_FACTOR;
pub const BOTTLE_CAP_SIZE: Vec2 = Vec2::new(4. * ASSETS_SCALE_FACTOR, 2. * ASSETS_SCALE_FACTOR);
pub const BOTTLE_DENSITY: f32 = 0.1;
pub const CONTENT_RADIUS: f32 = 3. * ASSETS_SCALE_FACTOR;
pub const CONTENT_DENSITY: f32 = 1.;

pub struct BottlePlugin;

impl Plugin for BottlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnPoint(Vec3::ZERO));

        app.add_systems(
            OnEnter(RoundState::Start),
            (spawn_bottle, spawn_bottle_content)
                .chain()
                .run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            OnExit(RoundState::Start),
            (despawn_bottle, despawn_bottle_content).run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            OnEnter(GameState::MainMenu),
            (despawn_bottle, despawn_bottle_content),
        );
        app.add_systems(
            OnEnter(GameState::Restarting),
            (despawn_bottle, despawn_bottle_content),
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
