use bevy::prelude::*;

pub fn spawn_ground(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/ground.png"),
        transform: Transform::from_xyz(96., -864., 0.).with_scale(Vec3::ONE * 3.),
        ..default()
    });
}

pub fn spawn_buildings(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/buildings.png"),
        transform: Transform::from_xyz(0., -240., 0.).with_scale(Vec3::ONE * 3.),
        ..default()
    });
}

pub fn spawn_skyline_far(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/skyline_far.png"),
        transform: Transform::from_xyz(0., 85. * 3., -3.).with_scale(Vec3::ONE * 3.),
        ..default()
    });
}

pub fn spawn_skyline_partway(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/skyline_partway.png"),
        transform: Transform::from_xyz(0., 110. * 3., -2.).with_scale(Vec3::ONE * 3.),
        ..default()
    });
}

pub fn spawn_skyline_near(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/skyline_near.png"),
        transform: Transform::from_xyz(0., 114. * 3., -1.).with_scale(Vec3::ONE * 3.),
        ..default()
    });
}

pub fn spawn_night_sky(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/night_sky.png"),
        transform: Transform::from_xyz(0., 149. * 3., -10.).with_scale(Vec3::ONE * 3.),
        ..default()
    });
}
