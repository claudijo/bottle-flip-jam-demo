use crate::bottle::components::{Bottle, BottleContent, BottleContentJoint, BottlePart};
use crate::camera::components::FocusTarget;
use crate::config::ASSETS_SCALE_FACTOR;
use crate::grabber::components::{GrabTarget, GrabZone};
use crate::physics::CustomCollisionLayer;
use avian2d::collision::{Collider, CollisionLayers};
use avian2d::prelude::*;
use bevy::prelude::*;

const BOTTLE_BODY_SIZE: Vec2 = Vec2::new(8. * ASSETS_SCALE_FACTOR, 15. * ASSETS_SCALE_FACTOR);
const BOTTLE_NECK_HEIGHT: f32 = 3. * ASSETS_SCALE_FACTOR;
const BOTTLE_CAP_SIZE: Vec2 = Vec2::new(4. * ASSETS_SCALE_FACTOR, 2. * ASSETS_SCALE_FACTOR);
const BOTTLE_DENSITY: f32 = 0.1;
const CONTENT_RADIUS: f32 = 3. * ASSETS_SCALE_FACTOR;
const CONTENT_DENSITY: f32 = 1.;

pub fn spawn_bottle(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bottle_translation = Vec3::new(-70. * ASSETS_SCALE_FACTOR, -16. * ASSETS_SCALE_FACTOR, 10.);

    let mut shape_caster_exclude_entities = vec![];

    // Bottle body
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
            FocusTarget,
            BottlePart,
            AngularDamping(1.),
            LinearDamping(0.5),
        ))
        .with_children(|parent| {
            // Grab zone
            parent.spawn((
                TransformBundle::from_transform(Transform::from_xyz(
                    0.,
                    BOTTLE_BODY_SIZE.y / 2. + BOTTLE_NECK_HEIGHT,
                    0.,
                )),
                GrabZone { radius: 16. },
            ));

            parent.spawn(SpriteBundle {
                texture: asset_server.load("bottle.png"),
                transform: Transform::from_xyz(0., 2. * ASSETS_SCALE_FACTOR, 0.)
                    .with_scale(Vec3::ONE * ASSETS_SCALE_FACTOR),
                ..default()
            });

            // Bottleneck
            let bottle_neck = parent
                .spawn((
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
                ))
                .id();

            // Bottle cap
            let bottle_cap = parent
                .spawn((
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
                ))
                .id();

            shape_caster_exclude_entities.push(bottle_neck);
            shape_caster_exclude_entities.push(bottle_cap);
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

        shape_caster_exclude_entities.push(bottle_content);

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

    // For detecting if bottle is grounded. See
    // https://github.com/Jondolf/avian/blob/main/crates/avian2d/examples/dynamic_character_2d/plugin.rs
    commands.entity(bottle).insert(
        ShapeCaster::new(
            Collider::rectangle(
                BOTTLE_BODY_SIZE.x,
                BOTTLE_BODY_SIZE.y + BOTTLE_NECK_HEIGHT + BOTTLE_CAP_SIZE.y / 2.,
            ),
            Vec2::Y * (BOTTLE_NECK_HEIGHT + BOTTLE_CAP_SIZE.y / 2.) / 2.,
            0.,
            Dir2::NEG_Y,
        )
        .with_query_filter(SpatialQueryFilter::from_excluded_entities(
            shape_caster_exclude_entities,
        ))
        .with_max_time_of_impact(2.),
    );
}
