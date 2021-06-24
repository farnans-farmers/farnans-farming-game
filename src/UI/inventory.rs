use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

pub struct Inventory<'a> {
    wincan: &'a Box<i32>,
    selected: i32,
    squares: Vec<Rect>
}

impl<'a> Inventory<'a> {
    pub fn new(wincan: &'a Box<i32>) -> Inventory {
        let temp_select = 0;
        let squares: Vec<Rect> = (0..10).map
        (|x| Rect::new(295+(x*70), 640, 64, 60)).collect();
        Inventory {
            wincan,
            selected: temp_select,
            squares
        }
    }
    pub fn draw(&self,wincan: &mut WindowCanvas){ 
        wincan.set_draw_color(Color::RGBA(159,82,30,255));
        wincan.fill_rect(Rect::new(290, 635, 705, 70));

        wincan.set_draw_color(Color::RGBA(244,0,0,255));
        wincan.fill_rect(Rect::new(292+(self.selected*70), 637, 70, 66));

        wincan.set_draw_color(Color::RGBA(244,182,110,255));
        wincan.fill_rects(&self.squares[..]).expect("ERROR");
    }
    pub fn set_selected(&mut self,_selected: i32){
        self.selected = _selected
    }
}