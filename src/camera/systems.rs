use crate::camera::components::FocusTarget;
use crate::config::ASSETS_SCALE_FACTOR;
use crate::free_hand_controller::resources::Grabbing;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

const CLOSEUP_CAMERA_SCALE: f32 = 0.75;
const NORMAL_CAMERA_SCALE: f32 = 1.;

const ZOOM_IN_SPEED: f32 = 4.;
const ZOOM_OUT_SPEED: f32 = 2.;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(20. * ASSETS_SCALE_FACTOR, 20. * ASSETS_SCALE_FACTOR, 0.),

        projection: OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scaling_mode: ScalingMode::AutoMin {
                min_width: 640.,
                min_height: 340.,
            },
            scale: NORMAL_CAMERA_SCALE,
            ..default()
        },
        ..default()
    });
}

pub fn aim_camera(
    target_query: Query<&GlobalTransform, With<FocusTarget>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    grabbing: Res<Grabbing>,
    time: Res<Time>,
) {
    if grabbing.0 {
        return;
    }

    for target_transform in &target_query {
        let target_translation =
            target_transform.translation() + Vec3::Y * ASSETS_SCALE_FACTOR * 20.;
        for mut camera_transform in &mut camera_query {
            camera_transform.translation = camera_transform
                .translation
                .lerp(target_translation, 5. * time.delta_seconds());

            if camera_transform.translation.y > target_translation.y {
                camera_transform.translation.y = target_translation.y;
            }
        }
    }
}

pub fn zoom_camera(
    mut camera_query: Query<&mut OrthographicProjection, With<Camera>>,
    grabbing: Res<Grabbing>,
    time: Res<Time>,
) {
    for mut projection in &mut camera_query {
        let (target_scale, zoom_speed) = if grabbing.0 {
            (CLOSEUP_CAMERA_SCALE, ZOOM_IN_SPEED)
        } else {
            (NORMAL_CAMERA_SCALE, ZOOM_OUT_SPEED)
        };

        if (projection.scale - target_scale).abs() > 0.0001 {
            projection.scale = projection
                .scale
                .lerp(target_scale, zoom_speed * time.delta_seconds())
        }
    }
}
