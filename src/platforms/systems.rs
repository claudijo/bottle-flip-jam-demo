use crate::physics::CustomCollisionLayer;
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn spawn_floor(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            VisibilityBundle::default(),
            TransformBundle::from_transform(
                Transform::from_xyz(0., -485., 1.).with_scale(Vec3::ONE * 5.),
            ),
            RigidBody::Static,
            Collider::half_space(Vec2::Y),
            CollisionLayers::new(
                CustomCollisionLayer::Platform,
                [CustomCollisionLayer::Bottle, CustomCollisionLayer::Platform],
            ),
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                transform: Transform::from_xyz(0., -15., 0.),
                texture: asset_server.load("floor.png"),
                ..default()
            });
        });
}

pub fn spawn_bench(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("bench.png"),
        transform: Transform::from_xyz(-85., -435., 2.).with_scale(Vec3::ONE * 5.),
        ..default()
    });
}
