use bevy::prelude::*;

#[derive(Component)]
pub struct Aerobat;

#[derive(Component)]
pub struct Grounded(pub Option<Entity>);

#[derive(Component)]
pub struct RestingTime(pub f32);

#[derive(Component)]
pub struct Resting;

#[derive(Component)]
pub struct Flipped {
    start_rotation: f32,
}
