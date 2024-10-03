use crate::aerobat::components::{Aerobat, FlipMeter, Grounded, Resting, RestingTime};
use crate::aerobat::resources::{RestingActivationTime, RestingThreshold};
use crate::bottle::components::{Bottle, BottleContent, BottlePart};
use crate::bottle::{BOTTLE_BODY_SIZE, BOTTLE_CAP_SIZE, BOTTLE_NECK_HEIGHT};
use crate::free_hand_controller::events::Released;
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn add_hit_detector(
    mut commands: Commands,
    bottle_query: Query<Entity, With<Bottle>>,
    bottle_parts_query: Query<Entity, With<BottlePart>>,
    bottle_content_query: Query<Entity, With<BottleContent>>,
) {
    for bottle in &bottle_query {
        let mut shape_caster_exclude_entities = vec![];
        for bottle_part in &bottle_parts_query {
            if bottle_part != bottle {
                shape_caster_exclude_entities.push(bottle_part);
            }
        }
        for bottle_content in &bottle_content_query {
            shape_caster_exclude_entities.push(bottle_content);
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
}

pub fn update_grounded(
    mut commands: Commands,
    aerobat_query: Query<(Entity, &ShapeHits, &Rotation), With<Aerobat>>,
) {
    for (entity, hits, rotation) in &aerobat_query {
        let mut hit_entity = None;
        let is_grounded = hits.iter().any(|hit| {
            let grounded = (rotation * -hit.normal2).angle_between(Vec2::Y).abs() <= 0.01;
            if grounded {
                hit_entity = Some(hit.entity);
            }
            grounded
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded(hit_entity));
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

pub fn update_resting_time(
    mut aerobat_query: Query<(&mut RestingTime, &LinearVelocity, &AngularVelocity), With<Aerobat>>,
    resting_threshold: Res<RestingThreshold>,
    time: Res<Time>,
) {
    for (mut resting_time, linear_velocity, angular_velocity) in &mut aerobat_query {
        if linear_velocity.length() <= resting_threshold.linear
            && angular_velocity.0.abs() <= resting_threshold.angular
        {
            resting_time.0 += time.delta_seconds();
        } else {
            resting_time.0 = 0.;
        }
    }
}

pub fn update_resting(
    mut commands: Commands,
    resting_activation_time: Res<RestingActivationTime>,
    aerobat_query: Query<(Entity, &RestingTime), With<Aerobat>>,
) {
    for (entity, resting_time) in &aerobat_query {
        if resting_time.0 > resting_activation_time.0 {
            commands.entity(entity).insert(Resting);
        } else {
            commands.entity(entity).remove::<Resting>();
        }
    }
}

pub fn insert_flip_meter_on_release(
    mut commands: Commands,
    aerobat_query: Query<Entity, With<Aerobat>>,
    mut release_event_reader: EventReader<Released>,
) {
    for _ in release_event_reader.read() {
        for aerobat in &aerobat_query {
            commands.entity(aerobat).insert(FlipMeter(0.));
        }
    }
}

pub fn track_flip_rotation(
    mut aerobat_query: Query<(&mut FlipMeter, &AngularVelocity), With<Aerobat>>,
    time: Res<Time>,
) {
    for (mut flip_meter, angular_velocity) in &mut aerobat_query {
        let rotation_step = angular_velocity.0 * time.delta_seconds();
        flip_meter.0 += rotation_step;
    }
}
