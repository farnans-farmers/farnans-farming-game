// Imports
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

// Import constant from main
use crate::{CAM_H, CAM_W, TILE_SIZE};
use std::str::FromStr;
use std::string::ParseError;

/// Crop type enum
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
	pub fn new(pos: Rect, stage: u8, texture: Texture<'a>, watered: bool, tex_path: String, t: CropType) -> Crop {
		let (x, y) = match t {
			CropType::None => (0, 0),
			CropType::Carrot => (stage as u32 * TILE_SIZE, 0),
			CropType::Corn => (stage as u32 * TILE_SIZE, TILE_SIZE),
			CropType::Potato => (stage as u32 * TILE_SIZE, TILE_SIZE * 2),
			CropType::Lettuce => (stage as u32 * TILE_SIZE, TILE_SIZE * 3),
		};

		let src = Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE);
		Crop {
			pos,
			stage,
			src,
			texture,
			watered,
			tex_path,
			t,
		}
	}

	/// Sets a crop's `watered` variable to `w`
	pub fn set_water(&mut self, w: bool) {
		self.watered = w;
	}

	/// Checks if a crop has been watered, then increments its
	/// stage of growth, clamping to `0..3`
	pub fn grow(&mut self) {
		if self.getWatered() && self.stage != 3 {
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
		let testx = self.getX() - x;
		let testy = self.getY() - y;

		if testx > -(self.getWidth() as i32)
			&& testx < CAM_W as i32
			&& testy > -(self.getHeight() as i32)
			&& testy < CAM_H as i32
		{
			let crop_sub_set = Rect::new(
				self.getX() - x,
				self.getY() - y,
				self.getWidth(),
				self.getHeight()
			);
			win.copy(self.getTexture(), self.getSrc(), crop_sub_set).unwrap();
			return win;
		}
		win
	}

	/// Get a Crop's texture
	pub fn getTexture(&self) -> &Texture {
		&self.texture
	}

	/// Get a Crop's `src`
	pub fn getSrc(&self) -> Rect {
		self.src
	}

	/// Get a Crop's position
	pub fn getPos(&self) -> Rect {
		self.pos
	}

	/// Get a Crop's width
	pub fn getWidth(&self) -> u32 {
		self.getPos().width()
	}

	/// Get a Crop's height
	pub fn getHeight(&self) -> u32 {
		self.getPos().height()
	}

	/// Get a Crop's x position
	pub fn getX(&self) -> i32 {
		self.getPos().x()
	}

	/// Get a Crop's y position
	pub fn getY(&self) -> i32 {
		self.getPos().y()
	}

	/// Get a Crop's watered status
	pub fn getWatered(&self) -> bool {
		self.watered
	}

	pub fn getTex_path(&self) -> &String { &self.tex_path }

	pub fn getStage(&self) -> u8 { self.stage }

	pub fn setStage(&mut self, n: u8) {
		self.stage = n;
	}

	pub fn GetCropType(&self) -> &str {
		match self.t {
			CropType::None => "None",
			CropType::Carrot => "Carrot",
			CropType::Corn => "Corn",
			CropType::Lettuce => "Lettuce",
			CropType::Potato => "Potato",
		}
	}

	pub fn SetCropType(&mut self, string: &str) {
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
