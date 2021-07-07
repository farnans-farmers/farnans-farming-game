use sdl2::rect::Rect;
use sdl2::render::Texture;

use crate::TILE_SIZE;

pub struct Tile<'a> {
    pos: Rect,
    src: Rect,
    texture: Texture<'a>,
    tilled: bool,
}

impl<'a> Tile<'a> {
    pub fn new(pos: Rect, texture: Texture<'a>) -> Tile {
        let src = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
        Tile {
            pos,
            src,
            texture,
            tilled: false,
        }
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

    pub fn pos(&self) -> Rect {
        self.pos
    }

    pub fn src(&self) -> Rect {
        self.src
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn tilled(&self) -> bool {
        self.tilled
    }

    pub fn set_tilled(&mut self, till: bool) {
        self.tilled = till;
        if till {
            self.src = Rect::new(TILE_SIZE as i32, TILE_SIZE as i32, TILE_SIZE, TILE_SIZE);
        } else {
            self.src = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
        }
    }

    pub fn set_water(&mut self, water: bool) {
        println!("In set_water");
        if self.tilled() {
            if water {
                self.src = Rect::new(2 * TILE_SIZE as i32, TILE_SIZE as i32, TILE_SIZE, TILE_SIZE);
            } else {
                self.src = Rect::new(TILE_SIZE as i32, TILE_SIZE as i32, TILE_SIZE, TILE_SIZE);
            }
        }
    }
}
