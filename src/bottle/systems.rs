use crate::bottle::components::{Bottle, BottleContent, BottleContentJoint};
use crate::grabber::components::{GrabTarget, GrabZone};
use crate::physics::CustomCollisionLayer;
use avian2d::collision::{Collider, CollisionLayers};
use avian2d::prelude::*;
use bevy::prelude::*;

const BOTTLE_BODY_SIZE: Vec2 = Vec2::new(40., 65.);
const BOTTLE_NECK_HEIGHT: f32 = 16.;
const BOTTLE_CAP_SIZE: Vec2 = Vec2::new(14., 10.);
const BOTTLE_DENSITY: f32 = 0.4;
const CONTENT_RADIUS: f32 = 10.;
const CONTENT_DENSITY: f32 = 4.;

pub fn spawn_bottle(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bottle_translation = Vec3::new(-400., -400., 10.);

    let bottle = commands
        .spawn((
            VisibilityBundle::default(),
            TransformBundle::from_transform(Transform::from_translation(bottle_translation)),
            ColliderDensity(BOTTLE_DENSITY),
            RigidBody::Dynamic,
            Collider::rectangle(BOTTLE_BODY_SIZE.x, BOTTLE_BODY_SIZE.y),
            CollisionLayers::new(
                CustomCollisionLayer::Bottle,
                [CustomCollisionLayer::Platform],
            ),
            Bottle,
            GrabTarget,
            GrabZone,
            AngularDamping(0.5),
        ))
        .with_children(|child_builder| {
            // Bottle body
            child_builder.spawn(SpriteBundle {
                texture: asset_server.load("bottle.png"),
                transform: Transform::from_xyz(0., 10., 0.).with_scale(Vec3::ONE * 4.),
                ..default()
            });

            // Bottleneck
            child_builder.spawn((
                TransformBundle::from_transform(Transform::from_xyz(
                    0.,
                    BOTTLE_BODY_SIZE.y / 2.,
                    0.,
                )),
                GrabZone,
                ColliderDensity(BOTTLE_DENSITY),
                Collider::triangle(
                    Vec2::Y * BOTTLE_NECK_HEIGHT,
                    Vec2::new(-BOTTLE_BODY_SIZE.x / 2., 0.),
                    Vec2::new(BOTTLE_BODY_SIZE.x / 2., 0.),
                ),
                CollisionLayers::new(
                    CustomCollisionLayer::Bottle,
                    [CustomCollisionLayer::Platform],
                ),
            ));
            // Bottle cap
            child_builder.spawn((
                TransformBundle::from_transform(Transform::from_xyz(
                    0.,
                    BOTTLE_BODY_SIZE.y / 2. + BOTTLE_NECK_HEIGHT - BOTTLE_CAP_SIZE.y / 2.,
                    0.,
                )),
                GrabZone,
                ColliderDensity(BOTTLE_DENSITY),
                Collider::rectangle(BOTTLE_CAP_SIZE.x, BOTTLE_CAP_SIZE.y),
                CollisionLayers::new(
                    CustomCollisionLayer::Bottle,
                    [CustomCollisionLayer::Platform],
                ),
            ));
        })
        .id();

    for offset_y in [CONTENT_RADIUS, -CONTENT_RADIUS] {
        let bottle_content = commands
            .spawn((
                TransformBundle::from_transform(Transform::from_translation(
                    bottle_translation + Vec3::Y * offset_y,
                )),
                ColliderDensity(CONTENT_DENSITY),
                RigidBody::Dynamic,
                Collider::circle(CONTENT_RADIUS),
                CollisionLayers::new(
                    CustomCollisionLayer::Content,
                    [CustomCollisionLayer::Content],
                ),
                BottleContent,
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
