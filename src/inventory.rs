use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::item::Item;
use crate::crop::Crop;
use crate::inventory_item_trait;

use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;

use sdl2::render::TextureQuery;

static INVENTORY_X_POS: i32 = 295;
static INVENTORY_Y_POS: i32 = 640;

static ITEM_BOX_SIZE: i32 = 64;
static BORDER_SIZE: i32 = 4;
static SELECTED_SIZE: i32 = 2;

struct Inventory_Item<'a>{
    item: Vec<Box<dyn inventory_item_trait + 'a>>,
    is_tool: bool
}

impl<'a> Inventory_Item<'a>{
    pub fn new(is_tool: bool) -> Inventory_Item<'a>{
        Inventory_Item{
            item: Vec::new(),
            is_tool
        }
    }

    pub fn get_len(&self) -> i32{
        self.item.len() as i32
    }
    pub fn add_item(&mut self,item: Item<'a>){
        self.item.push( Box::new(item));
    }
    pub fn pop_item(&self){
        println!("TODO");
    }
    pub fn get_item(&self, index: i32) -> &Box<dyn inventory_item_trait + 'a>{
        &(self.item[index as usize])
    }
}

pub struct Inventory<'a> {
    inventory_slots: Vec<Inventory_Item<'a>>,
    selected: i32,
    squares: Vec<Rect>,
    carrot_seeds: Vec<Crop<'a>>,
    corn_seeds: Vec<Crop<'a>>,
    potato_seeds: Vec<Crop<'a>>,
    lettuce_seeds: Vec<Crop<'a>>,
}

impl<'a> Inventory<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Inventory<'a> {


        let mut inventory_slots: Vec<Inventory_Item> = (0..10)
            .map(|x| {
                Inventory_Item::new(
                    x<3
                )
            })
            .collect();

            inventory_slots[5].add_item(
                Item::new(
                    Rect::new(5*32 , 0 , 32, 32),
                    texture_creator.load_texture("src/images/itemMenu.png").unwrap(),
                    "src/images/itemMenu.png".parse().unwrap(),
                    false,
                )
            );

            inventory_slots[5].add_item(
                Item::new(
                    Rect::new(5*32 , 0 , 32, 32),
                    texture_creator.load_texture("src/images/itemMenu.png").unwrap(),
                    "src/images/itemMenu.png".parse().unwrap(),
                    false,
                )
            );

/*
        let mut x = 0;
        for (i,inventory) in inventory_slots.iter().enumerate(){
            inventory.add_item(
                Item::new(
                    Rect::new(x*32 , 0 , 32, 32),
                    texture_creator.load_texture("src/images/itemMenu.png").unwrap(),
                    "src/images/itemMenu.png".parse().unwrap(),
                    false,
                )
            );
            x = x + 1;
        }
        */

        let temp_select = 0;
        let squares: Vec<Rect> = (0..10)
            .map(|x| {
                Rect::new(
                    INVENTORY_X_POS + (x * (ITEM_BOX_SIZE + BORDER_SIZE)),
                    INVENTORY_Y_POS,
                    ITEM_BOX_SIZE as u32,
                    ITEM_BOX_SIZE as u32,
                )
            })
            .collect();

        Inventory {
            inventory_slots,
            selected: temp_select,
            squares,
            carrot_seeds,
            corn_seeds,
            potato_seeds,
            lettuce_seeds,
        }
    }
    pub fn draw(&self,wincan: &mut WindowCanvas){
        wincan.set_draw_color(Color::RGBA(159,82,30,255));

        /// Draw background of inventory
        wincan.fill_rect(Rect::new(
            INVENTORY_X_POS-BORDER_SIZE,
            INVENTORY_Y_POS-BORDER_SIZE,
            (10*(ITEM_BOX_SIZE+BORDER_SIZE)+BORDER_SIZE) as u32,
            (ITEM_BOX_SIZE+2*BORDER_SIZE) as u32
            )).expect("ERROR");

        /// Draw selected box
        wincan.set_draw_color(Color::RGBA(244,0,0,255));
        wincan.fill_rect(Rect::new(
            INVENTORY_X_POS - SELECTED_SIZE+(self.selected*(ITEM_BOX_SIZE + 2*SELECTED_SIZE)),
            INVENTORY_Y_POS - SELECTED_SIZE,
            (ITEM_BOX_SIZE + 2*SELECTED_SIZE) as u32,
            (ITEM_BOX_SIZE + 2*SELECTED_SIZE) as u32
            )).expect("ERROR");

        wincan.set_draw_color(Color::RGBA(244, 182, 110, 255));
        wincan.fill_rects(&self.squares[..]).expect("ERROR");

        let mut x = 0;
        for inventory in &self.inventory_slots{
            if inventory.get_len() == 0{
                x = x + 1;
                continue;
            }
            //wincan.copy((*inventory.item[0]).texture(), (*inventory.item[0]).pos(),
            wincan.copy(inventory.get_item(0).texture(), inventory.get_item(0).pos(),

                 Rect::new(
                    INVENTORY_X_POS+(x*(ITEM_BOX_SIZE+BORDER_SIZE)),
                    INVENTORY_Y_POS,
                    ITEM_BOX_SIZE as u32,
                    ITEM_BOX_SIZE as u32
                )
                .unwrap();

                ).unwrap();
            if !inventory.is_tool{
                self.draw_numbers(wincan,  x, inventory.get_len());
            }

            x = x + 1;
        }

    }
    pub fn draw_numbers(&self, wincan: &mut WindowCanvas, inventory_slot: i32, mut value: i32){
        let NUMBER_SIZE = 20;


        let texture_creator = wincan.texture_creator();
        let values_texture = texture_creator.load_texture("src/images/outlined_numbers.png").unwrap();
        let mut digit_place = 1; 
        // Do-While loop in rust
        loop {
            let digit = value % 10;
            value /= 10;

            wincan.copy(
                &values_texture, 
                Rect::new(20*digit, 0, 20, 20), 
                Rect::new(
                    INVENTORY_X_POS+((inventory_slot+1)*(ITEM_BOX_SIZE+BORDER_SIZE))-digit_place*NUMBER_SIZE,
                    INVENTORY_Y_POS+ITEM_BOX_SIZE-NUMBER_SIZE, 
                    NUMBER_SIZE as u32, 
                    NUMBER_SIZE as u32
                    )
                );
            digit_place += 1;

            // While
            if (value == 0) {
                break;
            }

        }
    }

    pub fn set_selected(&mut self,_selected: i32){
        self.selected = _selected
    }

    pub fn get_selected(&self) -> i32 {
        self.selected
    }

    pub fn get_carrot_seed(&mut self, index: i32) -> Option<Crop<'a>> {
        if index < self.carrot_seeds.len() as i32 {
            Some(self.carrot_seeds.remove(index as usize))
        } else {
            None
        }
    }

    pub fn get_corn_seed(&mut self, index: i32) -> Option<Crop<'a>> {
        if index < self.corn_seeds.len() as i32 {
            Some(self.corn_seeds.remove(index as usize))
        } else {
            None
        }
    }

    pub fn get_potato_seed(&mut self, index: i32) -> Option<Crop<'a>> {
        if index < self.potato_seeds.len() as i32 {
            Some(self.potato_seeds.remove(index as usize))
        } else {
            None
        }
    }

    pub fn get_lettuce_seed(&mut self, index: i32) -> Option<Crop<'a>> {
        if index < self.lettuce_seeds.len() as i32 {
            Some(self.lettuce_seeds.remove(index as usize))
        } else {
            None
        }
    }
}