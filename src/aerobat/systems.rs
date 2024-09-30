use crate::aerobat::components::{Aerobat, FlipMeter, Grounded, Resting, RestingTime};
use crate::aerobat::resources::{RestingActivationTime, RestingThreshold};
use crate::grabber::events::Released;
use avian2d::prelude::*;
use bevy::prelude::*;

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
            && angular_velocity.0 <= resting_threshold.angular
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
    aerobat_query: Query<(Entity, &Transform), With<Aerobat>>,
    mut release_event_reader: EventReader<Released>,
) {
    for _ in release_event_reader.read() {
        for (aerobat, transform) in &aerobat_query {
            commands.entity(aerobat).insert(FlipMeter {
                start_rotation: transform.rotation,
            });
        }
    }
}
