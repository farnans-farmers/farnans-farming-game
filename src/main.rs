#![deny(nonstandard_style, unused_parens)]

extern crate sdl2;

// Modules
mod agent;
mod anim;
mod commodities;
mod crop;
mod genes;
mod inventory;
mod item;
mod market;
mod market_item;
mod pest;
mod pest_population;
mod player;
mod population;
mod save_load;
mod sleep_menu;
mod store;
mod tile;
mod tool;
mod trade_house;

use anim::Animation;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;
use sdl2::render::Texture;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;

use crate::crop::CropType;
use crate::market_item::MarketItem;
use crate::player::{PLAYER_HEIGHT, PLAYER_WIDTH};

const VSYNC: bool = true;
// Camera dimensions
pub const CAM_W: u32 = 1280;
pub const CAM_H: u32 = 720;
// Background dimensions
const BG_W: u32 = 3000;
const BG_H: u32 = 3000;
const TITLE: &str = "Farnan's Farmers";
pub const TILE_SIZE: u32 = 80; // Make this public so we can import it elsewhere

// Right and bottom tile bounds
pub const RIGHT_TILE_BOUND: i32 = (BG_W / TILE_SIZE) as i32;
pub const BOTTOM_TILE_BOUND: i32 = (BG_H / TILE_SIZE) as i32;

pub enum Menu {
    Sleep,
    ToMarket,
    ToHome,
    Shop,
}

#[derive(Copy, Clone)]
pub enum Area {
    Home,
    Market,
}

/// Trait used for items that can exist inside of the inventory
pub trait InventoryItemTrait {
    /// Return some determined value to sort the inventory
    fn get_value(&self) -> i32;
    // Get the texture
    fn texture(&self) -> &Texture;
    /// Get the pos
    fn src(&self) -> Rect;
    /// Perform the correct action for the inventory slot item
    fn inventory_input(
        &self,
        square: (i32, i32),
        pop: &mut population::Population,
    ) -> Option<(
        Option<crop::CropType>,
        Option<genes::Genes>,
        Option<genes::Genes>,
    )>;
    /// Make save string for crops; return None for tools
    fn to_save_string(&self) -> Option<String>;
}

fn main() {
    let sdl_cxt = sdl2::init().unwrap();
    let video_subsys = sdl_cxt.video().unwrap();

    let window = video_subsys
        .window(TITLE, CAM_W, CAM_H)
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let wincan = window.into_canvas().accelerated();

    // Check if we should lock to vsync
    let wincan = if VSYNC {
        wincan.present_vsync()
    } else {
        wincan
    };

    let mut wincan = wincan.build().map_err(|e| e.to_string()).unwrap();

    wincan.set_blend_mode(BlendMode::Blend);
    let texture_creator = wincan.texture_creator();
    let r = Rect::new((0) as i32, (0) as i32, CAM_W, CAM_H);
    wincan.set_draw_color(Color::RGBA(255, 255, 255, 255));
    wincan.clear();

    // Crop and tile textures; all use the same one, so
    // just reference it for efficiency
    let crop_texture = texture_creator
        .load_texture("src/images/Crop_Tileset.png")
        .unwrap();

    let rotten_texture = texture_creator
        .load_texture("src/images/RottingCrops.png")
        .unwrap();

    let tile_texture = texture_creator
        .load_texture("src/images/Background_Tileset.png")
        .unwrap();

    // Roll group credits
    // let _ = roll_credits(&mut wincan, &texture_creator, r);
    // roll_credits(&mut wincan, &texture_creator, r).unwrap();

    let mut event_pump = sdl_cxt.event_pump().unwrap();
    let _x_vel = 0;
    let _y_vel = 0;

    let _menu_location = 0;

    let mut p = player::Player::new(
        Rect::new(
            (BG_W / 2 - PLAYER_WIDTH / 2) as i32,
            (BG_H / 2 - PLAYER_HEIGHT / 2) as i32,
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
        ),
        texture_creator
            .load_texture("src/images/farmer.png")
            .unwrap(),
        &texture_creator,
    );

    let mut pest_pop = save_load::load_pests();

    let _crop_vec: Vec<crop::Crop> = Vec::new();

    // LOAD SAVE DATA
    // Load home area
    let home_tup = save_load::load_home(
        &texture_creator,
        &crop_texture,
        &rotten_texture,
        &tile_texture,
    );
    let mut pop = home_tup.0;
    let item_vec = home_tup.1;

    // Load market
    let market_tup = save_load::load_market(
        &texture_creator,
        &crop_texture,
        &rotten_texture,
        &tile_texture,
    );
    let m_pop = market_tup.0;
    let m_item_vec = market_tup.1;

    // Load inventory
    save_load::load_inventory(p.get_inventory(), &crop_texture, &rotten_texture);

    // create a store with temp items
    let _seed_textures = texture_creator
        .load_texture("src/images/Crop_Tileset.png")
        .unwrap();

    let store_item_0 = MarketItem::new(0, 10, 3, 4, Rect::new(0, 0, 80, 80), CropType::Carrot, 0);
    let store_item_1 = MarketItem::new(7, 12, 2, 3, Rect::new(0, 80, 80, 80), CropType::Corn, 0);
    let store_item_2 = MarketItem::new(14, 11, 4, 5, Rect::new(0, 160, 80, 80), CropType::Lettuce, 0);
    let store_item_3 = MarketItem::new(21, 15, 6, 7, Rect::new(0, 240, 80, 80), CropType::Potato, 0);
    let store_item_4 = MarketItem::new(28, 10, 3, 4, Rect::new(240, 0, 80, 80), CropType::Carrot, 3);
    let store_item_5 = MarketItem::new(35, 12, 2, 3, Rect::new(240, 80, 80, 80), CropType::Corn, 3);
    let store_item_6 = MarketItem::new(42, 11, 4, 5, Rect::new(240, 160, 80, 80), CropType::Lettuce, 3);
    let store_item_7 = MarketItem::new(49, 15, 6, 7, Rect::new(240, 240, 80, 80), CropType::Potato, 3);



    let mut market_items = vec![store_item_0, store_item_1, store_item_2, store_item_3, store_item_4, store_item_5, store_item_6, store_item_7];

    let mut store = store::Store::new(8, &mut market_items);

    let mut in_area = Area::Home;
    // Things that might be used every frame but should only be loaded once:
    let _bg_tiles_tex = texture_creator
        .load_texture("src/images/Background_Tileset.png")
        .unwrap();

    // enum used to pause the game while any menu is up.
    let mut in_menu: Option<Menu> = None;
    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    save_load::save_home(pop, item_vec);
                    save_load::save_inventory(p.get_inventory());
                    save_load::save_pests(pest_pop);
                    break 'gameloop;
                }
                _ => {}
            }
        }

        let keystate: HashSet<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let mut x_deltav_f: f32 = 0.0;
        let mut y_deltav_f: f32 = 0.0;

        match in_menu {
            None => {
                // Change directions using WASD
                if keystate.contains(&Keycode::W) {
                    y_deltav_f -= player::ACCEL_RATE;
                }
                if keystate.contains(&Keycode::A) {
                    x_deltav_f -= player::ACCEL_RATE;
                }
                if keystate.contains(&Keycode::S) {
                    y_deltav_f += player::ACCEL_RATE;
                }
                if keystate.contains(&Keycode::D) {
                    x_deltav_f += player::ACCEL_RATE;
                }

                if event_pump.mouse_state().left() || keystate.contains(&Keycode::C) {
                    let coordinates = p.get_facing();
                    // Use inventory slot function
                    // Result is given when we want to add an item to the inventory
                    // This is done when a fully grown crop is used by the hand
                    match in_area {
                        Area::Home => {
                            let result = p.use_inventory(coordinates, &mut pop);
                            match result {
                                Some((Some(t), Some(g), child)) => {
                                    //Return multiple seeds from harvesting a plant
                                    //This may want to be determined on a plant's genes later

                                    let mut grown_crop = crop::Crop::new(
                                        Rect::new(0, 0, 0, 0),
                                        3,
                                        &crop_texture,
                                        &rotten_texture,
                                        false,
                                        t,
                                        Some(g.clone()),
                                    );
                                    grown_crop.set_stage(3);
                                    p.add_item(grown_crop);

                                    if let Some(_child) = child {
                                        // Return seed with child genes
                                        let new_crop = crop::Crop::new(
                                            Rect::new(0, 0, 0, 0),
                                            0,
                                            &crop_texture,
                                            &rotten_texture,
                                            false,
                                            t,
                                            Some(_child.clone()),
                                        );
                                        p.add_item(new_crop);
                                    }

                                    // for _seeds_returned in 0..2 {
                                    //     let new_crop = crop::Crop::new(
                                    //         Rect::new(0, 0, 0, 0),
                                    //         0,
                                    //         &crop_texture,
                                    //         false,
                                    //         t,
                                    //         Some(g.clone()),
                                    //     );
                                    //     p.add_item(new_crop);
                                    // }
                                }
                                None => (),
                                _ => (),
                            };
                        }
                        Area::Market => (),
                    }
                }
                if keystate.contains(&Keycode::Num1) {
                    p.set_selected(0);
                }
                if keystate.contains(&Keycode::Num2) {
                    p.set_selected(1);
                }
                if keystate.contains(&Keycode::Num3) {
                    p.set_selected(2);
                }
                if keystate.contains(&Keycode::Num4) {
                    p.set_selected(3);
                }
                if keystate.contains(&Keycode::Num5) {
                    p.set_selected(4);
                }
                if keystate.contains(&Keycode::Num6) {
                    p.set_selected(5);
                }
                if keystate.contains(&Keycode::Num7) {
                    p.set_selected(6);
                }
                if keystate.contains(&Keycode::Num8) {
                    p.set_selected(7);
                }
                if keystate.contains(&Keycode::Num9) {
                    p.set_selected(8);
                }
                if keystate.contains(&Keycode::Num0) {
                    p.set_selected(9);
                }
                if keystate.contains(&Keycode::Minus) {
                    p.set_selected(10);
                }
            }
            //I know having 3 seperate methods isn't really 'modular' but the code has already been written for each and they all require different things so... this is it
            Some(Menu::Sleep) => {
                in_menu = sleep_menu::start_sleep_menu(
                    in_menu,
                    &mut wincan,
                    keystate,
                    &mut p,
                    &mut pop,
                    r,
                    &mut pest_pop,
                );
            }
            Some(Menu::ToMarket) => {
                let menu_and_area_tup = market::start_market_transition_menu(
                    in_menu,
                    &mut wincan,
                    keystate,
                    r,
                    Some(in_area),
                );
                in_menu = menu_and_area_tup.0;
                in_area = menu_and_area_tup.1;
            }
            Some(Menu::ToHome) => {
                let menu_and_area_tup = market::start_market_transition_menu(
                    in_menu,
                    &mut wincan,
                    keystate,
                    r,
                    Some(in_area),
                );
                in_menu = menu_and_area_tup.0;
                in_area = menu_and_area_tup.1;
            }
            Some(Menu::Shop) => {
                if keystate.contains(&Keycode::Q) {
                    in_menu = None;
                }
                if keystate.contains(&Keycode::Up) {
                    store.navigate(-1);
                    thread::sleep(Duration::from_millis(160));
                }
                if keystate.contains(&Keycode::Down) {
                    store.navigate(1);
                    thread::sleep(Duration::from_millis(160));
                }
                if keystate.contains(&Keycode::Left) {
                    store.cycle(-1);
                    thread::sleep(Duration::from_millis(160));
                }
                if keystate.contains(&Keycode::Right) {
                    store.cycle(1);
                    thread::sleep(Duration::from_millis(160));
                }
                if keystate.contains(&Keycode::P) {
                    let _new_crop_texture = texture_creator
                        .load_texture("src/images/Crop_Tileset.png")
                        .unwrap();
                    let _p = store.confirm_purchase();
                    if let Some((a, t, g)) = _p {
                        for _ in 0..a {
                            let _c = crop::Crop::new(
                                Rect::new(0, 0, 0, 0),
                                g as u8,
                                &crop_texture,
                                &rotten_texture,
                                false,
                                t,
                                Some(genes::Genes::new()),
                            );
                            p.add_item(_c);
                        }
                    }
                    in_menu = None;
                    thread::sleep(Duration::from_millis(160));
                }
            }
        }

        let player_vel = p.set_speed((x_deltav_f, y_deltav_f));
        p.set_direction(player_vel);

        // Update player position. Varies per area for collision detection.
        match in_area {
            Area::Home => {
                // X
                p.update_pos_x(player_vel, (0, (BG_W - TILE_SIZE) as i32));
                for item in &item_vec {
                    if p.check_collision(&item.pos()) {
                        p.stay_still_x(player_vel, (0, (BG_W - TILE_SIZE) as i32));
                        if item.tex_path() == "src/images/house.png" {
                            in_menu = Some(Menu::Sleep);
                        } else if item.tex_path() == "src/images/go_market.png" {
                            in_menu = Some(Menu::ToMarket);
                        } else if item.tex_path() == "src/images/Barn.png" {
                            in_menu = Some(Menu::Shop);
                        }
                    }
                }

                //Y
                p.update_pos_y(player_vel, (0, (BG_W - TILE_SIZE) as i32));
                for item in &item_vec {
                    if p.check_collision(&item.pos()) {
                        p.stay_still_y(player_vel, (0, (BG_W - TILE_SIZE) as i32));
                        if item.tex_path() == "src/images/house.png" {
                            in_menu = Some(Menu::Sleep);
                        } else if item.tex_path() == "src/images/go_market.png" {
                            in_menu = Some(Menu::ToMarket);
                        } else if item.tex_path() == "src/images/Barn.png" {
                            in_menu = Some(Menu::Shop);
                        }
                    }
                }
            }
            Area::Market => {
                market::update_market_pos(&mut p, &m_item_vec, player_vel, &mut in_menu)
            }
        }

        // Determine part of background to draw
        let cur_bg = match in_area {
            Area::Home => Rect::new(
                ((p.x() + ((p.width() / 2) as i32)) - ((CAM_W / 2) as i32))
                    .clamp(0, (BG_W - CAM_W) as i32),
                ((p.y() + ((p.height() / 2) as i32)) - ((CAM_H / 2) as i32))
                    .clamp(0, (BG_H - CAM_H) as i32),
                CAM_W,
                CAM_H,
            ),
            Area::Market => market::background_to_draw(&p),
        };

        // Convert player map position to be camera-relative
        let player_cam_pos = Rect::new(
            p.x() - cur_bg.x(),
            p.y() - cur_bg.y(),
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
        );

        wincan.set_draw_color(Color::BLACK);
        wincan.clear();

        // Draw tiles
        match in_area {
            Area::Home => {
                let coordinates = p.get_facing();
                for crop_tile in pop.get_vec().iter().flatten() {
                    let x_pos = crop_tile.tile.x() - cur_bg.x();
                    let y_pos = crop_tile.tile.y() - cur_bg.y();
                    //Don't bother drawing any tiles that are off screen
                    if x_pos > -(TILE_SIZE as i32)
                        && x_pos < (CAM_W as i32)
                        && y_pos > -(TILE_SIZE as i32)
                        && y_pos < (CAM_H as i32)
                    {
                        let cur_tile = Rect::new(
                            crop_tile.tile.x() - cur_bg.x(),
                            crop_tile.tile.y() - cur_bg.y(),
                            TILE_SIZE,
                            TILE_SIZE,
                        );
                        wincan
                            .copy(crop_tile.tile.texture(), crop_tile.tile.src(), cur_tile)
                            .unwrap();
                        if (
                            crop_tile.tile.x() / TILE_SIZE as i32,
                            crop_tile.tile.y() / TILE_SIZE as i32,
                        ) == coordinates
                        {
                            // println!("Drawing rect!");
                            wincan.set_draw_color(Color::RED);
                            wincan
                                .draw_rect(Rect::new(x_pos, y_pos, TILE_SIZE, TILE_SIZE))
                                .unwrap();
                        }
                    }
                }
                // Drawing item
                for item in &item_vec {
                    wincan = item.print_item(cur_bg.x(), cur_bg.y, CAM_W, CAM_H, wincan);
                }

                // Draw crops
                for _x in 0..((BG_W / TILE_SIZE) as i32 + 1) {
                    for _y in 0..((BG_H / TILE_SIZE) as i32 + 1) {
                        let _c = pop.get_crop_with_index(_x as u32, _y as u32);
                        match _c.get_crop_type() {
                            "None" => {}
                            _ => {
                                wincan = _c.print_crop(cur_bg.x(), cur_bg.y(), wincan);
                            }
                        }
                    }
                }
            }
            Area::Market => wincan = market::draw_market(wincan, &m_pop, &cur_bg, &m_item_vec),
        }

        // Draw inventory
        p.draw(&mut wincan, player_cam_pos);
        //ui.draw(&mut wincan);

        match in_menu {
            None => {}
            Some(Menu::Sleep) => {
                let sleep_box = texture_creator
                    .load_texture("src/images/sleep.png")
                    .unwrap();
                wincan
                    .copy(&sleep_box, None, Rect::new(400, 400, 600, 180))
                    .unwrap();
            }
            Some(Menu::ToMarket) => {
                let go_box = texture_creator
                    .load_texture("src/images/market_menu.png")
                    .unwrap();
                wincan
                    .copy(&go_box, None, Rect::new(400, 400, 600, 180))
                    .unwrap()
            }
            Some(Menu::ToHome) => {
                let go_box = texture_creator
                    .load_texture("src/images/go_home_menu.png")
                    .unwrap();
                wincan
                    .copy(&go_box, None, Rect::new(400, 400, 600, 180))
                    .unwrap()
            }
            Some(Menu::Shop) => {
                store.draw(&mut wincan);
            }
        }

        wincan.present();
    } // end gameloop
}
