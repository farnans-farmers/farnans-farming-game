use crate::inventory_item_trait;
use sdl2::render::Texture;
use sdl2::rect::Rect;

pub enum tool_type {
    hand,
    hoe,
    watering_can,
}


pub struct Tool<'a> {
    pos: Rect,
    texture: Texture<'a>,
    current_type: tool_type,
}


impl<'a> Tool<'a> {
    /// Creates a new `Player` instance.
    ///
    /// # Arguments
    /// * `pos` - Position of the player.
    /// * `texture` - Sprite sheet texture
    pub fn new(
        pos: Rect,
        texture: Texture<'a>,
        t: tool_type,
    ) -> Tool<'a> {
            Tool{
                pos,
                texture,
                current_type: t,
            }
        }
}


impl inventory_item_trait for Tool<'_>{
    fn get_value(&self) -> i32{
        1
    }
    fn texture(&self) -> &Texture{
        &self.texture
    }
    fn pos(&self) -> Rect {
        self.pos
    }
}