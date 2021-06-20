use sdl2::rect::Rect;
use sdl2::render::Texture;

pub struct Barn<'a> {
    pos: Rect,
    texture: Texture<'a>,
}

impl<'a> Barn<'a> {

    pub fn new(pos: Rect, texture: Texture<'a>) -> Barn {
        Barn {
            pos,
            texture,
        }
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

    pub fn getPos(&self) -> Rect {
		self.pos
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

}
