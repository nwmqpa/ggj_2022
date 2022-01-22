#![allow(dead_code)]
use std::time::Duration;

use bevy::{
    core::Timer, math::Vec3, prelude::*, render::render_resource::Texture,
    sprite::SpriteSheetBundle,
};

use crate::{
    animations::AnimationHandles,
    components::{Name, PlayerMovement, Position, Role},
    AnimationType,
};

pub(crate) fn setup_game(
    mut commands: Commands,
    animation_handles: Res<AnimationHandles>,
    server: Res<AssetServer>,
) {
    // spawn_map(&mut commands, server);

    commands
        .spawn()
        .insert(Position::new(-20., 0.))
        .insert(Name("Player 1".to_string()))
        .insert(Role::Defender)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: animation_handles.hero_guy_idle.clone_weak(),
            transform: Transform::from_scale(Vec3::splat(0.1))
                .with_translation(Vec3::new(-20., 0., 0.)),
            ..Default::default()
        })
        .insert(Timer::new(Duration::from_millis(16), false))
        .insert(PlayerMovement::default())
        .insert(AnimationType::Repeat);

    commands
        .spawn()
        .insert(Position::new(20., 0.))
        .insert(Name("Player 2".to_string()))
        .insert(Role::Assailant)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: animation_handles.hero_guy_idle.clone_weak(),
            transform: Transform::from_scale(Vec3::splat(0.1))
                .with_translation(Vec3::new(20., 0., 0.)),
            ..Default::default()
        })
        .insert(Timer::new(Duration::from_millis(16), false))
        .insert(PlayerMovement::default())
        .insert(AnimationType::Repeat);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_map(commands: &mut Commands, server: Res<AssetServer>) {
    let ground_handle = server.load("textures/terrain/ground.png");

    commands.spawn().insert_bundle(SpriteBundle {
        texture: ground_handle.clone_weak(),
        transform: Transform::from_scale(Vec3::splat(1.0)).with_translation(Vec3::new(0., 0., 0.0)),
        ..Default::default()
    });
    commands.insert_resource(ground_handle);
}
