use bevy::{asset::LoadState, prelude::*};
use image::{ImageBuffer, RgbaImage};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
enum AnimationType {
    Repeat,
    Once,
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
        asset_server.load_folder("textures/hero_guy/dying").unwrap();
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

fn squareize(items: usize) -> (usize, usize) {
    let mut x = 1;
    let mut y = 1;

    for i in 1..(items + 1) {
        if x * y < i {
            if (x * (y + 1)) >= i && (x * (y + 1)) < ((x + 1) * (x + 1)) {
                y += 1;
            } else {
                x += 1;
                y = x;
            }
        }
    }
    (x, y)
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
        let num_steps = 10;

        let mut texture_atlas_builder =
            TextureAtlasBuilder::default().max_size(Vec2::new(8192., 16384.));

        let mut i = 0;

        let key_frames = sprite_handles.hero_guy_idle_handles.len();
        let first_handle = sprite_handles.hero_guy_idle_handles.first().unwrap();
        let first_texture = textures.get(first_handle).unwrap();

        let sprite_width = first_texture.texture_descriptor.size.width;
        let sprite_height = first_texture.texture_descriptor.size.height;

        let (x, y) = squareize(key_frames);



        for handles in sprite_handles.hero_guy_idle_handles.windows(2) {
            let first_texture = textures.get(&handles[0]).unwrap();
            let last_texture = textures.get(&handles[1]).unwrap();

            let texture_descriptor = first_texture.texture_descriptor.clone();
            let sampler_descriptor = first_texture.sampler_descriptor.clone();

            let first_image = RgbaImage::from_raw(
                first_texture.texture_descriptor.size.width,
                first_texture.texture_descriptor.size.height,
                first_texture.data.clone(),
            )
            .unwrap();

            let last_image = RgbaImage::from_raw(
                last_texture.texture_descriptor.size.width,
                last_texture.texture_descriptor.size.height,
                last_texture.data.clone(),
            )
            .unwrap();

            let stepping = 1.0 / num_steps as f32;
            for step in 0..=num_steps {
                let position = stepping * step as f32;

                let buffer = ImageBuffer::from_fn(
                    texture_descriptor.size.width,
                    texture_descriptor.size.height,
                    |x, y| {
                        imageproc::pixelops::interpolate(
                            *first_image.get_pixel(x, y),
                            *last_image.get_pixel(x, y),
                            position,
                        )
                    },
                );

                let data = buffer.to_vec();

                image::save_buffer(
                    format!("./outputs/image_{:04}.png", i),
                    &data,
                    buffer.width(),
                    buffer.height(),
                    image::ColorType::Rgba8,
                )
                .unwrap();

                let texture_handle = textures.add(Image {
                    data,
                    texture_descriptor: texture_descriptor.clone(),
                    sampler_descriptor: sampler_descriptor.clone(),
                });
                let texture = textures.get(&texture_handle).unwrap();

                texture_atlas_builder.add_texture(texture_handle.clone_weak(), texture);
                i += 1;
            }
        }
        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        let texture_atlas_texture = texture_atlas.texture.clone();

        commands.spawn_bundle(SpriteBundle {
            texture: texture_atlas_texture,
            transform: Transform::from_xyz(-300.0, 0.0, 0.0).with_scale(Vec3::splat(0.1)),
            ..Default::default()
        });
        let atlas_handle = texture_atlases.add(texture_atlas);
        texture_atlas_handles.hero_guy_idle_handle = atlas_handle;
    }

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handles.hero_guy_idle_handle.clone(),
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert(AnimationType::Once);

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
