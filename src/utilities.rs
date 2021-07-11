//! Module to hold miscellaneous funtions that don't belong
//! to a particular struct
use crate::population;
use crate::player;
use crate::crop;

/// Perform an interaction on a given tile based on
/// the currently equipped tool
pub fn use_tool<'a>(x: i32, y: i32, mut pop: &mut population::Population<'a>, tool: i32, player: &mut player::Player<'a>) {
    //println!("x: {}, y: {}", x, y);
    match tool {
        // Hand
        0 => {
            // If tile has plant ready to harvest, harvest
            if pop.get_crop_with_index(x as u32, y as u32).get_stage() == 3 {
                // TODO add to inventory
                // Set tile's crop to "None" type to hide it

                let mut _c = pop.get_crop_with_index_mut(x as u32, y as u32);
                _c.set_crop_type("None");
                _c.set_stage(0);
                _c.set_water(false);
                let mut _t = pop.get_tile_with_index_mut(x as u32, y as u32);
                _t.set_tilled(false);
            }
        }
        // Hoe
        1 => {
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
        2 => {
            // println!("Used watering can");
            // If tyle has plant, call water()
            if !pop.get_crop_with_index(x as u32, y as u32).get_watered() {
                pop.get_crop_with_index_mut(x as u32, y as u32)
                    .set_water(true);
                pop.get_tile_with_index_mut(x as u32, y as u32)
                    .set_water(true);
            }
        }
        // TODO Add seed planting capabilities
        // Seed 1
        3 => {
            // Not sure what order the seeds will be in in the
            // inventory, but planting will look something like this
            if pop.get_tile_with_index(x as u32, y as u32).tilled()
                && pop
                    .get_crop_with_index(x as u32, y as u32)
                    .get_crop_type()
                    .to_owned()
                    == "None"
            {
                // TODO check to see if we have any seeds
                let crop = player.get_inventory().get_carrot_seed(0);
                if let Some(c) = crop {
                    pop.set_crop_with_index(x as u32, y as u32, c);
                }
            }
        }
        // Seed 2
        4 => {
            if pop.get_tile_with_index(x as u32, y as u32).tilled()
                && pop
                    .get_crop_with_index(x as u32, y as u32)
                    .get_crop_type()
                    .to_owned()
                    == "None"
            {
                // TODO check to see if we have any seeds
                let crop = player.get_inventory().get_corn_seed(0);
                if let Some(c) = crop {
                    pop.set_crop_with_index(x as u32, y as u32, c);
                }
            }
        }
        // Seed 3
        5 => {
            if pop.get_tile_with_index(x as u32, y as u32).tilled()
                && pop
                    .get_crop_with_index(x as u32, y as u32)
                    .get_crop_type()
                    .to_owned()
                    == "None"
            {
                // TODO check to see if we have any seeds
                let crop = player.get_inventory().get_potato_seed(0);
                if let Some(c) = crop {
                    pop.set_crop_with_index(x as u32, y as u32, c);
                }
            }
        }
        // Seed 4
        6 => {
            if pop.get_tile_with_index(x as u32, y as u32).tilled()
                && pop
                    .get_crop_with_index(x as u32, y as u32)
                    .get_crop_type()
                    .to_owned()
                    == "None"
            {
                // TODO check to see if we have any seeds
                let crop = player.get_inventory().get_lettuce_seed(0);
                if let Some(c) = crop {
                    pop.set_crop_with_index(x as u32, y as u32, c);
                }
            }
        }
        // other
        _ => {}
    }
}
