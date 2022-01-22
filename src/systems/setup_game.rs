#![allow(dead_code)]
use std::time::Duration;

use bevy::{core::Timer, math::Vec3, prelude::*, sprite::SpriteSheetBundle};

use crate::{
    animations::AnimationHandles,
    components::{Name, Position, Role},
    AnimationType,
};

pub(crate) fn setup_game(mut commands: Commands, animation_handles: Res<AnimationHandles>) {
    commands
        .spawn()
        .insert(Position::new(10., 10.))
        .insert(Name("Player 1".to_string()))
        .insert(Role::Defender)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: animation_handles.hero_guy_idle.clone_weak(),
            transform: Transform::from_scale(Vec3::splat(0.1))
                .with_translation(Vec3::new(-20., 0., 0.)),
            ..Default::default()
        })
        .insert(Timer::new(Duration::from_millis(16), false))
        .insert(AnimationType::Repeat);

    commands
        .spawn()
        .insert(Position::new(10., 20.))
        .insert(Name("Player 2".to_string()))
        .insert(Role::Assailant)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: animation_handles.hero_guy_idle.clone_weak(),
            transform: Transform::from_scale(Vec3::splat(0.1))
                .with_translation(Vec3::new(20., 0., 0.)),
            ..Default::default()
        })
        .insert(Timer::new(Duration::from_millis(16), false))
        .insert(AnimationType::Repeat);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
