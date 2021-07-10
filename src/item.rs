use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

use crate::inventory_item_trait;
use rand::Rng;

pub struct Item<'a> {
    pos: Rect,
    texture: Texture<'a>,
    tex_path: String,
    collision: bool,
    some_internal_genetic_value: i32,
}

impl<'a> Item<'a> {
    pub fn new(pos: Rect, texture: Texture<'a>, tex_path: String, collision: bool) -> Item {

        let mut rng = rand::thread_rng();


        Item {
            pos,
            texture,
            tex_path,
            collision,
            some_internal_genetic_value: rng.gen_range(0,100)
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

    pub fn tex_path(&self) -> &String {
        &self.tex_path
    }

    /*
        Takes ownership of a WindowCanvas, checks if the item needs to be printed, and prints it if it does.
        Inputs:
            x: current x position of the camera
            y: current y position of the camera
            w: width of the camera
            h: height of the camera
            win: WindowCanvas to be updated
        Return:
            The updated WindowCanvas
    */
    pub fn print_item(
        &self,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        mut win: WindowCanvas,
    ) -> WindowCanvas {
        let testx = self.x() - x;
        let testy = self.y() - y;
        // Draw barn
        if testx > -(self.width() as i32)
            && testx < w as i32
            && testy > -(self.height() as i32)
            && testy < h as i32
        {
            let barn_sub_set = Rect::new(self.x() - x, self.y() - y, self.width(), self.height());
            win.copy(self.texture(), None, barn_sub_set).unwrap();
            return win;
        }
        win
    }

    pub fn check_for_collision(&self, x: i32, y: i32, w: i32, h: i32) -> bool {
        true
    }
}

impl inventory_item_trait for Item<'_>{
    fn get_value(&self) -> i32{
        self.some_internal_genetic_value
    }
    fn texture(&self) -> &Texture{
        &self.texture
    }
    fn pos(&self) -> Rect {
        self.pos
    }
}