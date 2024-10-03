use bevy::prelude::*;

pub fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/background.png"),
        transform: Transform::from_xyz(0., 192., 0.).with_scale(Vec3::ONE * 3.),
        ..default()
    });
}

pub fn spawn_lamp_posts(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/lamp_post.png"),
        transform: Transform::from_xyz(-480., 30., 3.).with_scale(Vec3::ONE * 2.),
        ..default()
    });
}
