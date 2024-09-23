use crate::camera::systems::{aim_camera, spawn_camera};
use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, aim_camera);
    }
}
