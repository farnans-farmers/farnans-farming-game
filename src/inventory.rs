use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::crop::Crop;
use crate::crop::CropType;
use crate::tool::Tool;
use crate::population::Population;
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


/// Individual inventory slot. This takes in an inventory trait object(crop or tool)
/// Inventory slots are sorted, so you have the "best" seed at the bottom of the queue
/// This is done so that seed can have different genetics, but still have one inventory slot
/// The vectors are sorted by their value: a number that is determined in the crop class
struct Inventory_Item<'a>{
    items: Vec<Box<dyn inventory_item_trait + 'a>>,
    is_tool: bool
}

impl<'a> Inventory_Item<'a>{
    /// Takes in is_tool: used to differentiate tools from crops
    /// and initializes vector of inventory_item_trait
    pub fn new(is_tool: bool) -> Inventory_Item<'a>{
        Inventory_Item{
            items: Vec::new(),
            is_tool
        }
    }

    pub fn get_len(&self) -> i32{
        self.items.len() as i32
    }
    
    /// Insert item into sorted vector
    /// Right now its just insertion sort
    /// Might change to a more efficient insertion if there is time
    pub fn add_item(&mut self,new_item: Box<dyn inventory_item_trait + 'a>){
        let mut i = 0;
        let mut insert_pos = self.get_len() as usize;
        for item in &self.items{
            if item.get_value() < new_item.get_value(){
                insert_pos = i;
                break;
            }
            i = i + 1;
        }
        self.items.insert(insert_pos,new_item);
    }

    /// TODO still need to implement using/planting crops
    /// This will pop the highest sorted item at index 0
    pub fn pop_item(&mut self) -> Box<dyn inventory_item_trait + 'a> {
        self.items.pop().unwrap()
    }
    pub fn get_item(&self, index: i32) -> Option<&Box<dyn inventory_item_trait + 'a>>{
        if index >= self.get_len(){
           return None;
        }
        Some(&(self.items[index as usize]))
    }
}


/// Inventory class that has a vector of inventory slots
/// Also keeps track of the current selected inventory slot
/// squares is used to draw the inventory slot. It is kept here so that it doesn't have to be initialized each time you want to draw
pub struct Inventory<'a> {
    inventory_slots: Vec<Inventory_Item<'a>>,
    selected: i32,
    squares: Vec<Rect>,
    carrot_seeds: Vec<Crop<'a>>,
    corn_seeds: Vec<Crop<'a>>,
    potato_seeds: Vec<Crop<'a>>,
    lettuce_seeds: Vec<Crop<'a>>,
}

/// Takes in texture_creator in order to load tools into the tool slots
impl<'a> Inventory<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Inventory<'a> {

        /// Initializes inventory slots and sets tool slots to true
        let mut inventory_slots: Vec<Inventory_Item> = (0..10)
            .map(|x| {
                Inventory_Item::new(
                    x<3
                )
            }).collect();

            /// Add tool slots into the inventory
            inventory_slots[0].add_item(
                Box::new(
                    Tool::new(
                        Rect::new(0*32 , 0 , 32, 32),
                        texture_creator.load_texture("src/images/itemMenu.png").unwrap(),
                        crate::tool::tool_type::hand
                    )
                )
            );

            inventory_slots[1].add_item(
                Box::new(
                    Tool::new(
                        Rect::new(1*32 , 0 , 32, 32),
                        texture_creator.load_texture("src/images/itemMenu.png").unwrap(),
                        crate::tool::tool_type::hoe
                    )
                )
            );

            inventory_slots[2].add_item(
                Box::new(
                    Tool::new(
                        Rect::new(2*32 , 0 , 32, 32),
                        texture_creator.load_texture("src/images/itemMenu.png").unwrap(),
                        crate::tool::tool_type::watering_can
                    )
                )
            );

        let temp_select = 0;

        /// Initialize squares to be drawn
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

    /// Draw inventory slots onto the canvas 
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

        /// Draw each inventory slot
        for inventory in &self.inventory_slots{

            /// Don't draw empty slots
            if inventory.get_len() == 0{
                x = x + 1;
                continue;
            }

            let current_item = inventory.get_item(0).unwrap();
            wincan.copy(current_item.texture(), current_item.src(),

                 Rect::new(
                    INVENTORY_X_POS+(x*(ITEM_BOX_SIZE+BORDER_SIZE)),
                    INVENTORY_Y_POS,
                    ITEM_BOX_SIZE as u32,
                    ITEM_BOX_SIZE as u32
                )
            ).unwrap();

            /// Dont draw tool slots
            /// This is so that it isn't shown that there is (1) tool
            if !inventory.is_tool{
                self.draw_numbers(wincan,  x, inventory.get_len());
            }

            x = x + 1;
        }

    }

    /// Draw length for inventory slot
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

    /// Add item into the correct inventory slot
    /// Right now the correct slot is hard coded
    pub fn add_item(&mut self, new_crop: Crop<'a>){
        let inventory_slot_index = match new_crop.get_crop_type_enum(){
            CropType::Carrot => 3,
            CropType::Corn => 4,
            CropType::Potato => 5,
            CropType::Lettuce => 6,
            _ => 0,
        };
        self.inventory_slots[inventory_slot_index].add_item(
                Box::new(
                    new_crop
                )
            );
    }

    /// Use the inventory slot for the correct function
    /// For crops, this means planting the crop onto tilled soil
    pub fn use_inventory(&mut self,square:(i32, i32), mut pop: &mut Population) -> Option<CropType>{
        let current_item = self.inventory_slots[self.selected as usize].get_item(0);
        match current_item{
            Some(x) => {
                let ret_val = x.inventory_input(square,pop);
                
                match ret_val{
                    Some(y) => {
                        if matches!(y,CropType::None){
                            //TODO return this value rather than return the CropType
                            self.inventory_slots[self.selected as usize].pop_item();
                            return None;
                        }
                        else{
                            ret_val
                        }
                    },
                    None => None
                }
            },
            None    => None,
        }
    }
}