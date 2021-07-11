// Imports
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

use crate::inventory_item_trait;
use crate::population::Population;

// Import constant from main
use crate::{CAM_H, CAM_W, TILE_SIZE};
use std::str::FromStr;
use std::string::ParseError;

use rand::Rng;

/// Crop type enum
#[derive(Copy, Clone)]
pub enum CropType {
    None,
    Carrot,
    Corn,
    Potato,
    Lettuce,
}

/// Crop struct
pub struct Crop<'a> {
    /// Rectangle to manage crop position.
    pos: Rect,
    /// The stage of growth the crop is in, from
    /// 0 to 3.
    stage: u8,
    /// Rectangle to crop the sprite sheet to the
    /// appropriate tile.
    src: Rect,
    /// Texture of sprite sheet.
    texture: Texture<'a>,
    /// Boolean to hold whether plant has been
    /// watered or not.
    watered: bool,

    tex_path: String,

	t: CropType,

	/// Example to show sorting
	/// I'm not sure how this will be implemented further on
	/// May need to make seperate seed class?
	some_internal_genetic_value: i32
}
// TODO add crop genetics

impl<'a> Crop<'a> {
    /// Creates a new Crop instance.
    ///
    /// Uses the enum `CropType` to select a row on the crop sprite
    /// sheet.
    ///
    /// # Arguments
    /// * `t` - Enum to select type of crop
    /// * `pos` - Position of the crop. Make sure `pos % TILE_SIZE == 0`
    /// * `texture` - Sprite sheet texture
    pub fn new(
        pos: Rect,
        stage: u8,
        texture: Texture<'a>,
        watered: bool,
        tex_path: String,
        t: CropType,
    ) -> Crop {
        let (x, y) = match t {
            CropType::None => (0, 0),
            CropType::Carrot => (stage as u32 * TILE_SIZE, 0),
            CropType::Corn => (stage as u32 * TILE_SIZE, TILE_SIZE),
            CropType::Potato => (stage as u32 * TILE_SIZE, TILE_SIZE * 2),
            CropType::Lettuce => (stage as u32 * TILE_SIZE, TILE_SIZE * 3),
        };

		let src = Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE);

		let mut rng = rand::thread_rng();


		Crop {
			pos,
			stage,
			src,
			texture,
			watered,
			tex_path,
			t,
			some_internal_genetic_value: rng.gen_range(0,100)
		}
	}

    pub fn new_inventory_crop(
        stage: u8,
        texture: Texture<'a>,
        watered: bool,
        tex_path: String,
        t: CropType,
    ) -> Crop {
        let (x, y) = match t {
            CropType::None => (0, 0),
            CropType::Carrot => (stage as u32 * TILE_SIZE, 0),
            CropType::Corn => (stage as u32 * TILE_SIZE, TILE_SIZE),
            CropType::Potato => (stage as u32 * TILE_SIZE, TILE_SIZE * 2),
            CropType::Lettuce => (stage as u32 * TILE_SIZE, TILE_SIZE * 3),
        };

        let src = Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE);

        let mut rng = rand::thread_rng();

        let pos = Rect::new(
                    0,
                    y as i32,
                    TILE_SIZE,
                    TILE_SIZE,
                );

        Crop {
            pos,
            stage,
            src,
            texture,
            watered,
            tex_path,
            t,
            some_internal_genetic_value: rng.gen_range(0,100)
        }
    }



    /// Sets a crop's `watered` variable to `w`
    pub fn set_water(&mut self, w: bool) {
        self.watered = w;
    }

    /// Checks if a crop has been watered, then increments its
    /// stage of growth, clamping to `0..3`
    pub fn grow(&mut self) {
        if self.get_watered() && self.stage != 3 {
            self.stage = (self.stage + 1).clamp(0, 3);
            // Change src from sprite sheet
            self.src.set_x(self.src.x() + (TILE_SIZE as i32));
            // Plant requires more water after growing
            self.watered = false;
        }
    }

    /// Takes ownership of a `WindowCanvas`, checks if the
    /// crop is in frame, and prints it if it is.
    ///
    /// Returns the updated `WindowCanvas`
    ///
    /// # Arguments
    /// * `x` - current x position of camera
    /// * `y` - current y position of camera
    /// * `win` - `WindowCanvas` to be updated
    pub fn print_crop(&self, x: i32, y: i32, mut win: WindowCanvas) -> WindowCanvas {
        let testx = self.get_x() - x;
        let testy = self.get_y() - y;

        if testx > -(self.get_width() as i32)
            && testx < CAM_W as i32
            && testy > -(self.get_height() as i32)
            && testy < CAM_H as i32
        {
            let crop_sub_set = Rect::new(
                self.get_x() - x,
                self.get_y() - y,
                self.get_width(),
                self.get_height(),
            );
            win.copy(self.get_texture(), self.get_src(), crop_sub_set)
                .unwrap();
            return win;
        }
        win
    }

    /// Get a Crop's texture
    pub fn get_texture(&self) -> &Texture {
        &self.texture
    }

    /// Get a Crop's `src`
    pub fn get_src(&self) -> Rect {
        self.src
    }

    /// Get a Crop's position
    pub fn get_pos(&self) -> Rect {
        self.pos
    }

    pub fn set_pos(&mut self, new_pos: Rect) {
        self.pos = new_pos;
    }

    /// Get a Crop's width
    pub fn get_width(&self) -> u32 {
        self.get_pos().width()
    }

    /// Get a Crop's height
    pub fn get_height(&self) -> u32 {
        self.get_pos().height()
    }

    /// Get a Crop's x position
    pub fn get_x(&self) -> i32 {
        self.get_pos().x()
    }

    /// Get a Crop's y position
    pub fn get_y(&self) -> i32 {
        self.get_pos().y()
    }

    /// Get a Crop's watered status
    pub fn get_watered(&self) -> bool {
        self.watered
    }

    pub fn get_tex_path(&self) -> &String {
        &self.tex_path
    }

    pub fn get_stage(&self) -> u8 {
        self.stage
    }

    pub fn set_stage(&mut self, n: u8) {
        self.stage = n;
    }

    pub fn get_crop_type(&self) -> &str {
        match self.t {
            CropType::None => "None",
            CropType::Carrot => "Carrot",
            CropType::Corn => "Corn",
            CropType::Lettuce => "Lettuce",
            CropType::Potato => "Potato",
        }
    }

    pub fn get_crop_type_enum(&self) -> CropType{
        self.t
    }

    pub fn set_crop_type(&mut self, string: &str) {
        match string {
            "None" => self.t = CropType::None,
            "Carrot" => self.t = CropType::Carrot,
            "Corn" => self.t = CropType::Corn,
            "Lettuce" => self.t = CropType::Lettuce,
            "Potato" => self.t = CropType::Potato,
            _ => println!("invalid CropType"),
        };

        let (x, y) = match self.t {
            CropType::None => (0, 0),
            CropType::Carrot => (self.stage as u32 * TILE_SIZE, 0),
            CropType::Corn => (self.stage as u32 * TILE_SIZE, TILE_SIZE),
            CropType::Potato => (self.stage as u32 * TILE_SIZE, TILE_SIZE * 2),
            CropType::Lettuce => (self.stage as u32 * TILE_SIZE, TILE_SIZE * 3),
        };

        self.src = Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE);
    }
    pub fn set_crop_type_enum(&mut self, new_crop_type: CropType){
        self.t = new_crop_type;
        let (x, y) = match self.t {
            CropType::None => (0, 0),
            CropType::Carrot => (self.stage as u32 * TILE_SIZE, 0),
            CropType::Corn => (self.stage as u32 * TILE_SIZE, TILE_SIZE),
            CropType::Potato => (self.stage as u32 * TILE_SIZE, TILE_SIZE * 2),
            CropType::Lettuce => (self.stage as u32 * TILE_SIZE, TILE_SIZE * 3),
        };

        self.src = Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE);
    }
}

impl inventory_item_trait for Crop<'_>{
	/// Sort inventory so that you take the best item from the inventory
	/// This can be a combination of factors
	/// i.e. 2*speed + resistance
	fn get_value(&self) -> i32{
		self.some_internal_genetic_value
	}
    fn texture(&self) -> &Texture{
        &self.texture
    }
    fn pos(&self) -> Rect{
		self.pos
    }    
    fn inventory_input(&self, square:(i32, i32), pop: &mut Population) -> Option<CropType>{
        println!("CROP");
        let (x,y) = square;
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
            let mut _c = pop.get_crop_with_index_mut(x as u32, y as u32);
            _c.set_crop_type_enum(self.t);
            _c.set_stage(0);
            _c.set_water(false);
        }
        return None;
    }
}

impl FromStr for CropType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Carrot" => Ok(CropType::Carrot),
            "Corn" => Ok(CropType::Corn),
            "Lettuce" => Ok(CropType::Lettuce),
            "Potato" => Ok(CropType::Potato),
            _ => Err(()),
        }
    }
}
