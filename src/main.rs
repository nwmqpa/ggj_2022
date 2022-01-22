use bevy::{asset::LoadState, prelude::*};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    PreInit,
    Init,
    Finished,
}

#[derive(Default)]
struct SpriteHandles {
    handles: Vec<HandleUntyped>,
}

fn main() {
    App::new()
        .init_resource::<SpriteHandles>()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::PreInit)
        .add_system_set(SystemSet::on_enter(AppState::PreInit).with_system(load_textures))
        .add_system_set(SystemSet::on_update(AppState::PreInit).with_system(check_textures))
        .add_system_set(SystemSet::on_enter(AppState::Init).with_system(setup))
        .run();
}

fn load_textures(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    todo!()
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        state.set(AppState::Finished).unwrap();
    }
}

fn setup(
    mut commands: Commands,
    sprite_handles: Res<SpriteHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    todo!();
}
