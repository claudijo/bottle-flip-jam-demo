use crate::bottle::components::Bottle;
use crate::camera::utils::world_from_viewport;
use crate::level_editor::components::Movable;
use avian2d::collision::Collider;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn adjust_platform(
    mut platform_query: Query<&mut Transform, With<Movable>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mut transform in &mut platform_query {
        if keys.pressed(KeyCode::KeyW) {
            transform.translation.y += 3.;
            println!("platform translation {:?}", transform.translation);
        }
        if keys.pressed(KeyCode::KeyS) {
            transform.translation.y -= 3.;
            println!("platform translation {:?}", transform.translation);
        }
        if keys.pressed(KeyCode::KeyA) {
            transform.translation.x -= 3.;
            println!("platform translation {:?}", transform.translation);
        }
        if keys.pressed(KeyCode::KeyD) {
            transform.translation.x += 3.;
            println!("platform translation {:?}", transform.translation);
        }

        // Step half pixel
        if keys.just_pressed(KeyCode::ArrowUp) {
            transform.translation.y += 1.5;
            println!("platform translation {:?}", transform.translation);
        }
        if keys.just_pressed(KeyCode::ArrowDown) {
            transform.translation.y -= 1.5;
            println!("platform translation {:?}", transform.translation);
        }
        if keys.just_pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= 1.5;
            println!("platform translation {:?}", transform.translation);
        }
        if keys.just_pressed(KeyCode::ArrowRight) {
            transform.translation.x += 1.5;
            println!("platform translation {:?}", transform.translation);
        }
    }
}

pub fn pan_camera_with_mouse_wheel(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    for ev in evr_scroll.read() {
        for mut camera_transform in &mut camera_query {
            if ev.unit == MouseScrollUnit::Pixel {
                camera_transform.translation.y += ev.y;
                camera_transform.translation.x -= ev.x;
            }
        }
    }
}

pub fn pick_editable(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    movable_query: Query<Entity, With<Movable>>,
    collider_query: Query<(Entity, &Transform, &Collider, Option<&Name>), With<Bottle>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for entity in &movable_query {
            commands.entity(entity).remove::<Movable>();
        }

        let (camera, camera_transform) = camera_query.single();
        let window = windows.single();
        let Some(cursor_position) =
            world_from_viewport(camera, camera_transform, window.cursor_position())
        else {
            return;
        };

        for (entity, transfporm, collider, name) in &collider_query {
            if collider.contains_point(
                transfporm.translation.xy(),
                transfporm.rotation,
                cursor_position,
            ) {
                if let Some(name) = name {
                    info!("Pick {:?} ({:?})", entity, name);
                } else {
                    info!("Pick {:?}", entity);
                }
                commands.entity(entity).insert(Movable);
            }
        }
    }
}
