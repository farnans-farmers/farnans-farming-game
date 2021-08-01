use crate::crop::CropType;
use crate::genes;
use crate::population::Population;
use crate::InventoryItemTrait;
use sdl2::rect::Rect;
use sdl2::render::Texture;

/// This class is for tool functionality
/// Right now, just have 3 tools

pub enum ToolType {
    Hand,
    Hoe,
    WateringCan,
}

pub struct Tool<'a> {
    src: Rect,
    texture: Texture<'a>,
    current_type: ToolType,
}

impl<'a> Tool<'a> {
    /// Creates a new `Player` instance.
    ///
    /// # Arguments
    /// * `pos` - Position of the player.
    /// * `texture` - Sprite sheet texture
    pub fn new(src: Rect, texture: Texture<'a>, t: ToolType) -> Tool<'a> {
        Tool {
            src,
            texture,
            current_type: t,
        }
    }
}

impl InventoryItemTrait for Tool<'_> {
    fn get_value(&self) -> i32 {
        1
    }
    fn texture(&self) -> &Texture {
        &self.texture
    }
    fn src(&self) -> Rect {
        self.src
    }
    fn to_save_string(&self) -> Option<String> {
        None
    }
    fn inventory_input(
        &self,
        square: (i32, i32),
        pop: &mut Population,
    ) -> Option<(Option<CropType>, Option<genes::Genes>, Option<genes::Genes>)> {
        let (x, y) = square;

        match self.current_type {
            // Hand
            ToolType::Hand => {
                // TODO remove debugging that prints genes
                if let Some(_i) = pop
                    .get_crop_with_index(x as u32, y as u32)
                    .get_gene(crate::genes::GeneType::GrowthRate)
                {
                    println!(
                        "{}",
                        pop.get_crop_with_index(x as u32, y as u32)
                            .get_all_genes()
                            .as_ref()
                            .unwrap()
                    );
                    if let Some(p) = pop
                        .get_crop_with_index(x as u32, y as u32)
                        .get_child()
                        .as_ref()
                    {
                        println!("{}", p);
                    } else {
                        println!("None");
                    }
                }
                // If crop rotten, don't harvest, just remove
                if pop.get_crop_with_index(x as u32, y as u32).rotten() {
                    let mut _c = pop.get_crop_with_index_mut(x as u32, y as u32);
                    _c.set_stage(0);
                    _c.set_rotten(false);
                    _c.set_crop_type_enum(CropType::None);
                    _c.set_water(false);
                    _c.set_genes(None);
                    _c.set_child(None);
                    return None;
                }

                // If tile has plant ready to harvest, harvest
                if pop.get_crop_with_index(x as u32, y as u32).get_stage() == 3 {
                    let _g = pop
                        .get_crop_with_index(x as u32, y as u32)
                        .get_all_genes()
                        .as_ref()
                        .unwrap()
                        .clone();
                    let mut _c = pop.get_crop_with_index_mut(x as u32, y as u32);
                    let return_crop_type = _c.get_crop_type_enum();
                    // let _g = _c.get_all_genes().unwrap().clone();
                    // _c.set_crop_type("None");
                    _c.set_crop_type_enum(CropType::None);
                    _c.set_stage(0);
                    _c.set_water(false);
                    _c.set_genes(None);
                    _c.set_child(None);
                    // let mut _t = pop.get_tile_with_index_mut(x as u32, y as u32);
                    // _t.set_tilled(false);

                    let child = _c.get_child().clone();

                    return Some((Some(return_crop_type), Some(_g), child));
                }
            }
            // Hoe
            ToolType::Hoe => {
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
            ToolType::WateringCan => {
                if !pop.get_crop_with_index(x as u32, y as u32).get_watered() {
                    pop.get_crop_with_index_mut(x as u32, y as u32)
                        .set_water(true);
                }
                if pop.get_tile_with_index(x as u32, y as u32).tilled() {
                    pop.get_tile_with_index_mut(x as u32, y as u32)
                        .set_water(true);
                }
            }
        }
        return None;
    }
}
