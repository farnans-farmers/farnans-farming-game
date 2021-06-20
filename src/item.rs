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
        let testx = barn.x() - cur_bg.x();
        let testy = barn.x() - cur_bg.y();
        // Draw barn
        if testx > -(barn.width() as i32) && testx < (CAM_W as i32) &&
            testy > -(barn.height() as i32) && testy < (CAM_W as i32) {
            let barnSubSet = Rect::new(
                barn.x() - cur_bg.x(),
                barn.y() - cur_bg.y(),
                barn.width(),
                barn.height(),
            );
            wincan.copy(barn.texture(), None, barnSubSet);
        }
    }

}