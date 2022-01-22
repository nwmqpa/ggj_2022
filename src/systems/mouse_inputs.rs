use bevy::{
    input::mouse::{MouseButtonInput},
    prelude::{Commands, EventReader, Res, Transform}, window::Windows, math::{Vec2, Vec3}, sprite::SpriteSheetBundle,
};

use crate::{components::Position, animations::AnimationHandles};

fn spawn_monster(commands: &mut Commands, position: Vec2, animation_handles: &Res<AnimationHandles>) {
    commands
        .spawn()
        .insert(Position::new(position.x, position.y))
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: animation_handles.mummy_idle.clone_weak(),
            transform: Transform::from_scale(Vec3::splat(0.2)),
            ..Default::default()
        });
}

pub fn mouse_button_events(
    mut commands: Commands,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    windows: Res<Windows>,
    animation_handles: Res<AnimationHandles>
) {
    use bevy::input::ElementState;
    let window = windows.get_primary().unwrap();

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ElementState::Pressed => {
                if let Some(position) = window.cursor_position() {
                    spawn_monster(&mut commands, position, &animation_handles);
                }
            }
            _ => {}
        }
    }
}
