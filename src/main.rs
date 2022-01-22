use bevy::{asset::LoadState, ecs::event, prelude::*};
use systems::setup_game::setup_game;

mod events;
mod gamestate;
mod systems;

use std::collections::HashMap;

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
        .init_resource::<gamestate::GameData>()
        .add_event::<events::EndRoundEvent>()
        .add_event::<events::StartRoundEvent>()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::PreInit)
        .add_system_set(SystemSet::on_enter(AppState::PreInit).with_system(load_textures))
        .add_system_set(SystemSet::on_update(AppState::PreInit).with_system(check_textures))
        .add_system_set(SystemSet::on_enter(AppState::Init).with_system(systems::setup_game::setup_game))
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
