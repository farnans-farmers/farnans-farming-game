use sdl2::rect::Rect;
use sdl2::render::Texture;

pub struct Tile<'a> {
    pos: Rect,
    texture: Texture<'a>,
}

impl<'a> Tile<'a> {
    pub fn new(pos: Rect, texture: Texture<'a>) -> Tile {
        Tile {
            pos,
            texture,
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

}

/*pub struct item<'a> {
    pos: Rect,
    texture: Texture<'a>,
    printed: bool,
}

pub struct itemTile<'a> {
    xPos: int,
    yPos: int,
    itemPntr: item<'a>,
}

pub struct itemMap<'a> {
    itemTileMap: Vec<itemTile<'a>>,
    items: Vec<T>,
}

impl<'a> itemMap<'a> {
    pub fn contains(&self, x: int, y: int) -> bool {
        for i in 0 ..itemTileMap.len() {
            if itemTileMap[i].xPos == x && itemTileMap[i].yPos == y {
                return true;
            }
        }
        false
    }


}*/