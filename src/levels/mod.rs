use crate::levels::systems::{
    spawn_buildings, spawn_ground, spawn_night_sky, spawn_skyline_far, spawn_skyline_near,
    spawn_skyline_partway,
};
use bevy::prelude::*;

mod systems;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_ground,
                spawn_buildings,
                spawn_skyline_far,
                spawn_skyline_partway,
                spawn_skyline_near,
                spawn_night_sky,
            ),
        );
    }
}
