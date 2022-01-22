use bevy::{asset::LoadState, ecs::event, prelude::*};

mod events;
mod systems;
use std::collections::HashMap;

use crate::animation::generate_texture_atlas_from_sprites;

mod animation;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    PreInit,
    Init,
    Finished,
}

#[derive(Default)]
struct AnimationHandles {
    sprite_handles: HashMap<String, Vec<HandleUntyped>>,
    texture_atlas_handles: HashMap<String, Handle<TextureAtlas>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
enum AnimationType {
    Repeat,
    Once,
}

fn main() {
    App::new()
        .init_resource::<AnimationHandles>()
        .add_event::<events::EndRoundEvent>()
        .add_event::<events::StartRoundEvent>()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::PreInit)
        .add_system_set(SystemSet::on_enter(AppState::PreInit).with_system(load_textures))
        .add_system_set(SystemSet::on_update(AppState::PreInit).with_system(check_textures))
        .add_system_set(SystemSet::on_enter(AppState::Init).with_system(setup))
        .add_system(animate_sprite_system)
        .run();
}

fn load_textures(mut sprite_handles: ResMut<AnimationHandles>, asset_server: Res<AssetServer>) {
    let animations = vec![
        (
            "dying".to_string(),
            asset_server.load_folder("textures/hero_guy/dying").unwrap(),
        ),
        (
            "idle".to_string(),
            asset_server.load_folder("textures/hero_guy/idle").unwrap(),
        ),
    ];

    sprite_handles.sprite_handles = animations.into_iter().collect();
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    animation_handles: ResMut<AnimationHandles>,
    asset_server: Res<AssetServer>,
) {
    let mut handle_ids = vec![];

    for (_, handles) in animation_handles.sprite_handles.iter() {
        handle_ids.extend(handles);
    }

    if let LoadState::Loaded = asset_server.get_group_load_state(handle_ids.iter().map(|h| h.id)) {
        state.set(AppState::Init).unwrap();
    }
}

fn setup(
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

    animation_handles.texture_atlas_handles = texture_atlas_handles.into_iter().collect();

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: animation_handles
                .texture_atlas_handles
                .get("idle")
                .unwrap()
                .clone_weak(),
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.016, true))
        .insert(AnimationType::Repeat);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &AnimationType,
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (animation_type, mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            match animation_type {
                AnimationType::Repeat => {
                    sprite.index = (sprite.index + 1) % texture_atlas.textures.len()
                }
                AnimationType::Once => {
                    sprite.index = std::cmp::min(sprite.index + 1, texture_atlas.textures.len() - 1)
                }
            }
        }
    }
}
