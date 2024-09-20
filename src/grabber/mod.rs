use crate::grabber::systems::{
    drag_using_mouse, grab_using_mouse, release_using_mouse, spawn_grab_anchor,
};
use bevy::prelude::*;

pub mod components;
mod systems;

pub struct GrabberPlugin;

impl Plugin for GrabberPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_grab_anchor);
        app.add_systems(
            Update,
            (grab_using_mouse, drag_using_mouse, release_using_mouse),
        );
    }
}
