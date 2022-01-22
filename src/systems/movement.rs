use bevy::{
    input::Input,
    prelude::{KeyCode, Query, Res, Transform, With, Without}, core::Time,
};

use crate::{components::{Position, Role, Enemy}, utils};

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

pub fn monster_movement(
    time: Res<Time>,
    players: Query<(&Position, &Role, Without<Enemy>)>,
    mut monsters: Query<(&mut Position, With<Enemy>)>
) {
    for (position, role, _) in players.iter() {
        if role.is_assailant() {
            for (mut m_position, _) in monsters.iter_mut() {
                println!("{m_position:?}");
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