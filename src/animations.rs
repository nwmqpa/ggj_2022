use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
pub struct AnimationHandles {
    #[asset(texture_atlas(
        tile_size_x = 560.,
        tile_size_y = 480.,
        columns = 15,
        rows = 17,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "textures/hero_guy/idle/spritesheet.png")]
    pub hero_guy_idle: Handle<TextureAtlas>,

    #[asset(texture_atlas(
        tile_size_x = 560.,
        tile_size_y = 480.,
        columns = 12,
        rows = 13,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "textures/hero_guy/dying/spritesheet.png")]
    pub hero_guy_dying: Handle<TextureAtlas>,

    #[asset(texture_atlas(
        tile_size_x = 560.,
        tile_size_y = 480.,
        columns = 12,
        rows = 13,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "textures/hero_guy/melee_attack/spritesheet.png")]
    pub hero_guy_melee_attack: Handle<TextureAtlas>,

    #[asset(texture_atlas(
        tile_size_x = 560.,
        tile_size_y = 480.,
        columns = 13,
        rows = 15,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "textures/hero_guy/running/spritesheet.png")]
    pub hero_guy_running: Handle<TextureAtlas>,

    #[asset(texture_atlas(
        tile_size_x = 900.,
        tile_size_y = 900.,
        columns = 12,
        rows = 13,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "textures/mummy/dying/spritesheet.png")]
    pub mummy_dying: Handle<TextureAtlas>,

    #[asset(texture_atlas(
        tile_size_x = 900.,
        tile_size_y = 900.,
        columns = 11,
        rows = 11,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "textures/mummy/hurt/spritesheet.png")]
    pub mummy_hurt: Handle<TextureAtlas>,

    #[asset(texture_atlas(
        tile_size_x = 900.,
        tile_size_y = 900.,
        columns = 11,
        rows = 11,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "textures/mummy/melee_attack/spritesheet.png")]
    pub mummy_melee_attack: Handle<TextureAtlas>,

    #[asset(texture_atlas(
        tile_size_x = 900.,
        tile_size_y = 900.,
        columns = 11,
        rows = 11,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "textures/mummy/run_melee_attack/spritesheet.png")]
    pub mummy_run_melee_attack: Handle<TextureAtlas>,

    #[asset(texture_atlas(
        tile_size_x = 900.,
        tile_size_y = 900.,
        columns = 12,
        rows = 15,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "textures/mummy/idle/spritesheet.png")]
    pub mummy_idle: Handle<TextureAtlas>,
}
