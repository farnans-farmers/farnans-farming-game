use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use sdl2::render::WindowCanvas;

use crate::market_item::MarketItem;

pub struct Store<'a> {
    item_selected: i32,
    amount_selected: i32,
    price: i32,
    sub_menu: i32,
    number_of_goods: i32,
    item_rect: Rect,
    money_rect: Rect,
    amount_rect: Rect,
    menu_rect: Rect,
    items_array: &'a mut Vec<MarketItem>,
}

impl<'a> Store<'a> {
    pub fn new(number_of_goods: i32, items_array: &'a mut Vec<MarketItem>) -> Store<'a> {
        let item_selected = 0;
        let amount_selected = 1;
        let price = 1;
        let sub_menu = 0;
        let item_rect = Rect::new(150, 30, 500, 580);
        let money_rect = Rect::new(660, 570, 470, 40);
        let amount_rect = Rect::new(660, 510, 470, 40);
        let menu_rect = Rect::new(150, 30, 500, 580);
        Store {
            item_selected,
            amount_selected,
            price,
            sub_menu,
            number_of_goods,
            item_rect,
            money_rect,
            amount_rect,
            menu_rect,
            items_array,
        }
    }

    pub fn draw(&self, wincan: &mut WindowCanvas) {
        let _item_rect = Rect::new(150, 30, 500, 580);

        //draw menu canvas
        wincan.set_draw_color(Color::RGBA(159, 82, 30, 255));
        wincan.fill_rect(Rect::new(140, 20, 1000, 600)).unwrap();

        // draw menu features
        wincan.set_draw_color(Color::RGBA(244, 182, 110, 255));
        // items
        wincan.fill_rect(Rect::new(150, 30, 500, 580)).unwrap();
        // image
        wincan.fill_rect(Rect::new(660, 30, 470, 470)).unwrap();
        // amount and price
        wincan.fill_rect(Rect::new(660, 510, 470, 100)).unwrap();

        // draw item labels
        Store::item_list_draw(wincan, self.items_array);

        // selection
        wincan.set_draw_color(Color::RGBA(255, 0, 0, 60));
        wincan
            .fill_rect(Rect::new(150, 30 + self.item_selected * 50, 500, 50))
            .unwrap();

        // submenu
        wincan.set_draw_color(Color::RGBA(0, 0, 0, 40));
        wincan.fill_rect(self.menu_rect).unwrap();
        Store::price_draw(wincan, 6, 665, 578, self.price);
        Store::price_draw(wincan, 6, 665, 518, self.amount_selected);

        let texture_creator = wincan.texture_creator();
        let label_texture = texture_creator
            .load_texture("src/images/MoneyLabels.png")
            .unwrap();
        wincan
            .copy(
                &label_texture,
                Rect::new(0, 0, 16, 5),
                Rect::new(850, 578, 80, 25),
            )
            .unwrap();
        wincan
            .copy(
                &label_texture,
                Rect::new(18, 0, 20, 5),
                Rect::new(850, 518, 100, 25),
            )
            .unwrap();

        let item_textures = texture_creator
            .load_texture("src/images/Crop_Tileset.png")
            .unwrap();
        wincan
            .copy(
                &item_textures,
                self.items_array[self.item_selected as usize].pos,
                Rect::new(665, 35, 460, 460),
            )
            .unwrap();
    }

    pub fn navigate(&mut self, increment: i32) {
        if self.sub_menu == 0 {
            if increment == -1 && self.item_selected != 0 {
                self.item_selected = self.item_selected + increment;
                self.price = 1;
                self.amount_selected = 1;
            }
            if increment == 1 && self.item_selected != self.number_of_goods - 1 {
                self.item_selected = self.item_selected + increment;
                self.price = 1;
                self.amount_selected = 1;
            }
        }

        if self.sub_menu == 2 {
            if increment == -1 && self.price != 99999 {
                self.price = self.price - increment;
            }
            if increment == 1 && self.price != 1 {
                self.price = self.price - increment;
            }
        }

        if self.sub_menu == 1 {
            if increment == -1
                && self.amount_selected != self.items_array[self.item_selected as usize].amount
            {
                self.price = self.price + (self.price / self.amount_selected);
                self.amount_selected = self.amount_selected - increment;
            }
            if increment == 1 && self.amount_selected != 1 {
                self.price = self.price - (self.price / self.amount_selected);
                self.amount_selected = self.amount_selected - increment;
            }
        }
    }

    pub fn cycle(&mut self, increment: i32) {
        if increment == -1 && self.sub_menu != 0 {
            self.sub_menu = self.sub_menu + increment;
        }
        if increment == 1 && self.sub_menu != 2 {
            self.sub_menu = self.sub_menu + increment;
        }
        if self.sub_menu == 0 {
            self.menu_rect = self.item_rect;
        }
        if self.sub_menu == 1 {
            self.menu_rect = self.amount_rect;
        }
        if self.sub_menu == 2 {
            self.menu_rect = self.money_rect;
        }
    }

    pub fn price_draw(
        wincan: &mut WindowCanvas,
        mut steps: i32,
        initx: i32,
        inity: i32,
        value: i32,
    ) {
        let texture_creator = wincan.texture_creator();
        let values_texture = texture_creator
            .load_texture("src/images/MoneySpriteSheet.png")
            .unwrap();
        let initsteps = steps;
        let mut disp_price = value;
        let mut tailing_zero = false;
        while steps >= 0 {
            let temp = i32::pow(10, steps as u32);
            let mut modulo = (disp_price - (disp_price % temp)) / temp;
            disp_price = disp_price - (modulo * temp);
            if tailing_zero && modulo == 0 {
                modulo = 10;
            }

            if modulo != 0 {
                tailing_zero = true;
            }

            wincan
                .copy(
                    &values_texture,
                    Rect::new(5 * modulo, 0, 5, 5),
                    Rect::new((initx + (initsteps - steps) * 25) as i32, inity, 25, 25),
                )
                .unwrap();

            steps = steps - 1;
        }
    }

    pub fn item_list_draw(wincan: &mut WindowCanvas, items_array: &[MarketItem]) {
        let texture_creator = wincan.texture_creator();
        let market_menu_items = texture_creator
            .load_texture("src/images/Market_menu_items.png")
            .unwrap();
        let mut i = 0;
        for item in items_array {
            wincan
                .copy(
                    &market_menu_items,
                    Rect::new(0, item.item_label_offset, 100, 6),
                    Rect::new(150, 30 + i * 50, 500, 50),
                )
                .unwrap();
            Store::price_draw(wincan, 3, 380, 45 + i * 50, item.amount);
            Store::price_draw(wincan, 3, 530, 45 + i * 50, item.price);
            i = i + 1;
        }
    }

    pub fn confirm_purchase(&mut self) {
        let total = self.items_array[self.item_selected as usize].price * self.amount_selected;

        if total <= self.price && total != 0 {
            self.items_array[self.item_selected as usize].amount =
                self.items_array[self.item_selected as usize].amount - self.amount_selected;

            // loop for self.amount_selected
            // add seed type to the inventory
            // self.items_array[self.item_selected as usize].crop is the enum of the crop type

            /*
            let _c = Crop::new(
                Rect::new(0, 0, 80, 80),
                0,
                texture,
                false,
                CropType::Lettuce,
                Some(Genes::new()),
            );
            p.add_item(_c);
            */
        }

        self.amount_selected = 1;
        self.price = 1;
    }
}
