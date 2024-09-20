use bevy::prelude::*;

pub fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background.png"),
        transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::ONE * 5.),
        ..default()
    });
}