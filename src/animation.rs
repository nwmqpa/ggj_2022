use bevy::{
    asset::HandleId,
    math::Vec2,
    prelude::{Assets, Handle, Image, ResMut},
    render::render_resource::{Extent3d, TextureDescriptor},
    sprite::{TextureAtlas},
};
use image::{ImageBuffer, Rgba, RgbaImage};

fn squareize(items: usize) -> (u32, u32) {
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
    (x as u32, y as u32)
}

fn get_image_from_handle<H: Into<HandleId>>(
    handle: H,
    textures: &mut ResMut<Assets<Image>>,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let texture = textures.get(handle).unwrap();
    let sprite_width = texture.texture_descriptor.size.width;
    let sprite_height = texture.texture_descriptor.size.height;

    RgbaImage::from_raw(sprite_width, sprite_height, texture.data.clone()).unwrap()
}

fn generate_spritesheet(
    num_steps: u32,
    sprites: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>,
) -> (ImageBuffer<Rgba<u8>, Vec<u8>>, usize, usize) {
    let first_sprite = sprites.first().unwrap();

    let sprite_width = first_sprite.width();
    let sprite_height = first_sprite.height();

    let mut images = vec![];

    for sprite_window in sprites.windows(2) {
        let first_sprite = &sprite_window[0];
        let last_sprite = &sprite_window[1];

        let stepping = 1.0 / num_steps as f32;
        for step in 0..=num_steps {
            let position = stepping * step as f32;

            let buffer = ImageBuffer::from_fn(sprite_width, sprite_height, |x, y| {
                imageproc::pixelops::interpolate(
                    *last_sprite.get_pixel(x, y),
                    *first_sprite.get_pixel(x, y),
                    position,
                )
            });
            images.push(buffer);
        }
    }

    let (x, y) = squareize(images.len());

    let mut image = ImageBuffer::new(x as u32 * sprite_width, y as u32 * sprite_height);

    for (n, img) in images.iter().enumerate() {
        let i = n as u32 % x;
        let j = (n as u32 - i) / x;
        image::imageops::overlay(&mut image, img, i * sprite_width, j * sprite_height);
    }

    (image, x as usize, y as usize)
}

pub fn generate_texture_atlas_from_sprites<H: Into<HandleId> + Clone>(
    num_steps: u32,
    handles: Vec<H>,
    textures: &mut ResMut<Assets<Image>>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Handle<TextureAtlas> {
    let sample_texture = textures.get(handles[0].clone()).unwrap();
    let t_descriptor = sample_texture.texture_descriptor.clone();
    let s_descriptor = sample_texture.sampler_descriptor.clone();
    let images = handles
        .into_iter()
        .map(|h| get_image_from_handle(h, textures))
        .collect::<Vec<_>>();
    let sprite_width = t_descriptor.size.width;
    let sprite_height = t_descriptor.size.height;

    let (spritesheet, x, y) = generate_spritesheet(num_steps, images);

    let data = spritesheet.to_vec();

    let bevy_image = Image {
        data,
        texture_descriptor: TextureDescriptor {
            size: Extent3d {
                width: spritesheet.width(),
                height: spritesheet.height(),
                depth_or_array_layers: 1,
            },
            ..t_descriptor
        },
        sampler_descriptor: s_descriptor,
    };

    let texture_handle = textures.add(bevy_image);

    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(sprite_width as f32, sprite_height as f32),
        x,
        y,
    );
    texture_atlases.add(texture_atlas)
}
