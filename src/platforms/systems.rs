use bevy::prelude::*;

pub fn spawn_floor(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("floor.png"),
        transform: Transform::from_xyz(0., -560., 1.).with_scale(Vec3::ONE * 5.),
        ..default()
    });
}

pub fn spawn_bench(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("bench.png"),
        transform: Transform::from_xyz(-85., -435., 2.).with_scale(Vec3::ONE * 5.),
        ..default()
    });
}