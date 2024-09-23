use crate::grabber::components::{GrabAnchor, GrabJoint, GrabTarget, GrabZone};
use crate::grabber::resources::{GrabTouchId, Grabbing};
use avian2d::collision::Collider;
use avian2d::prelude::*;
use bevy::input::touch::TouchPhase;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn world_from_viewport(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    position: Option<Vec2>,
) -> Option<Vec2> {
    let viewport_position = position?;

    // Calculate a world position based on the cursor's position.
    camera.viewport_to_world_2d(camera_transform, viewport_position)
}

fn try_grab_target(
    commands: &mut Commands,
    anchor: Entity,
    grab_target: Entity,
    grab_target_transform: &GlobalTransform,
    cursor_position: Vec2,
    grab_zone_transform: &GlobalTransform,
    grab_zone: &Collider,
) -> bool {
    let (_scale, rotation, translation) = grab_zone_transform.to_scale_rotation_translation();
    if grab_zone.contains_point(translation.xy(), rotation, cursor_position) {
        let grabbed_at = grab_target_transform
            .affine()
            .inverse()
            .transform_point(cursor_position.extend(0.));

        commands.spawn((
            RevoluteJoint::new(anchor, grab_target)
                .with_local_anchor_2(grabbed_at.xy())
                .with_angular_velocity_damping(20.),
            GrabJoint,
        ));

        return true;
    }

    false
}

pub fn spawn_grab_anchor(mut commands: Commands) {
    commands.spawn((
        TransformBundle::from_transform(Transform::from_xyz(0., 0., 0.)),
        RigidBody::Kinematic,
        GrabAnchor,
    ));
}

pub fn grab_using_mouse(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    anchor_query: Query<Entity, With<GrabAnchor>>,
    target_query: Query<(Entity, &GlobalTransform), With<GrabTarget>>,
    grab_zone_query: Query<(&GlobalTransform, &Collider), With<GrabZone>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut grabbing: ResMut<Grabbing>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera_query.single();
        let window = windows.single();
        let Some(cursor_position) =
            world_from_viewport(camera, camera_transform, window.cursor_position())
        else {
            return;
        };

        for anchor in &anchor_query {
            for (bottle, bottle_transform) in &target_query {
                for (grab_zone_transform, collider) in &grab_zone_query {
                    if try_grab_target(
                        &mut commands,
                        anchor,
                        bottle,
                        bottle_transform,
                        cursor_position,
                        grab_zone_transform,
                        collider,
                    ) {
                        grabbing.0 = true;
                        return;
                    }
                }
            }
        }
    }
}

pub fn grab_using_touch(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    anchor_query: Query<Entity, With<GrabAnchor>>,
    target_query: Query<(Entity, &GlobalTransform), With<GrabTarget>>,
    grab_zone_query: Query<(&GlobalTransform, &Collider), With<GrabZone>>,
    touches: Res<Touches>,
    mut grab_touch_id: ResMut<GrabTouchId>,
    mut grabbing: ResMut<Grabbing>,
) {
    if grab_touch_id.0.is_some() {
        return;
    }

    for touch in touches.iter() {
        if touches.just_pressed(touch.id()) {
            let (camera, camera_transform) = camera_query.single();
            let Some(cursor_position) =
                world_from_viewport(camera, camera_transform, Some(touch.position()))
            else {
                return;
            };

            for anchor in &anchor_query {
                for (bottle, bottle_transform) in &target_query {
                    for (grab_zone_transform, collider) in &grab_zone_query {
                        if try_grab_target(
                            &mut commands,
                            anchor,
                            bottle,
                            bottle_transform,
                            cursor_position,
                            grab_zone_transform,
                            collider,
                        ) {
                            grabbing.0 = true;
                            grab_touch_id.0 = Some(touch.id());
                            return;
                        }
                    }
                }
            }
        }
    }
}

pub fn drag_using_mouse(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut anchor_query: Query<&mut Transform, With<GrabAnchor>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = windows.single();
    let Some(cursor_point) =
        world_from_viewport(camera, camera_transform, window.cursor_position())
    else {
        return;
    };

    for mut anchor_transform in &mut anchor_query {
        anchor_transform.translation = cursor_point.extend(0.);
    }
}

pub fn drag_using_touch(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut anchor_query: Query<&mut Transform, With<GrabAnchor>>,
    grab_touch_id: Res<GrabTouchId>,
    mut touch_event_reader: EventReader<TouchInput>,
) {
    if grab_touch_id.0.is_none() {
        return;
    }

    for touch_event in touch_event_reader.read() {
        if TouchPhase::Moved == touch_event.phase {
            if grab_touch_id.0 != Some(touch_event.id) {
                continue;
            }

            let (camera, camera_transform) = camera_query.single();
            let Some(cursor_point) =
                world_from_viewport(camera, camera_transform, Some(touch_event.position))
            else {
                return;
            };

            for mut anchor_transform in &mut anchor_query {
                anchor_transform.translation = cursor_point.extend(0.);
            }

            return;
        }
    }
}

pub fn release_using_mouse(
    mut commands: Commands,
    joint_query: Query<Entity, With<GrabJoint>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut grabbing: ResMut<Grabbing>,
) {
    if buttons.just_released(MouseButton::Left) {
        for joint in &joint_query {
            commands.entity(joint).despawn();
            grabbing.0 = false;
        }
    }
}

pub fn release_using_touch(
    mut commands: Commands,
    joint_query: Query<Entity, With<GrabJoint>>,
    mut grab_touch_id: ResMut<GrabTouchId>,
    mut touch_event_reader: EventReader<TouchInput>,
    mut grabbing: ResMut<Grabbing>,
) {
    if grab_touch_id.0.is_none() {
        return;
    }

    for touch_event in touch_event_reader.read() {
        if touch_event.phase == TouchPhase::Ended || touch_event.phase == TouchPhase::Canceled {
            if grab_touch_id.0 != Some(touch_event.id) {
                continue;
            }

            for joint in &joint_query {
                commands.entity(joint).despawn();
            }

            grabbing.0 = false;
            grab_touch_id.0 = None;
            return;
        }
    }
}
