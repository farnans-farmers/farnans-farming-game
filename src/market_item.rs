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
    pub min: i32,
    pub max: i32,
    pub pos: Rect,
    pub crop: CropType,
    pub growth: i32,
}

impl MarketItem {
    pub fn new(
        item_label_offset: i32,
        amount: i32,
        min: i32,
        max: i32,
        pos: Rect,
        crop: CropType,
        growth: i32,
    ) -> MarketItem {
        MarketItem {
            item_label_offset,
            amount,
            min,
            max,
            pos,
            crop,
            growth,
            //texture,
        }
    }
}
