use crate::grabber::events::Released;
use crate::grabber::resources::{GrabTouchId, Grabbing};
use crate::grabber::systems::{
    despawn_grab_anchor, drag_using_mouse, drag_using_touch, grab_using_mouse, grab_using_touch,
    release_using_mouse, release_using_touch, spawn_grab_anchor,
};
use crate::progression::states::GameState;
use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod resources;
mod systems;

pub struct GrabberPlugin;

impl Plugin for GrabberPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Released>();
        app.insert_resource(Grabbing::default());
        app.insert_resource(GrabTouchId::default());

        app.add_systems(OnEnter(GameState::InGame), spawn_grab_anchor);
        app.add_systems(OnEnter(GameState::MainMenu), despawn_grab_anchor);
        app.add_systems(OnEnter(GameState::Restarting), despawn_grab_anchor);

        app.add_systems(
            Update,
            (
                grab_using_mouse,
                grab_using_touch,
                drag_using_mouse,
                drag_using_touch,
                release_using_mouse,
                release_using_touch,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
