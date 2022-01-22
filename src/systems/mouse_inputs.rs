use std::time::Duration;

use bevy::{
    input::mouse::MouseButtonInput,
    math::{Vec2, Vec3},
    prelude::{Commands, EventReader, Res, Transform, Query, With, Camera},
    sprite::SpriteSheetBundle,
    window::Windows, core::Timer,
};

use crate::{animations::AnimationHandles, components::{Position, Enemy, Health}, utils, AnimationType};



fn spawn_monster(
    commands: &mut Commands,
    position: Vec3,
    animation_handles: &Res<AnimationHandles>,
) {
    commands
        .spawn()
        .insert(Position::new(position.x, position.y))
        .insert(Enemy)
        .insert(AnimationType::Repeat)
        .insert(Timer::new(Duration::from_millis(16), false))
        .insert(Health::default())
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: animation_handles.mummy_idle.clone_weak(),
            transform: Transform::from_scale(Vec3::splat(0.07))
                .with_translation(position),
            ..Default::default()
        });
}

pub fn mouse_button_events(
    mut commands: Commands,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    windows: Res<Windows>,
    animation_handles: Option<Res<AnimationHandles>>,
    camera: Query<&Transform, With<Camera>>
) {
    use bevy::input::ElementState;
    let window = windows.get_primary().unwrap();

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ElementState::Pressed => {
                if let Some(position) = window.cursor_position() {
                    if let Some(animation_handles) = animation_handles.as_ref() {
                        let camera = camera.single();

                        let position = utils::window_to_world(
                            position,
                            window,
                            camera
                        );

                        spawn_monster(&mut commands, position, &animation_handles);
                    }
                }
            }
            _ => {}
        }
    }
}
