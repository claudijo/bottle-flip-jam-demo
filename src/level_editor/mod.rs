mod components;
mod systems;

use crate::level_editor::systems::{adjust_platform, pan_camera_with_mouse_wheel, pick_editable};
use bevy::prelude::*;

pub struct LevelEditorPlugin;

impl Plugin for LevelEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, pan_camera_with_mouse_wheel);
        app.add_systems(Update, adjust_platform);
        app.add_systems(Update, pick_editable);
    }
}
