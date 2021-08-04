//! Module containing the Player struct and its
//! associated functions

use std::time::{Duration, Instant};

// Imports

use rand::Rng;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::video::WindowContext;

use crate::anim::Animation;
use crate::crop::Crop;
use crate::crop::CropType;
use crate::genes;
use crate::inventory::Inventory;

use crate::population::Population;

// Import constants from main
use crate::{BG_H, BG_W, TILE_SIZE};

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
    /// Texture of tool usage sprite sheet
    tool_texture: Texture<'a>,
    /// Direction the player is facing
    dir: Direction,
    /// Whether the player is moving
    moving: bool,
    /// Player's velocity vector
    velocity: (f32, f32),
    /// Player's inventory
    inventory: Inventory<'a>,
    /// is a tool being used
    tooluse: bool,
}

impl<'a> Player<'a> {
    /// Creates a new `Player` instance.
    ///
    /// # Arguments
    /// * `pos` - Position of the player.
    /// * `texture` - Sprite sheet texture
    pub fn new(
        pos: Rect,
        texture: Texture<'a>,
        tool_texture: Texture<'a>,
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
            tool_texture,
            dir: Direction::Down,
            moving: false,
            velocity: (0.0, 0.0),
            inventory,
            tooluse: false,
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
    #[allow(dead_code)]
    pub fn left(&self) -> i32 {
        self.pos.left()
    }

    /// Get right bound of player
    #[allow(dead_code)]
    pub fn right(&self) -> i32 {
        self.pos.right()
    }

    /// Get top bound of player
    #[allow(dead_code)]
    pub fn top(&self) -> i32 {
        self.pos.top()
    }

    /// Get bottom bound of player
    #[allow(dead_code)]
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

    ///#[allow(dead_code)]
    pub fn get_selected(&self) -> i32 {
        self.inventory.get_selected()
    }

    pub fn use_inventory(
        &mut self,
        square: (i32, i32),
        pop: &mut Population,
    ) -> Option<(Option<CropType>, Option<genes::Genes>, Option<genes::Genes>)> {
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
        if self.tooluse {
            wincan.copy(self.tool_texture(), src, player_cam_pos).unwrap();
            if self.get_selected() == 1 {
                if self.get_dir() == 0 {
                    let src = Rect::new(0 as i32, 0 as i32, PLAYER_HEIGHT, PLAYER_WIDTH);
                }
                if self.get_dir() == 1 {
                    let src = Rect::new(0 as i32, PLAYER_HEIGHT as i32, PLAYER_HEIGHT, PLAYER_WIDTH);
                }
                if self.get_dir() == 2 {
                    let src = Rect::new(0 as i32, 2*PLAYER_HEIGHT as i32, PLAYER_HEIGHT, PLAYER_WIDTH);
                }
                if self.get_dir() == 3 {
                    let src = Rect::new(0 as i32, 3*PLAYER_HEIGHT as i32, PLAYER_HEIGHT, PLAYER_WIDTH);
                }
            }
            else if self.get_selected() == 2 {
                if self.get_dir() == 0 {
                    let src = Rect::new(PLAYER_WIDTH as i32, 0 as i32, PLAYER_HEIGHT, PLAYER_WIDTH);
                }
                if self.get_dir() == 1 {
                    let src = Rect::new(PLAYER_WIDTH as i32, PLAYER_HEIGHT as i32, PLAYER_HEIGHT, PLAYER_WIDTH);
                }
                if self.get_dir() == 2 {
                    let src = Rect::new(PLAYER_WIDTH as i32, 2*PLAYER_HEIGHT as i32, PLAYER_HEIGHT, PLAYER_WIDTH);
                }
                if self.get_dir() == 3 {
                    let src = Rect::new(PLAYER_WIDTH as i32, 3*PLAYER_HEIGHT as i32, PLAYER_HEIGHT, PLAYER_WIDTH);
                }
            }
            else if self.get_selected() == 3 {
                if self.get_dir() == 0 {
                    let src = Rect::new(2*PLAYER_WIDTH as i32, 0 as i32, PLAYER_HEIGHT, PLAYER_WIDTH);
                }
                if self.get_dir() == 1 {
                    let src = Rect::new(2*PLAYER_WIDTH as i32, PLAYER_HEIGHT as i32, PLAYER_HEIGHT, PLAYER_WIDTH);
                }
                if self.get_dir() == 2 {
                    let src = Rect::new(2*PLAYER_WIDTH as i32, 2*PLAYER_HEIGHT as i32, PLAYER_HEIGHT, PLAYER_WIDTH);
                }
                if self.get_dir() == 3 {
                    let src = Rect::new(2*PLAYER_WIDTH as i32, 3*PLAYER_HEIGHT as i32, PLAYER_HEIGHT, PLAYER_WIDTH);
                }
            }
            else {
                if self.get_dir() == 0 {
                    let src = Rect::new(3*PLAYER_WIDTH as i32, 0 as i32, PLAYER_HEIGHT, PLAYER_WIDTH);
                }
                if self.get_dir() == 1 {
                    let src = Rect::new(3*PLAYER_WIDTH as i32, PLAYER_HEIGHT as i32, PLAYER_HEIGHT, PLAYER_WIDTH);
                }
                if self.get_dir() == 2 {
                    let src = Rect::new(3*PLAYER_WIDTH as i32, 2*PLAYER_HEIGHT as i32, PLAYER_HEIGHT, PLAYER_WIDTH);
                }
                if self.get_dir() == 3 {
                    let src = Rect::new(3*PLAYER_WIDTH as i32, 3*PLAYER_HEIGHT as i32, PLAYER_HEIGHT, PLAYER_WIDTH);
                }
            }

        }
        else {
            wincan.copy(self.texture(), src, player_cam_pos).unwrap();
        }
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

    pub fn tooluse(&self) -> bool {
        self.tooluse
    }

    pub fn set_tooluse(&mut self, r: bool) {
        self.tooluse = r;
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
        let x_deltav_f = self.resist(self.velocity.0, vel.0);
        let y_deltav_f = self.resist(self.velocity.1, vel.1);

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
        // Set the player's movement animation status
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

    /// Returns the grid coordinates of the
    /// tile the player is facing
    pub fn get_facing(&self) -> (i32, i32) {
        let offset: (i32, i32) = {
            match self.get_dir() {
                // Down
                0 => (0, 1),
                // Left
                1 => (-1, 0),
                // Right
                2 => (1, 0),
                // Up
                3 => (0, -1),
                // Other (shouldn't happen)
                _ => (0, 0),
            }
        };
        let coordinates = (
            (((self.x() + TILE_SIZE as i32 / 2) / TILE_SIZE as i32) + offset.0)
                .clamp(0, (BG_W / TILE_SIZE) as i32),
            (((self.y() + TILE_SIZE as i32) / TILE_SIZE as i32) + offset.1)
                .clamp(0, (BG_H / TILE_SIZE) as i32),
        );
        coordinates
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

    pub fn tool_texture(&self) -> &Texture {
        &self.tool_texture
    }

    pub fn check_collision(&self, a: &Rect) -> bool {
        let b = self.get_pos();
        !(a.bottom() < b.top()
            || a.top() > b.bottom()
            || a.right() < b.left()
            || a.left() > b.right())
    }

    #[allow(dead_code)]
    pub fn get_inventory(&mut self) -> &mut Inventory<'a> {
        &mut self.inventory
    }

    /// Eat two or three randomly selected crops. Returns the number of
    /// crops that the PC wanted to eat but couldn't.
    pub fn dinner(&mut self) -> i32 {
        let mut rng = rand::thread_rng();
        let mut n = if rng.gen_ratio(1, 3) { 3 } else { 2 };
        // Shuffle a list of the available crop types.
        let mut opts = [
            CropType::Carrot,
            CropType::Corn,
            CropType::Potato,
            CropType::Lettuce,
        ];
        rand::seq::SliceRandom::shuffle(&mut opts[..], &mut rng);
        // The crops are in random order. If we have them, eat them in the
        // same order; otherwise, try the next.
        for kind in opts {
            if n <= 0 {
                return 0;
            }
            if self.inventory.eat(kind) {
                println!("eat a {:?}", kind);
                n -= 1;
            }
        }
        n
    }
}
