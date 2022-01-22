use bevy::{
    input::mouse::MouseButtonInput,
    math::{Vec2, Vec3},
    prelude::{Commands, EventReader, Res, Transform},
    sprite::SpriteSheetBundle,
    window::Windows,
};

use crate::{animations::AnimationHandles, components::Position};

fn spawn_monster(
    commands: &mut Commands,
    position: Vec2,
    animation_handles: &Res<AnimationHandles>,
) {
    commands
        .spawn()
        .insert(Position::new(position.x, position.y))
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: animation_handles.mummy_idle.clone_weak(),
            transform: Transform::from_scale(Vec3::splat(0.1))
                .with_translation(Vec3::new(position.x, position.y, 1.0)),
            ..Default::default()
        });
}

pub fn mouse_button_events(
    mut commands: Commands,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    windows: Res<Windows>,
    animation_handles: Option<Res<AnimationHandles>>,
) {
    use bevy::input::ElementState;
    let window = windows.get_primary().unwrap();

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ElementState::Pressed => {
                if let Some(position) = window.cursor_position() {
                    if let Some(animation_handles) = animation_handles.as_ref() {
                        spawn_monster(&mut commands, position, &animation_handles);
                    }
                }
            }
            _ => {}
        }
    }
}
