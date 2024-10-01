use crate::config::ASSETS_SCALE_FACTOR;
use bevy::prelude::*;

pub fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/background.png"),
        transform: Transform::from_xyz(0., 64. * ASSETS_SCALE_FACTOR, 0.)
            .with_scale(Vec3::ONE * ASSETS_SCALE_FACTOR),
        ..default()
    });
}
