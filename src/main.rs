use animations::AnimationHandles;
use bevy::prelude::*;
use bevy_asset_loader::AssetLoader;
use systems::{mouse_inputs::mouse_button_events, movement::monster_movement};

mod animations;
mod components;
mod events;
mod gamestate;
mod systems;
mod utils;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    PreInit,
    Init,
    Finished,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
enum AnimationType {
    Repeat,
    Once,
}

fn main() {
    let mut app = App::new();

    AssetLoader::new(AppState::PreInit)
        .continue_to_state(AppState::Init)
        .with_collection::<AnimationHandles>()
        .build(&mut app);

    app.init_resource::<gamestate::GameData>()
        .add_event::<events::EndRoundEvent>()
        .add_event::<events::StartRoundEvent>()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::PreInit)
        .add_system_set(
            SystemSet::on_enter(AppState::Init).with_system(systems::setup_game::setup_game),
        )
        .add_system(animate_sprite_system)
        .add_system(mouse_button_events)
        .add_system(monster_movement)
        .run();
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
