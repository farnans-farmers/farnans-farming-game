use std::time::{Duration, Instant};

// Imports
use sdl2::rect::Rect;
use sdl2::render::Texture;

use crate::anim::Animation;

// Player sprites are 54x90 px.
pub const PLAYER_WIDTH: u32 = 54;
pub const PLAYER_HEIGHT: u32 = 90;
/// PLAYER_EFF_HEIGHT_SKIP is the number of pixels to skip when computing
/// collision.
const PLAYER_EFF_HEIGHT_SKIP: i32 = 10;

/// Player struct
pub struct Player<'a> {
	/// Rectangle to manage player position
	pos: Rect,
	/// Animation spritesheet
	src: Animation<Rect>,
	/// Texture of sprite sheet
	texture: Texture<'a>,
	/// Whether the player is moving
	moving: bool
}

// TODO implement player animation
impl<'a> Player<'a> {
	/// Creates a new `Player` instance. 
	/// 
	/// # Arguments
	/// * `pos` - Position of the player. 
	/// * `texture` - Sprite sheet texture
	pub fn new(pos: Rect, texture: Texture<'a>) -> Player {
		// Derive the number of frames from the size of the texture.
		let sz = texture.query();
		let bounds = Rect::new(0, 0, sz.width, sz.height);
		let dur = Duration::from_secs_f64(1.0/30.0);
		let anim = Animation::from_sheet(&bounds, 0, PLAYER_WIDTH, PLAYER_HEIGHT, dur, Instant::now());
		Player{
			pos,
			src: anim,
			texture,
			moving: false
		}
	}

	/// Get player position `Rect`
	pub fn get_pos(&self) -> Rect {
		let mut pos = self.pos;
		pos.set_y(pos.y + PLAYER_EFF_HEIGHT_SKIP);
		pos.set_height((pos.height() as i32 - PLAYER_EFF_HEIGHT_SKIP) as u32);
		pos
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

	/// Set the player's movement animation status
	pub fn set_moving(&mut self, moving: bool) {
		self.moving = moving;
	}

	/// Get `src` of player
	pub fn src(&mut self) -> Rect {
		// Animate if the player is moving *or* if the animation hasn't looped
		// yet, so that the sprite doesn't jerk downward when stopping.
		if self.moving || self.src.current_index() != 0 {
			*self.src.tick()
		} else {
			self.src.reset(Instant::now());
			*self.src.current()
		}
	}

	/// Get texture of player
	pub fn texture(&self) -> &Texture {
		&self.texture
	}
}
