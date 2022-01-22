use std::path::Path;

use image::{ImageBuffer, Rgba};


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

fn generate_spritesheet(
    num_steps: u32,
    sprites: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>,
) -> (ImageBuffer<Rgba<u8>, Vec<u8>>, u32, u32, u32, u32) {
    let first_sprite = sprites.first().unwrap();

    let sprite_width = first_sprite.width();
    let sprite_height = first_sprite.height();

    let images = sprites
        .windows(2)
        .map(|sprite_window| {
            let first_sprite = &sprite_window[0];
            let last_sprite = &sprite_window[1];

            let stepping = 1.0 / num_steps as f32;
            (0..=num_steps)
                .into_iter()
                .map(|step| {
                    let position = stepping * step as f32;

                    let buffer = ImageBuffer::from_fn(sprite_width, sprite_height, |x, y| {
                        imageproc::pixelops::interpolate(
                            *last_sprite.get_pixel(x, y),
                            *first_sprite.get_pixel(x, y),
                            position,
                        )
                    });
                    buffer
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    let (x, y) = squareize(images.len());

    let mut image = ImageBuffer::new(x as u32 * sprite_width, y as u32 * sprite_height);

    for (n, img) in images.iter().enumerate() {
        let i = n as u32 % x;
        let j = (n as u32 - i) / x;
        image::imageops::overlay(&mut image, img, i * sprite_width, j * sprite_height);
    }

    (image, x, y, sprite_width, sprite_height)
}



fn main() {
    let num_steps = std::env::args().nth(1).expect("Cannot get first argument").parse::<u32>().expect("Cannot convert first argument to u32");

    for folder in std::env::args().skip(2) {
        let images = std::fs::read_dir(&folder)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|e| e.path().is_file())
            .filter_map(|entry| image::open(entry.path()).ok())
            .map(|image| image.to_rgba8())
            .collect::<Vec<_>>();

        let (spritesheet, x, y, width, height) = generate_spritesheet(num_steps, images);

        let dest_path = Path::new(&folder).join(format!("spritesheet.png"));
        spritesheet.save(&dest_path).unwrap();
        println!("Generated {:?}", dest_path);
    }
}