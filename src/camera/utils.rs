use bevy::prelude::*;

pub fn world_from_viewport(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    position: Option<Vec2>,
) -> Option<Vec2> {
    let viewport_position = position?;

    // Calculate a world position based on the cursor's position.
    camera.viewport_to_world_2d(camera_transform, viewport_position)
}
