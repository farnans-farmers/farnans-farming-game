use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

pub struct Barn<'a> {
    pos: Rect,
    texture: Texture<'a>,
    collision: bool,
}

impl<'a> Barn<'a> {

    pub fn new(pos: Rect, texture: Texture<'a>, collision: bool) -> Barn {
        Barn {
            pos,
            texture,
            collision,
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

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn collision(&self) -> bool {
        self.collision
    }

    // Takes wincan checks if barn needs to be printed and prints it if it does
    // Returns the WindowCanvas
    pub fn printItem(&self, x: i32, y: i32, w: u32, h: u32, mut win: WindowCanvas) -> WindowCanvas {
        let testx = self.x() - x;
        let testy = self.y() - y;
        // Draw barn
        if testx > -(self.width() as i32) && testx < w as i32 &&
            testy > -(self.height() as i32) && testy < h as i32 {
            let barnSubSet = Rect::new(
                self.x() - x,
                self.y() - y,
                self.width(),
                self.height(),
            );
            win.copy(self.texture(), None, barnSubSet);
            return win;
        }
        win
    }

}