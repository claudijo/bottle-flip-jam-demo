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
    commands
        .spawn((
            VisibilityBundle::default(),
            TransformBundle::from_transform(
                Transform::from_xyz(-85., -460., 2.)
            ),
            RigidBody::Static,
            Collider::rectangle(145., 50.),
            CollisionLayers::new(
                CustomCollisionLayer::Platform,
                [CustomCollisionLayer::Bottle, CustomCollisionLayer::Platform],
            ),
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                transform: Transform::from_xyz(2.5, 25., 0.).with_scale(Vec3::ONE * 5.),
                texture: asset_server.load("bench.png"),
                ..default()
            });
        });
}
