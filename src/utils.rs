use bevy::{math::{Vec3, Vec2}, window::Window, prelude::Transform};

pub fn window_to_world(
    position: Vec2,
    window: &Window,
    camera: &Transform,
) -> Vec3 {
    let norm = Vec3::new(
        position.x - window.width() / 2.,
        position.y - window.height() / 2.,
        0.,
    );

    *camera * norm
}