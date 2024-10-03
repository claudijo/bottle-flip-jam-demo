use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GrabTouchId(pub Option<u64>);

#[derive(Resource, Default)]
pub struct Grabbing(pub bool);
