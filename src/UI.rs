use sdl2::render::WindowCanvas;

use super::inventory::Inventory;

/*
trait UI_object{
	fn draw(&self,wincan: &mut WindowCanvas);
}
*/

pub struct UI<'a> {
    inventory: &'a mut Inventory<'a>
}

impl<'a> UI<'a> {
    pub fn new(inventory: &'a mut Inventory<'a>) -> UI<'a> {
        UI {
            inventory
        }
    }

	pub fn draw(&self,wincan: &mut WindowCanvas){ 
		self.inventory.draw(wincan);
	}
}