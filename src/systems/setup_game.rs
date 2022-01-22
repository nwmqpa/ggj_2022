#![allow(dead_code)]
use bevy::{
    math::Vec3,
    prelude::{Assets, Commands, Image, OrthographicCameraBundle, ResMut, Transform},
    sprite::{SpriteSheetBundle, TextureAtlas},
};

use crate::{
    animation::generate_texture_atlas_from_sprites,
    components::{Name, Position, Role},
    AnimationHandles,
};

pub(crate) fn setup_game(
    mut commands: Commands,
    mut animation_handles: ResMut<AnimationHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut texture_atlas_handles = vec![];

    for (name, handles) in animation_handles.sprite_handles.iter() {
        texture_atlas_handles.push((
            name.clone(),
            generate_texture_atlas_from_sprites(
                10,
                handles.clone(),
                &mut textures,
                &mut texture_atlases,
            ),
        ));
    }
    println!("Atlas generated.");

    animation_handles.texture_atlas_handles = texture_atlas_handles.into_iter().collect();

    commands
        .spawn()
        .insert(Position::new(10., 10.))
        .insert(Name("Player 1".to_string()))
        .insert(Role::Defender)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: animation_handles
                .texture_atlas_handles
                .get("idle")
                .unwrap()
                .clone_weak(),
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        });

    commands
        .spawn()
        .insert(Position::new(10., 20.))
        .insert(Name("Player 2".to_string()))
        .insert(Role::Assailant)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: animation_handles
                .texture_atlas_handles
                .get("idle")
                .unwrap()
                .clone_weak(),
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        });

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
