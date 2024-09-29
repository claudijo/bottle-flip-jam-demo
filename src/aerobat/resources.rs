use bevy::prelude::*;

#[derive(Resource)]
pub struct RestingThreshold {
    pub linear: f32,
    pub angular: f32,
}

#[derive(Resource)]
pub struct RestingActivationTime(pub f32);
