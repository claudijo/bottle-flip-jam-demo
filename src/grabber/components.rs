use bevy::prelude::*;

#[derive(Component)]
pub struct GrabZone {
    pub radius: f32,
}

#[derive(Component)]
pub struct GrabTarget;

#[derive(Component)]
pub struct GrabAnchor;

#[derive(Component)]
pub struct GrabJoint;
