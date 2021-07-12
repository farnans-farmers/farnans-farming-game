//! Module containing the Player struct and its
//! associated functions

use std::time::{Duration, Instant};

// Imports
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::video::WindowContext;

use crate::anim::Animation;
use crate::crop::Crop;
use crate::crop::CropType;
use crate::inventory::Inventory;
use crate::item::Item;
use crate::population::Population;

// Player sprites are 54x90 px.
pub const PLAYER_WIDTH: u32 = 54;
pub const PLAYER_HEIGHT: u32 = 90;
/// PLAYER_EFF_HEIGHT_SKIP is the number of pixels to skip when computing
/// collision.
const PLAYER_EFF_HEIGHT_SKIP: i32 = 10;

const SPEED_LIMIT: f32 = 5.0;
pub const ACCEL_RATE: f32 = 1.0;

/// Sprite directions.
pub enum Direction {
    Down,
    Left,
    Right,
    Up,
}

/// Player struct
pub struct Player<'a> {
    /// Rectangle to manage player position
    pos: Rect,
    /// Animation spritesheet
    src: Vec<Animation<Rect>>,
    /// Texture of sprite sheet
    texture: Texture<'a>,
    /// Direction the player is facing
    dir: Direction,
    /// Whether the player is moving
    moving: bool,
    /// Player's velocity vector
    velocity: (f32, f32),
    /// Player's inventory
    inventory: Inventory<'a>,
}

// TODO implement player animation
impl<'a> Player<'a> {
    /// Creates a new `Player` instance.
    ///
    /// # Arguments
    /// * `pos` - Position of the player.
    /// * `texture` - Sprite sheet texture
    pub fn new(
        pos: Rect,
        texture: Texture<'a>,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Player<'a> {
        // Derive the number of frames from the size of the texture.
        let sz = texture.query();
        let bounds = Rect::new(0, 0, sz.width, sz.height);
        let dur = Duration::from_secs_f64(1.0 / 30.0);
        let mut anims = Vec::with_capacity(4);
        let now = Instant::now();
        for i in 0..4 {
            let anim = Animation::from_sheet(
                &bounds,
                i * PLAYER_HEIGHT as i32,
                PLAYER_WIDTH,
                PLAYER_HEIGHT,
                dur,
                now,
            );
            anims.push(anim);
        }

        let inventory = Inventory::new(texture_creator);

        Player {
            pos,
            src: anims,
            texture,
            dir: Direction::Down,
            moving: false,
            velocity: (0.0, 0.0),
            inventory,
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

    pub fn set_selected(&mut self, _selected: i32) {
        self.inventory.set_selected(_selected);
    }

    pub fn get_selected(&self) -> i32 {
        self.inventory.get_selected()
    }

    pub fn use_inventory(
        &mut self,
        square: (i32, i32),
        mut pop: &mut Population,
    ) -> Option<CropType> {
        self.inventory.use_inventory(square, pop)
        /*match return_crop{
            Some(x) => Some(x),
            None => (),
        }*/
    }
    pub fn add_item(&mut self, new_crop: Crop<'a>) {
        self.inventory.add_item(new_crop);
    }

    pub fn draw(&mut self, wincan: &mut WindowCanvas, player_cam_pos: Rect) {
        self.inventory.draw(wincan);
        let src = self.src();
        wincan.copy(self.texture(), src, player_cam_pos).unwrap();
    }

    /// Set a player's x position, clamping between given bounds
    pub fn update_pos_x(&mut self, vel: (i32, i32), x_bounds: (i32, i32)) {
        self.pos
            .set_x((self.pos.x() + vel.0).clamp(x_bounds.0, x_bounds.1));
    }

    /// Set a player's y position, clamping between given bounds
    pub fn update_pos_y(&mut self, vel: (i32, i32), y_bounds: (i32, i32)) {
        self.pos
            .set_y((self.pos.y() + vel.1).clamp(y_bounds.0, y_bounds.1));
    }

    /// Stop a player from moving in the x direction
    pub fn stay_still_x(&mut self, vel: (i32, i32), x_bounds: (i32, i32)) {
        self.pos
            .set_x((self.pos.x() - vel.0).clamp(x_bounds.0, x_bounds.1));
    }

    /// Stop a player from moving in the y direction
    pub fn stay_still_y(&mut self, vel: (i32, i32), y_bounds: (i32, i32)) {
        self.pos
            .set_y((self.pos.y() - vel.1).clamp(y_bounds.0, y_bounds.1));
    }

    pub fn set_speed(&mut self, vel: (f32, f32)) -> (i32, i32) {
        // Update player velocity
        let mut x_deltav_f = self.resist(self.velocity.0, vel.0);
        let mut y_deltav_f = self.resist(self.velocity.1, vel.1);

        self.velocity.0 = (self.velocity.0 + x_deltav_f as f32).clamp(-SPEED_LIMIT, SPEED_LIMIT);
        self.velocity.1 = (self.velocity.1 + y_deltav_f as f32).clamp(-SPEED_LIMIT, SPEED_LIMIT);

        let speed = (self.velocity.0 * self.velocity.0 + self.velocity.1 * self.velocity.1).sqrt();
        if speed > 0.0 && self.velocity.0 != 0.0 && self.velocity.1 != 0.0 {
            let angle: f32 = (self.velocity.1 / self.velocity.0).abs().atan();
            let kx = angle.cos() * SPEED_LIMIT;
            let ky = angle.sin() * SPEED_LIMIT;

            self.velocity.0 = self.velocity.0.clamp(-kx, kx);
            self.velocity.1 = self.velocity.1.clamp(-ky, ky);
        }
        (self.velocity.0 as i32, self.velocity.1 as i32)
    }

    fn resist(&mut self, vel: f32, deltav: f32) -> f32 {
        if deltav == 0.0 {
            if vel > 1.0 {
                -1.0
            } else if vel < -1.0 {
                1.0
            } else {
                -vel
            }
        } else {
            deltav
        }
    }

    /// Set the player's sprite facing direction.
    pub fn set_direction(&mut self, vel: (i32, i32)) {
        /// Set the player's movement animation status
        if vel.0 == 0 && vel.1 == 0 {
            self.moving = false;
            return;
        }
        self.moving = true;

        // Update player animation status based on movement
        self.dir = if vel.0 > 0 {
            Direction::Right
        } else if vel.0 < 0 {
            Direction::Left
        } else if vel.1 < 0 {
            Direction::Up
        } else {
            Direction::Down
        };
    }

    pub fn get_dir(&self) -> i32 {
        let k = match self.dir {
            Direction::Down => 0,
            Direction::Left => 1,
            Direction::Right => 2,
            Direction::Up => 3,
        };
        k
    }

    /// Get `src` of player
    pub fn src(&mut self) -> Rect {
        let k = match self.dir {
            Direction::Down => 0,
            Direction::Left => 1,
            Direction::Right => 2,
            Direction::Up => 3,
        };
        let src = &mut self.src[k];
        // Animate if the player is moving *or* if the animation hasn't looped
        // yet, so that the sprite doesn't jerk downward when stopping.
        if self.moving || src.current_index() != 0 {
            *src.tick()
        } else {
            src.reset(Instant::now());
            *src.current()
        }
    }

    /// Get texture of player
    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn check_collision(&self, a: &Rect) -> bool {
        let b = self.get_pos();
        !(a.bottom() < b.top()
            || a.top() > b.bottom()
            || a.right() < b.left()
            || a.left() > b.right())
    }

    pub fn get_inventory(&mut self) -> &mut Inventory<'a> {
        &mut self.inventory
    }
}
