use bevy::{asset::LoadState, prelude::*};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    PreInit,
    Init,
    Finished,
}

#[derive(Default)]
struct SpriteHandles {
    hero_guy_idle_handles: Vec<HandleUntyped>,
}

#[derive(Default)]
struct TextureAtlasHandles {
    hero_guy_idle_handle: Handle<TextureAtlas>,
}

fn main() {
    App::new()
        .init_resource::<SpriteHandles>()
        .init_resource::<TextureAtlasHandles>()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::PreInit)
        .add_system_set(SystemSet::on_enter(AppState::PreInit).with_system(load_textures))
        .add_system_set(SystemSet::on_update(AppState::PreInit).with_system(check_textures))
        .add_system_set(SystemSet::on_enter(AppState::Init).with_system(setup))
        .add_system(animate_sprite_system)
        .run();
}

fn load_textures(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    sprite_handles.hero_guy_idle_handles =
        asset_server.load_folder("textures/hero_guy/idle").unwrap();
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    let handle_ids = []
        .iter()
        .chain(&sprite_handles.hero_guy_idle_handles)
        .map(|handle| handle.id);

    if let LoadState::Loaded = asset_server.get_group_load_state(handle_ids) {
        state.set(AppState::Init).unwrap();
    }
}

fn setup(
    mut commands: Commands,
    sprite_handles: Res<SpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    mut texture_atlas_handles: ResMut<TextureAtlasHandles>,
) {
    // HeroGuy Idle
    {
        let mut texture_atlas_builder =
            TextureAtlasBuilder::default().max_size(Vec2::new(8192., 8192.));
        for handle in sprite_handles.hero_guy_idle_handles.iter() {
            let texture = textures.get(handle).unwrap();
            texture_atlas_builder.add_texture(handle.clone_weak().typed::<Image>(), texture);
        }
        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        let atlas_handle = texture_atlases.add(texture_atlas);
        texture_atlas_handles.hero_guy_idle_handle = atlas_handle;
    }

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handles.hero_guy_idle_handle.clone(),
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.2, true));

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
