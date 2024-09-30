use crate::config::ASSETS_SCALE_FACTOR;
use crate::physics::CustomCollisionLayer;
use crate::platforms::components::FanAnimationTimer;
use crate::progression::components::WaypointPlatform;
use avian2d::prelude::*;
use bevy::prelude::*;
use std::time::Duration;

pub fn remove_waypoint_marker(
    mut commands: Commands,
    waypoint_platform_query: Query<Entity, With<WaypointPlatform>>,
) {
    for waypoint in &waypoint_platform_query {
        commands.entity(waypoint).remove::<WaypointPlatform>();
    }
}

pub fn spawn_ground(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            VisibilityBundle::default(),
            TransformBundle::from_transform(Transform::from_xyz(
                0.,
                -32. * ASSETS_SCALE_FACTOR,
                1.,
            )),
            RigidBody::Static,
            Collider::half_space(Vec2::Y),
            CollisionLayers::new(
                CustomCollisionLayer::Platform,
                [CustomCollisionLayer::Bottle, CustomCollisionLayer::Platform],
            ),
            Name::new("Ground collider"),
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                transform: Transform::from_xyz(0., -16. * ASSETS_SCALE_FACTOR, 0.)
                    .with_scale(Vec3::ONE * ASSETS_SCALE_FACTOR),
                texture: asset_server.load("ground.png"),
                ..default()
            });
        });
}

pub fn spawn_bench(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            VisibilityBundle::default(),
            TransformBundle::from_transform(Transform::from_xyz(
                -30. * ASSETS_SCALE_FACTOR,
                -27.5 * ASSETS_SCALE_FACTOR,
                2.,
            )),
            RigidBody::Static,
            Collider::rectangle(31. * ASSETS_SCALE_FACTOR, 11. * ASSETS_SCALE_FACTOR),
            CollisionLayers::new(
                CustomCollisionLayer::Platform,
                [CustomCollisionLayer::Bottle, CustomCollisionLayer::Platform],
            ),
            WaypointPlatform,
            Name::new("Bench collider"),
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                transform: Transform::from_xyz(1.5, 4.5 * ASSETS_SCALE_FACTOR, 0.)
                    .with_scale(Vec3::ONE * ASSETS_SCALE_FACTOR),
                texture: asset_server.load("bench.png"),
                ..default()
            });
        });
}

pub fn spawn_cardboard_boxes(mut commands: Commands, asset_server: Res<AssetServer>) {
    for translation in [
        Vec3::new(32. * ASSETS_SCALE_FACTOR, -22. * ASSETS_SCALE_FACTOR, 2.),
        Vec3::new(59. * ASSETS_SCALE_FACTOR, -22. * ASSETS_SCALE_FACTOR, 2.),
        Vec3::new(54. * ASSETS_SCALE_FACTOR, -1. * ASSETS_SCALE_FACTOR, 2.),
    ] {
        commands
            .spawn((
                VisibilityBundle::default(),
                TransformBundle::from_transform(Transform::from_translation(translation)),
                RigidBody::Static,
                Collider::rectangle(28. * ASSETS_SCALE_FACTOR, 22. * ASSETS_SCALE_FACTOR),
                CollisionLayers::new(
                    CustomCollisionLayer::Platform,
                    [CustomCollisionLayer::Bottle, CustomCollisionLayer::Platform],
                ),
                WaypointPlatform,
            ))
            .with_children(|child_builder| {
                child_builder.spawn(SpriteBundle {
                    texture: asset_server.load("cardboard_box.png"),
                    transform: Transform::from_xyz(0., 3. * ASSETS_SCALE_FACTOR, 0.)
                        .with_scale(Vec3::ONE * ASSETS_SCALE_FACTOR),
                    ..default()
                });
            });
    }
}

pub fn spawn_fan(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("fan_sprite.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn((
            VisibilityBundle::default(),
            TransformBundle::from_transform(Transform::from_xyz(
                112. * ASSETS_SCALE_FACTOR,
                16. * ASSETS_SCALE_FACTOR,
                2.,
            )),
            RigidBody::Static,
            Collider::rectangle(32. * ASSETS_SCALE_FACTOR, 32. * ASSETS_SCALE_FACTOR),
            CollisionLayers::new(
                CustomCollisionLayer::Platform,
                [CustomCollisionLayer::Bottle, CustomCollisionLayer::Platform],
            ),
            WaypointPlatform,
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(0., 0., 0.)
                        .with_scale(Vec3::ONE * ASSETS_SCALE_FACTOR),
                    texture: texture_handle,
                    ..default()
                },
                TextureAtlas::from(texture_atlas_handle),
                FanAnimationTimer(Timer::new(Duration::from_millis(80), TimerMode::Repeating)),
            ));
        });
}

pub fn animate_fan(
    mut fan_atlas_query: Query<(&mut TextureAtlas, &mut FanAnimationTimer)>,
    time: Res<Time>,
) {
    for (mut fan_atlas, mut fan_animation_timer) in &mut fan_atlas_query {
        fan_animation_timer.0.tick(time.delta());
        if fan_animation_timer.0.just_finished() {
            fan_atlas.index = (fan_atlas.index + 1) % 4;
        }
    }
}
