use crate::grabber::components::{GrabAnchor, GrabJoint, GrabTarget, GrabZone};
use crate::grabber::events::Released;
use crate::grabber::resources::{GrabTouchId, Grabbing};
use avian2d::prelude::*;
use bevy::input::touch::TouchPhase;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const TOUCH_ANCHOR_OFFSET: f32 = 16.;

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
    grab_zone: &GrabZone,
) -> bool {
    let offset = cursor_position - grab_zone_transform.translation().xy();
    if offset.length() < grab_zone.radius {
        let grabbed_at = grab_target_transform
            .affine()
            .inverse()
            .transform_point(grab_zone_transform.translation());

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

pub fn despawn_grab_anchor(mut commands: Commands, grab_anchor_query: Query<Entity, With<GrabAnchor>>) {
    for grab_anchor in &grab_anchor_query {
        commands.entity(grab_anchor).despawn();
    }
}

pub fn grab_using_mouse(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut anchor_query: Query<(Entity, &mut Transform), With<GrabAnchor>>,
    target_query: Query<(Entity, &GlobalTransform), With<GrabTarget>>,
    grab_zone_query: Query<(&GlobalTransform, &GrabZone)>,
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

        for (anchor, mut anchor_transform) in &mut anchor_query {
            for (bottle, bottle_transform) in &target_query {
                for (grab_zone_transform, grab_zone) in &grab_zone_query {
                    if try_grab_target(
                        &mut commands,
                        anchor,
                        bottle,
                        bottle_transform,
                        cursor_position,
                        grab_zone_transform,
                        grab_zone,
                    ) {
                        anchor_transform.translation = cursor_position.extend(0.);
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
    mut anchor_query: Query<(Entity, &mut Transform), With<GrabAnchor>>,
    target_query: Query<(Entity, &GlobalTransform), With<GrabTarget>>,
    grab_zone_query: Query<(&GlobalTransform, &GrabZone)>,
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

            for (anchor, mut anchor_transform) in &mut anchor_query {
                for (bottle, bottle_transform) in &target_query {
                    for (grab_zone_transform, grab_zone) in &grab_zone_query {
                        if try_grab_target(
                            &mut commands,
                            anchor,
                            bottle,
                            bottle_transform,
                            cursor_position,
                            grab_zone_transform,
                            grab_zone,
                        ) {
                            anchor_transform.translation =
                                cursor_position.extend(0.) - Vec3::Y * TOUCH_ANCHOR_OFFSET;
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
                anchor_transform.translation =
                    cursor_point.extend(0.) - Vec3::Y * TOUCH_ANCHOR_OFFSET;
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
    mut released_event_writer: EventWriter<Released>,
) {
    if buttons.just_released(MouseButton::Left) {
        for joint in &joint_query {
            commands.entity(joint).despawn();
            grabbing.0 = false;
            released_event_writer.send(Released(joint));
        }
    }
}

pub fn release_using_touch(
    mut commands: Commands,
    joint_query: Query<Entity, With<GrabJoint>>,
    mut grab_touch_id: ResMut<GrabTouchId>,
    mut touch_event_reader: EventReader<TouchInput>,
    mut grabbing: ResMut<Grabbing>,
    mut released_event_writer: EventWriter<Released>,
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
                grabbing.0 = false;
                grab_touch_id.0 = None;
                released_event_writer.send(Released(joint));
            }

            return;
        }
    }
}
