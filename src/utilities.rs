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
			if pop.getCropWithIndex(x as u32, y as u32).getStage() == 3 {
				// TODO add to inventory
				// Set tile's crop to "None" type to hide it
				let mut _c = pop.getCropWithIndex_mut(x as u32, y as u32);
				_c.SetCropType("None");
				_c.setStage(0);
				_c.set_water(false);
				let mut _t = pop.getTileWithIndex_mut(x as u32, y as u32);
				_t.set_tilled(false);
			}
		}
		// Hoe
		1 => {
			// If tile is empty, set as tilled dirt
			if pop
				.getCropWithIndex(x as u32, y as u32)
				.GetCropType()
				.to_owned() == "None"
			{
				let mut _tile = pop.getTileWithIndex_mut(x as u32, y as u32);
				_tile.set_tilled(true);
			}
		}
		// Watering can
		2 => {
			// If tyle has plant, call water()
			if !pop.getCropWithIndex(x as u32, y as u32).getWatered() {
				pop.getCropWithIndex_mut(x as u32, y as u32).set_water(true);
			}
		}
		// TODO Add seed planting capabilities
		// Seed 1
		3 => {
			// Not sure what order the seeds will be in in the
			// inventory, but planting will look something like this
			if pop.getTileWithIndex(x as u32, y as u32).tilled()
				&& pop
					.getCropWithIndex(x as u32, y as u32)
					.GetCropType()
					.to_owned() == "None"
			{
				// TODO check to see if we have any seeds
				let mut _c = pop.get_croptile_with_index_mut(x as u32, y as u32);
				_c.crop.SetCropType("Carrot");
				_c.crop.setStage(0);
				_c.crop.set_water(false);
			}
		}
		// Seed 2
		4 => {
			if pop.getTileWithIndex(x as u32, y as u32).tilled()
				&& pop
					.getCropWithIndex(x as u32, y as u32)
					.GetCropType()
					.to_owned() == "None"
			{
				// TODO check to see if we have any seeds
				let mut _c = pop.get_croptile_with_index_mut(x as u32, y as u32);
				_c.crop.SetCropType("Corn");
				_c.crop.setStage(0);
				_c.crop.set_water(false);
			}
		}
		// Seed 3
		5 => {
			if pop.getTileWithIndex(x as u32, y as u32).tilled()
				&& pop
					.getCropWithIndex(x as u32, y as u32)
					.GetCropType()
					.to_owned() == "None"
			{
				// TODO check to see if we have any seeds
				let mut _c = pop.get_croptile_with_index_mut(x as u32, y as u32);
				_c.crop.SetCropType("Potato");
				_c.crop.setStage(0);
				_c.crop.set_water(false);
			}
		}
		// Seed 4
		6 => {
			if pop.getTileWithIndex(x as u32, y as u32).tilled()
				&& pop
					.getCropWithIndex(x as u32, y as u32)
					.GetCropType()
					.to_owned() == "None"
			{
				// TODO check to see if we have any seeds
				let mut _c = pop.get_croptile_with_index_mut(x as u32, y as u32);
				_c.crop.SetCropType("Lettuce");
				_c.crop.setStage(0);
				_c.crop.set_water(false);
			}
		}
		// other
		_ => {}
	}
}
