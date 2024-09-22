use crate::platforms::systems::{
    animate_fan, spawn_bench, spawn_cardboard_boxes, spawn_fan, spawn_ground,
};
use bevy::prelude::*;

mod components;
mod systems;

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (spawn_ground, spawn_bench, spawn_cardboard_boxes, spawn_fan),
        );
        app.add_systems(Update, animate_fan);
    }
}
