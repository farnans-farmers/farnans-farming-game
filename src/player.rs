// Imports
use sdl2::rect::Rect;
use sdl2::render::Texture;

// Import constant from main
use crate::TILE_SIZE;

/**
 * Player struct
 */
pub struct Player<'a> {
	pos: Rect,
	src: Rect,
	texture: Texture<'a>,
}

impl<'a> Player<'a> {
	pub fn new(pos: Rect, texture: Texture<'a>) -> Player {
		// src selects which part of the character sheet gets
=======
		// displayed. We only have one sprite on the sheet for
		// now
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
		Player{
			pos,
			src,
			texture,
		}
	}

	pub fn getPos(&self) -> Rect {
		self.pos
	}


	pub fn left(&self) -> i32 {
		self.pos.left()
	}

	pub fn right(&self) -> i32 {
		self.pos.right()
	}

	pub fn top(&self) -> i32 {
		self.pos.top()
	}

	pub fn bottom(&self) -> i32 {
		self.pos.bottom()
	}

	pub fn x(&self) -> i32 {
		self.pos.x()
	}

	pub fn y(&self) -> i32 {
		self.pos.y()
	}

	pub fn width(&self) -> u32 {
		self.pos.width()
	}

	pub fn height(&self) -> u32 {
		self.pos.height()
	}

	pub fn update_pos_x(&mut self, vel: (i32, i32), x_bounds: (i32, i32)) {
		self.pos.set_x((self.pos.x() + vel.0).clamp(x_bounds.0, x_bounds.1));
	}

	pub fn update_pos_y(&mut self, vel: (i32, i32), y_bounds: (i32, i32)) {
		self.pos.set_y((self.pos.y() + vel.1).clamp(y_bounds.0, y_bounds.1));
	}

	pub fn stay_still_x(&mut self, vel: (i32, i32), x_bounds: (i32, i32)) {
		self.pos.set_x((self.pos.x() - vel.0).clamp(x_bounds.0, x_bounds.1));
	}

	pub fn stay_still_y(&mut self, vel: (i32, i32), y_bounds: (i32, i32)) {
		self.pos.set_y((self.pos.y() - vel.1).clamp(y_bounds.0, y_bounds.1));
	}

	pub fn src(&self) -> Rect {
		self.src
	}

	pub fn texture(&self) -> &Texture {
		&self.texture
	}
}
