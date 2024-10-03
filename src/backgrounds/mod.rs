use crate::backgrounds::systems::{spawn_background, spawn_lamp_posts};
use bevy::prelude::*;

mod systems;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_background);
        app.add_systems(Startup, spawn_lamp_posts);
    }
}
