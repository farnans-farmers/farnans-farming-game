//! Module to hold miscellaneous funtions that don't belong
//! to a particular struct
use crate::population;
/// Perform an interaction on a given tile based on
/// the currently equipped tool
pub fn use_tool(x: i32, y: i32, mut pop: &mut population::Population, tool: i32) {
	// TODO edit args to take a tile once that struct is made
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

			}
		}
		// Hoe
		1 => {

			// If tile is empty, set as tilled dirt
			if pop
				.get_crop_with_index(x as u32, y as u32)
				.get_crop_type()
				.to_owned() == "None"
			{
				let mut _tile = pop.get_tile_with_index_mut(x as u32, y as u32);
				_tile.set_tilled(true);
			}

		}
		// Watering can
		2 => {
			// If tyle has plant, call water()
			if !pop.get_crop_with_index(x as u32, y as u32).get_watered() {
				pop.get_crop_with_index_mut(x as u32, y as u32).set_water(true);
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
					.to_owned() == "None"
			{
				// TODO check to see if we have any seeds
				let mut _c = pop.get_crop_with_index_mut(x as u32, y as u32);
				_c.set_crop_type("Carrot");
				_c.set_stage(0);
				_c.set_water(false);
			}
		}
		// Seed 2
		4 => {
			if pop.get_tile_with_index(x as u32, y as u32).tilled()
				&& pop
					.get_crop_with_index(x as u32, y as u32)
					.get_crop_type()
					.to_owned() == "None"
			{
				// TODO check to see if we have any seeds
				let mut _c = pop.get_crop_with_index_mut(x as u32, y as u32);
				_c.set_crop_type("Corn");
				_c.set_stage(0);
				_c.set_water(false);
			}
		}
		// Seed 3
		5 => {
			if pop.get_tile_with_index(x as u32, y as u32).tilled()
				&& pop
					.get_crop_with_index(x as u32, y as u32)
					.get_crop_type()
					.to_owned() == "None"
			{
				// TODO check to see if we have any seeds
				let mut _c = pop.get_crop_with_index_mut(x as u32, y as u32);
				_c.set_crop_type("Potato");
				_c.set_stage(0);
				_c.set_water(false);
			}
		}
		// Seed 4
		6 => {
			if pop.get_tile_with_index(x as u32, y as u32).tilled()
				&& pop
					.get_crop_with_index(x as u32, y as u32)
					.get_crop_type()
					.to_owned() == "None"
			{
				// TODO check to see if we have any seeds
				let mut _c = pop.get_crop_with_index_mut(x as u32, y as u32);
				_c.set_crop_type("Lettuce");
				_c.set_stage(0);
				_c.set_water(false);
			}
		}
		// other
		_ => {}
	}
}
