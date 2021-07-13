use crate::crop::CropType;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;

/*OFFSETS FOR REFERENCE*/
// SEED 1 - 0
// SEED 2 -
// SEED 3 -
// SEED 4 -

pub struct Market_item {
    pub item_label_offset: i32,
    pub amount: i32,
    pub price: i32,
    pub pos: Rect,
    pub crop: CropType,
    // texture: Texture,
}

impl Market_item {
    pub fn new(
        item_label_offset: i32,
        amount: i32,
        price: i32,
        pos: Rect,
        crop: CropType, /*, texture: Texture */
    ) -> Market_item {
        Market_item {
            item_label_offset,
            amount,
            price,
            pos,
            crop,
            //texture,
        }
    }
}
