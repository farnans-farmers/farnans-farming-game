use crate::inventory_item_trait;
use crate::population::Population;
use crate::crop::CropType;
use sdl2::render::Texture;
use sdl2::rect::Rect;


/// This class is for tool functionality
/// Right now, just have 3 tools

pub enum tool_type {
    hand,
    hoe,
    watering_can,
}

pub struct Tool<'a> {
    src: Rect,
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
        src: Rect,
        texture: Texture<'a>,
        t: tool_type,
    ) -> Tool<'a> {
            Tool{
                src,
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
    fn src(&self) -> Rect {
        self.src
    }
    fn inventory_input(&self, square:(i32, i32), pop: &mut Population) -> Option<CropType>{
        let (x,y) = square;

        match self.current_type {
            // Hand
            tool_type::hand => {
                // If tile has plant ready to harvest, harvest
                if pop.get_crop_with_index(x as u32, y as u32).get_stage() == 3 {
                    let mut _c = pop.get_crop_with_index_mut(x as u32, y as u32);
                    let return_crop_type = _c.get_crop_type_enum();
                    _c.set_crop_type("None");
                    _c.set_stage(0);
                    _c.set_water(false);
                    let mut _t = pop.get_tile_with_index_mut(x as u32, y as u32);
                    _t.set_tilled(false);

                    // I couldn't get cloning to work so I'm passing back the type
                    // TODO send back clone of crop or some other datastructure with genetic info
                    return Some(return_crop_type)
                }
            }
            // Hoe
            tool_type::hoe => {
                // If tile is empty, set as tilled dirt
                if pop
                    .get_crop_with_index(x as u32, y as u32)
                    .get_crop_type()
                    .to_owned()
                    == "None"
                    && !pop.get_tile_with_index(x as u32, y as u32).tilled()
                {
                    let mut _tile = pop.get_tile_with_index_mut(x as u32, y as u32);
                    _tile.set_tilled(true);
                }
            }
            // Watering can
            tool_type::watering_can => {
                if !pop.get_crop_with_index(x as u32, y as u32).get_watered() {
                    pop.get_crop_with_index_mut(x as u32, y as u32)
                        .set_water(true);
                    pop.get_tile_with_index_mut(x as u32, y as u32)
                        .set_water(true);
                }
            }
        }
        return None
    }
}