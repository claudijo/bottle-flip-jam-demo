use crate::grabber::resources::{Grabbing, GrabTouchId};
use crate::grabber::systems::{
    drag_using_mouse, drag_using_touch, grab_using_mouse, grab_using_touch, release_using_mouse,
    release_using_touch, spawn_grab_anchor,
};
use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

pub struct GrabberPlugin;

impl Plugin for GrabberPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grabbing::default());
        app.insert_resource(GrabTouchId::default());
        app.add_systems(Startup, spawn_grab_anchor);
        app.add_systems(
            Update,
            (
                grab_using_mouse,
                grab_using_touch,
                drag_using_mouse,
                drag_using_touch,
                release_using_mouse,
                release_using_touch,
            ),
        );
    }
}
