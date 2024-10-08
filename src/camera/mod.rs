use crate::camera::systems::{aim_camera, spawn_camera, zoom_camera};
use crate::progression::states::GameState;
use avian2d::prelude::*;
use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod utils;

pub const WINDOW_DEFAULT_SIZE: Vec2 = Vec2 { x: 640., y: 360. };
// pub const WINDOW_DEFAULT_SIZE: Vec2 = Vec2 { x: 1280., y: 720. };

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        // To prevent jitter as proposed https://docs.rs/avian2d/latest/avian2d/#why-does-my-camera-following-jitter
        app.add_systems(
            PostUpdate,
            (aim_camera, zoom_camera)
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate)
                .run_if(in_state(GameState::InGame)),
        );
    }
}
