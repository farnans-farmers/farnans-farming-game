use sdl2::render::WindowCanvas;

use super::inventory::Inventory;
/*
trait UI_object{
	fn draw(&self,wincan: &mut WindowCanvas);
}
*/

pub struct UI<'a> {
    temp_box: &'a Box<i32>,
    inventory: Inventory<'a>
}

impl<'a> UI<'a> {
    pub fn new(temp_box: &'a Box<i32>) -> UI {
		let inventory = Inventory::new(&temp_box);
        UI {
            temp_box,
            inventory
        }
    }

	pub fn draw(&self,wincan: &mut WindowCanvas){ 
		self.inventory.draw(wincan);
	}

	pub fn set_selected(&mut self,_selected: i32){
        self.inventory.set_selected(_selected);
    }
}