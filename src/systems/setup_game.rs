#![allow(dead_code)]
use bevy::{
    math::Vec3,
    prelude::{Assets, Commands, Component, Image, ResMut, Transform, Res},
    sprite::{SpriteSheetBundle, TextureAtlas},
};

use crate::{animation::generate_texture_atlas_from_sprites, AnimationHandles};

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

impl Position {
    fn new(x: f32, y: f32) -> Self {
        Position { x, y }
    }
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
enum Role {
    Defender,
    Assailant,
}

pub(crate) fn setup_game(
    mut commands: Commands,
    animation_handles: Res<AnimationHandles>,
) {
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
}
