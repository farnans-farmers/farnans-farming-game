use sdl2::rect::Rect;
use sdl2::render::Texture;

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

    pub fn printItem(&self, x: int, y: int) {
        let testx = self.x() - x;
        let testy = self.y() - y;
        // Draw barn
        if testx > -(self.width() as i32) && testx < (CAM_W as i32) &&
            testy > -(self.height() as i32) && testy < (CAM_W as i32) {
            let barnSubSet = Rect::new(
                self.x() - x,
                self.y() - y,
                self.width(),
                self.height(),
            );
            wincan.copy(self.texture(), None, barnSubSet);
        }
    }

}