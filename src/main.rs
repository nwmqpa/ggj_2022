use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

mod components;
mod events;
mod gamestate;
mod systems;

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

#[derive(AssetCollection)]
struct AnimationHandles {
    #[asset(texture_atlas(
        tile_size_x = 560.,
        tile_size_y = 480.,
        columns = 15,
        rows = 17,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "textures/hero_guy/idle/spritesheet.png")]
    hero_guy_idle: Handle<TextureAtlas>,

    #[asset(texture_atlas(
        tile_size_x = 560.,
        tile_size_y = 480.,
        columns = 12,
        rows = 13,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "textures/hero_guy/dying/spritesheet.png")]
    hero_guy_dying: Handle<TextureAtlas>,

    
    #[asset(texture_atlas(
        tile_size_x = 560.,
        tile_size_y = 480.,
        columns = 12,
        rows = 13,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "textures/hero_guy/melee_attack/spritesheet.png")]
    hero_guy_melee_attack: Handle<TextureAtlas>,
    
    #[asset(texture_atlas(
        tile_size_x = 560.,
        tile_size_y = 480.,
        columns = 13,
        rows = 15,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "textures/hero_guy/running/spritesheet.png")]
    hero_guy_running: Handle<TextureAtlas>,
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
