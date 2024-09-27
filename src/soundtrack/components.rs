use bevy::prelude::*;

// This component will be attached to an entity to fade the audio in
#[derive(Component)]
pub struct FadeIn;

// This component will be attached to an entity to fade the audio out
#[derive(Component)]
pub struct FadeOut;
