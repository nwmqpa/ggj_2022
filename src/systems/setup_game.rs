#![allow(dead_code)]
use bevy::{
    math::Vec3,
    prelude::{OrthographicCameraBundle, Transform, Commands, Res},
    sprite::SpriteSheetBundle,
};

use crate::{
    components::{Name, Position, Role},
    AnimationHandles,
};

pub(crate) fn setup_game(mut commands: Commands, animation_handles: Res<AnimationHandles>) {
    commands
        .spawn()
        .insert(Position::new(10., 10.))
        .insert(Name("Player 1".to_string()))
        .insert(Role::Defender)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: animation_handles.hero_guy_idle.clone_weak(),
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        });

    commands
        .spawn()
        .insert(Position::new(10., 20.))
        .insert(Name("Player 2".to_string()))
        .insert(Role::Assailant)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: animation_handles.hero_guy_idle.clone_weak(),
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        });

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
