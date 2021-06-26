// Imports
use sdl2::rect::Rect;
use sdl2::render::Texture;

// Import constant from main
use crate::TILE_SIZE;

/// Player struct
pub struct Player<'a> {
	/// Rectangle to manage player position
	pos: Rect,
	/// Rectangle to crop the sprite sheet to
	/// the appropriate tile
	src: Rect,
	/// Texture of sprite sheet
	texture: Texture<'a>,
}

// TODO implement player animation
impl<'a> Player<'a> {
	/// Creates a new `Player` instance. 
	/// 
	/// # Arguments
	/// * `pos` - Position of the player. 
	/// * `texture` - Sprite sheet texture
	pub fn new(pos: Rect, texture: Texture<'a>) -> Player {
		// src selects which part of the character sheet gets
		// displayed. We only have one sprite on the sheet for
		// now
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
		Player{
			pos,
			src,
			texture,
		}
	}

	/// Get player position `Rect`
	pub fn get_pos(&self) -> Rect {
		self.pos
	}

	/// Get left bound of player
	pub fn left(&self) -> i32 {
		self.pos.left()
	}

	/// Get right bound of player
	pub fn right(&self) -> i32 {
		self.pos.right()
	}

	/// Get top bound of player
	pub fn top(&self) -> i32 {
		self.pos.top()
	}

	/// Get bottom bound of player
	pub fn bottom(&self) -> i32 {
		self.pos.bottom()
	}

	/// Get x position of player
	pub fn x(&self) -> i32 {
		self.pos.x()
	}

	/// Get y position of player
	pub fn y(&self) -> i32 {
		self.pos.y()
	}

	/// Get width of player
	pub fn width(&self) -> u32 {
		self.pos.width()
	}

	/// Get height of player
	pub fn height(&self) -> u32 {
		self.pos.height()
	}

	/// Set a player's x position, clamping between given bounds
	pub fn update_pos_x(&mut self, vel: (i32, i32), x_bounds: (i32, i32)) {
		self.pos.set_x((self.pos.x() + vel.0).clamp(x_bounds.0, x_bounds.1));
	}

	/// Set a player's y position, clamping between given bounds
	pub fn update_pos_y(&mut self, vel: (i32, i32), y_bounds: (i32, i32)) {
		self.pos.set_y((self.pos.y() + vel.1).clamp(y_bounds.0, y_bounds.1));
	}

	/// Stop a player from moving in the x direction
	pub fn stay_still_x(&mut self, vel: (i32, i32), x_bounds: (i32, i32)) {
		self.pos.set_x((self.pos.x() - vel.0).clamp(x_bounds.0, x_bounds.1));
	}

	/// Stop a player from moving in the y direction
	pub fn stay_still_y(&mut self, vel: (i32, i32), y_bounds: (i32, i32)) {
		self.pos.set_y((self.pos.y() - vel.1).clamp(y_bounds.0, y_bounds.1));
	}

	/// Get `src` of player
	pub fn src(&self) -> Rect {
		self.src
	}

	/// Get texture of player
	pub fn texture(&self) -> &Texture {
		&self.texture
	}
}
