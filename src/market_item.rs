use crate::crop::CropType;

use sdl2::rect::Rect;

/*OFFSETS FOR REFERENCE*/
// SEED 1 - 0
// SEED 2 -
// SEED 3 -
// SEED 4 -

pub struct MarketItem {
    pub item_label_offset: i32,
    pub amount: i32,
    pub price: i32,
    pub pos: Rect,
    pub crop: CropType,
    // texture: Texture,
}

impl MarketItem {
    pub fn new(
        item_label_offset: i32,
        amount: i32,
        price: i32,
        pos: Rect,
        crop: CropType, /*, texture: Texture */
    ) -> MarketItem {
        MarketItem {
            item_label_offset,
            amount,
            price,
            pos,
            crop,
            //texture,
        }
    }
}
