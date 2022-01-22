use bevy::{
    core::Time,
    input::Input,
    math::Vec3,
    prelude::{KeyCode, Query, Res, Transform, Without, With},
};

use crate::{components::{PlayerMovement, Position, Role, Enemy}, utils};

pub fn keyboard_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_movement: Query<&mut PlayerMovement>,
) {
    for mut mvt in player_movement.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Left) {
            mvt.left = -1.0;
        } else if keyboard_input.just_released(KeyCode::Left) {
            mvt.left = 0.0;
        }

        if keyboard_input.just_pressed(KeyCode::Right) {
            mvt.right = 1.0;
        } else if keyboard_input.just_released(KeyCode::Right) {
            mvt.right = 0.0;
        }

        if keyboard_input.just_pressed(KeyCode::Up) {
            mvt.up = 1.0;
        } else if keyboard_input.just_released(KeyCode::Up) {
            mvt.up = 0.0;
        }

        if keyboard_input.just_pressed(KeyCode::Down) {
            mvt.down = -1.0;
        } else if keyboard_input.just_released(KeyCode::Down) {
            mvt.down = 0.0;
        }
    }
}

pub fn sync_transform_position_system(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in query.iter_mut() {
        *transform = transform.with_translation(Vec3::new(position.x, position.y, 0.0));
    }
}

pub fn move_system(time: Res<Time>, mut query: Query<(&mut Position, &PlayerMovement, &Role)>) {
    let delta = time.delta();

    for (mut position, player_movement, role) in query.iter_mut() {
        if let Role::Defender = role {
            position.x += (player_movement.left + player_movement.right)
                * delta.as_secs_f32()
                * player_movement.scale;
            position.y += (player_movement.up + player_movement.down)
                * delta.as_secs_f32()
                * player_movement.scale;
        }
    }
}

pub fn monster_movement(
    time: Res<Time>,
    players: Query<(&Position, &Role, Without<Enemy>)>,
    mut monsters: Query<(&mut Position, With<Enemy>)>
) {
    for (position, role, _) in players.iter() {
        if role.is_assailant() {
            for (mut m_position, _) in monsters.iter_mut() {
                let x_dir = position.x - m_position.x;
                let y_dir = position.y - m_position.y;
                let magn = (x_dir.powf(2.) + y_dir.powf(2.)).sqrt();
                let x_dir = x_dir / magn;
                let y_dir = y_dir / magn;

                *m_position = Position {
                    x: m_position.x + x_dir * time.delta_seconds() * utils::MONSTER_VELOCITY,
                    y: m_position.y + y_dir * time.delta_seconds() * utils::MONSTER_VELOCITY, 
                }
            }
        }
    }
}