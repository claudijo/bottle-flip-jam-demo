use bevy::prelude::*;

pub fn spawn_bottle(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("bottle.png"),
        transform: Transform::from_xyz(-400., -400., 10.).with_scale(Vec3::ONE * 5.),
        ..default()
    });
}