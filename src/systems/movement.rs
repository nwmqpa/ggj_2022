use bevy::{
    input::Input,
    prelude::{KeyCode, Query, Res, Transform},
};

use crate::components::Position;

fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<(&mut Transform, &mut Position)>,
) {
    for (mut transform, mut position) in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 2.;
        }
    }
}
