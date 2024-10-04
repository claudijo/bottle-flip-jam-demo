use crate::aerobat::components::{Aerobat, RestingTime};
use crate::bottle::components::{Bottle, BottleContent, BottleContentJoint, BottlePart};
use crate::bottle::resources::SpawnPoint;
use crate::bottle::{
    BOTTLE_BODY_SIZE, BOTTLE_CAP_SIZE, BOTTLE_DENSITY, BOTTLE_NECK_HEIGHT, CONTENT_DENSITY,
    CONTENT_RADIUS,
};
use crate::camera::components::FocusTarget;
use crate::config::ASSETS_SCALE_FACTOR;
use crate::free_hand_controller::components::{GrabTarget, GrabZone};
use crate::free_hand_controller::resources::Grabbing;
use crate::physics::CustomCollisionLayer;
use avian2d::collision::{Collider, CollisionLayers};
use avian2d::prelude::*;
use bevy::core::Name;
use bevy::prelude::*;

pub fn set_spawn_point_1(mut spawn_point: ResMut<SpawnPoint>) {
    spawn_point.0 = Vec3::new(-90. * ASSETS_SCALE_FACTOR, -12. * ASSETS_SCALE_FACTOR, 10.);
}

pub fn set_spawn_point_2(mut spawn_point: ResMut<SpawnPoint>) {
    spawn_point.0 = Vec3::new(-30. * ASSETS_SCALE_FACTOR, -2. * ASSETS_SCALE_FACTOR, 10.);
}

pub fn set_spawn_point_3(mut spawn_point: ResMut<SpawnPoint>) {
    spawn_point.0 = Vec3::new(54. * ASSETS_SCALE_FACTOR, 30. * ASSETS_SCALE_FACTOR, 10.);
}

pub fn set_spawn_point_4(mut spawn_point: ResMut<SpawnPoint>) {
    spawn_point.0 = Vec3::new(112. * ASSETS_SCALE_FACTOR, 52. * ASSETS_SCALE_FACTOR, 10.);
}

pub fn spawn_bottle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    spawn_location: Res<SpawnPoint>,
) {
    // Bottle body
    commands
        .spawn((
            VisibilityBundle::default(),
            TransformBundle::from_transform(Transform::from_translation(spawn_location.0)),
            ColliderDensity(BOTTLE_DENSITY),
            RigidBody::Dynamic,
            Collider::rectangle(BOTTLE_BODY_SIZE.x, BOTTLE_BODY_SIZE.y),
            CollisionLayers::new(
                CustomCollisionLayer::Bottle,
                [CustomCollisionLayer::Platform],
            ),
            Bottle,
            GrabTarget,
            FocusTarget,
            BottlePart,
            Aerobat,
            RestingTime(0.),
            Name::new("Bottle body collider"),
            // Will be adjusted in game loop
            AngularDamping::default(),
            LinearDamping::default(),
        ))
        .with_children(|parent| {
            // Grab zone
            parent.spawn((
                TransformBundle::from_transform(Transform::from_xyz(
                    0.,
                    BOTTLE_BODY_SIZE.y / 2. + BOTTLE_NECK_HEIGHT,
                    0.,
                )),
                GrabZone { radius: 24. },
            ));

            parent.spawn(SpriteBundle {
                texture: asset_server.load("images/bottle.png"),
                transform: Transform::from_xyz(0., 2. * ASSETS_SCALE_FACTOR, 0.)
                    .with_scale(Vec3::ONE * ASSETS_SCALE_FACTOR),
                ..default()
            });

            // Bottleneck
            parent.spawn((
                TransformBundle::from_transform(Transform::from_xyz(
                    0.,
                    BOTTLE_BODY_SIZE.y / 2.,
                    0.,
                )),
                ColliderDensity(BOTTLE_DENSITY),
                Collider::triangle(
                    Vec2::Y * BOTTLE_NECK_HEIGHT,
                    Vec2::new(-BOTTLE_BODY_SIZE.x / 2., 0.),
                    Vec2::new(BOTTLE_BODY_SIZE.x / 2., 0.),
                ),
                BottlePart,
                CollisionLayers::new(
                    CustomCollisionLayer::Bottle,
                    [CustomCollisionLayer::Platform],
                ),
                Name::new("Bottle neck collider"),
            ));

            // Bottle cap
            parent.spawn((
                TransformBundle::from_transform(Transform::from_xyz(
                    0.,
                    BOTTLE_BODY_SIZE.y / 2. + BOTTLE_NECK_HEIGHT,
                    0.,
                )),
                ColliderDensity(BOTTLE_DENSITY),
                Collider::rectangle(BOTTLE_CAP_SIZE.x, BOTTLE_CAP_SIZE.y),
                BottlePart,
                CollisionLayers::new(
                    CustomCollisionLayer::Bottle,
                    [CustomCollisionLayer::Platform],
                ),
                Name::new("Bottle cap collider"),
            ));
        });
}

pub fn spawn_bottle_content(
    mut commands: Commands,
    bottle_query: Query<Entity, With<Bottle>>,
    spawn_location: Res<SpawnPoint>,
) {
    for bottle in &bottle_query {
        for offset_y in [
            CONTENT_RADIUS * ASSETS_SCALE_FACTOR,
            -CONTENT_RADIUS * ASSETS_SCALE_FACTOR,
        ] {
            let bottle_content = commands
                .spawn((
                    TransformBundle::from_transform(Transform::from_translation(
                        spawn_location.0 + Vec3::Y * offset_y,
                    )),
                    ColliderDensity(CONTENT_DENSITY),
                    RigidBody::Dynamic,
                    Collider::circle(CONTENT_RADIUS),
                    CollisionLayers::new(
                        CustomCollisionLayer::Content,
                        [CustomCollisionLayer::Content],
                    ),
                    BottleContent,
                    Name::new("Bottle content collider"),
                ))
                .id();

            commands.spawn((
                PrismaticJoint::new(bottle, bottle_content)
                    .with_free_axis(Vec2::Y)
                    .with_limits(
                        -BOTTLE_BODY_SIZE.y / 2. + CONTENT_RADIUS,
                        BOTTLE_BODY_SIZE.y / 2. - CONTENT_RADIUS + BOTTLE_NECK_HEIGHT,
                    ),
                BottleContentJoint,
            ));
        }
    }
}

pub fn despawn_bottle(mut commands: Commands, bottle_query: Query<Entity, With<Bottle>>) {
    for entity in &bottle_query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn despawn_bottle_content(
    mut commands: Commands,
    content_query: Query<Entity, With<BottleContent>>,
    joint_query: Query<Entity, With<BottleContentJoint>>,
) {
    for entity in &content_query {
        commands.entity(entity).despawn_recursive();
    }

    for entity in &joint_query {
        commands.entity(entity).despawn_recursive();
    }
}


pub fn adjust_angular_damping(
    mut bottle_query: Query<(&AngularVelocity, &mut AngularDamping), With<Bottle>>,
) {
    for (angular_velocity, mut angular_damping) in &mut bottle_query {
        angular_damping.0 = (angular_velocity.0 / 10.).powi(2);
    }
}

pub fn adjust_linear_damping(
    mut bottle_query: Query<(&LinearVelocity, &mut LinearDamping), With<Bottle>>,
) {
    for (linear_velocity, mut linear_damping) in &mut bottle_query {
        linear_damping.0 = (linear_velocity.0.length() / 200.).powi(2);
    }
}

pub fn filter_collisions_for_grabbed_bottle(
    mut collisions: ResMut<Collisions>,
    query: Query<(), With<BottlePart>>,
    grabbing: Res<Grabbing>,
) {
    if grabbing.0 {
        collisions.retain(|contacts| {
            !query.contains(contacts.entity1) && !query.contains(contacts.entity2)
        });
    }
}
