extern crate sdl2;

// Modules
mod anim;
mod crop;
mod inventory;
mod item;
mod player;
mod population;
mod store;
mod tile;
mod utilities;
mod home_area;
mod market_area;
mod sleep_menu;
mod market_transition_menu;

use anim::Animation;
use item::Item;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use crate::player::{Direction, PLAYER_HEIGHT, PLAYER_WIDTH};
use std::fs::File;
use std::io::{Read, Write};

const VSYNC: bool = true;
// Camera dimensions
pub const CAM_W: u32 = 1280;
pub const CAM_H: u32 = 720;
// Background dimensions
const BG_W: u32 = 3000;
const BG_H: u32 = 3000;
const TITLE: &str = "Farnan's Farmers";
pub const TILE_SIZE: u32 = 80; // Make this public so we can import it elsewhere

pub enum Menu {
    Sleep,
    ToMarket,
    Shop,
}

#[derive(Copy, Clone)]
pub enum Area {
    Home,
    Market,
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

    // Roll group credits
    // let _ = roll_credits(&mut wincan, &texture_creator, r);
    // roll_credits(&mut wincan, &texture_creator, r).unwrap();

    let mut event_pump = sdl_cxt.event_pump().unwrap();
    let mut x_vel = 0;
    let mut y_vel = 0;

    let mut tile_vec = Vec::new();
    for x in 0..((BG_W / TILE_SIZE) as i32) + 1 {
        let mut sub_vec = Vec::new();
        for y in 0..((BG_H / TILE_SIZE) as i32) + 1 {
            sub_vec.push(population::Crop_Tile::new(
                tile::Tile::new(
                    Rect::new(
                        (TILE_SIZE as i32) * x,
                        (TILE_SIZE as i32) * y,
                        TILE_SIZE,
                        TILE_SIZE,
                    ),
                    texture_creator
                        .load_texture("src/images/Background_Tileset.png")
                        .unwrap(),
                ),
                crop::Crop::new(
                    Rect::new(
                        (TILE_SIZE as i32) * x,
                        (TILE_SIZE as i32) * y,
                        TILE_SIZE,
                        TILE_SIZE,
                    ),
                    0,
                    texture_creator
                        .load_texture("src/images/Crop_Tileset.png")
                        .unwrap(),
                    false,
                    "src/images/Crop_Tileset.png".parse().unwrap(),
                    crop::CropType::None,
                ),
            ));
        }
        tile_vec.push(sub_vec);
    }
    let mut pop = population::Population::new(tile_vec);

    let mut menu_location = 0;

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

    let mut item_vec = Vec::new();
    let mut crop_vec: Vec<crop::Crop> = Vec::new();

    //Loading items and crops into the game
    {
        let mut file = File::open("src/foo.txt").expect("Can't open save file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Can't read file");
        print!("{}", contents);
        for line in contents.lines() {
            let results: Vec<&str> = line.split(";").collect();
            if (results[0] == "item") {
                item_vec.push(item::Item::new(
                    Rect::new(
                        results[1].parse::<i32>().unwrap(),
                        results[2].parse::<i32>().unwrap(),
                        results[3].parse::<u32>().unwrap(),
                        results[4].parse::<u32>().unwrap(),
                    ),
                    texture_creator.load_texture(results[5]).unwrap(),
                    results[5].parse().unwrap(),
                    results[6].parse::<bool>().unwrap(),
                ));
            } else if (results[0] == "crop") {
                let _x = results[1].parse::<i32>().unwrap();
                let _y = results[2].parse::<i32>().unwrap();
                pop.get_vec_mut()
                    .get_mut(_x as usize)
                    .unwrap()
                    .get_mut(_y as usize)
                    .unwrap()
                    .setCrop(
                        // crop_vec.push(
                        crop::Crop::new(
                            Rect::new(
                                results[1].parse::<i32>().unwrap() * TILE_SIZE as i32,
                                results[2].parse::<i32>().unwrap() * TILE_SIZE as i32,
                                TILE_SIZE,
                                TILE_SIZE,
                            ),
                            results[3].parse::<u8>().unwrap(),
                            texture_creator.load_texture(results[4]).unwrap(),
                            results[5].parse::<bool>().unwrap(),
                            results[4].parse().unwrap(),
                            results[6].parse::<crop::CropType>().unwrap(),
                        ),
                    );
                // If crop is present, set tile as tilled
                if results[6]
                    .parse::<std::string::String>()
                    .unwrap()
                    .to_owned()
                    != "None"
                {
                    let _tile = pop.get_tile_with_index_mut(_x as u32, _y as u32);
                    _tile.set_tilled(true);
                }
            }
        }
    }

    let mut store = store::Store::new(24);

    let mut in_area = Area::Home;
    // Things that might be used every frame but should only be loaded once:
    let bg_tiles_tex = texture_creator
        .load_texture("src/images/Background_Tileset.png")
        .unwrap();
    // TODO(branden): move this someplace reasonable
    let market_house = {
        let pos = Rect::new(2000, 2000, 533, 408);
        let texture = texture_creator
            .load_texture("src/images/Farmhouse.png")
            .unwrap();
        Item::new(pos, texture, "src/images/Farmhouse.png".into(), true)
    };
    // variable for sleep menu
    let mut in_menu: Option<Menu> = None;
    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    //Iterates through item vector and crop vector saving their positions into a txt file
                    let mut file = match File::create("src/foo.txt") {
                        Err(why) => panic!("couldn't create foo.txt: {}", why),
                        Ok(file) => file,
                    };
                    for item in item_vec {
                        let mut output = "item;".to_owned()
                            + &item.x().to_string()
                            + ";"
                            + &item.y().to_string()
                            + ";"
                            + &item.width().to_string()
                            + ";"
                            + &item.height().to_string()
                            + ";"
                            + &item.tex_path()
                            + ";"
                            + &item.collision().to_string()
                            + "\n";
                        match file.write_all(output.as_ref()) {
                            Err(why) => panic!("couldn't write to foo.txt: {}", why),
                            Ok(_) => println!("successfully wrote item to foo.txt"),
                        }
                    }

                    for _x in 0..((BG_W / TILE_SIZE) as i32 + 1) {
                        for _y in 0..((BG_H / TILE_SIZE) as i32 + 1) {
                            let _c = pop.get_crop_with_index(_x as u32, _y as u32);
                            match _c.get_crop_type() {
                                "None" => {}
                                _ => {
                                    let output = "crop;".to_owned()
                                        + &(_c.get_x() / TILE_SIZE as i32).to_string()
                                        + ";"
                                        + &(_c.get_y() / TILE_SIZE as i32).to_string()
                                        + ";"
                                        + &_c.get_stage().to_string()
                                        + ";"
                                        + &_c.get_tex_path()
                                        + ";"
                                        + &_c.get_watered().to_string()
                                        + ";"
                                        + &_c.get_crop_type()
                                        + "\n";
                                    match file.write_all(output.as_ref()) {
                                        Err(why) => panic!("couldn't write to foo.txt: {}", why),
                                        Ok(_) => println!("successfully wrote crop to foo.txt"),
                                    }
                                }
                            }
                        }
                    }
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
                    let offset: (i32, i32) = {
                        match p.get_dir() {
                            // Down
                            0 => (0, 1),
                            // Left
                            1 => (-1, 0),
                            // Right
                            2 => (1, 0),
                            // Up
                            3 => (0, -1),
                            // Other (shouldn't happen)
                            _ => (0, 0),
                        }
                    };
                    let coordinates = (
                        (((p.x() + TILE_SIZE as i32 / 2) / TILE_SIZE as i32) + offset.0)
                            .clamp(0, ((BG_W / TILE_SIZE) as i32) + 1),
                        (((p.y() + TILE_SIZE as i32) / TILE_SIZE as i32) + offset.1)
                            .clamp(0, ((BG_H / TILE_SIZE) as i32) + 1),
                    );
                    utilities::use_tool(coordinates.0, coordinates.1, &mut pop, p.get_selected());
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
            }
            //I know having 3 seperate methods isn't really 'modular' but the code has already been written for each and they all require different things so... this is it
            Some(Menu::Sleep) => {
                in_menu = sleep_menu::start_sleep_menu(in_menu, &mut wincan, keystate, &mut pop, r);
            }
            Some(Menu::ToMarket) => {
                let menu_and_area_tup = market_transition_menu::start_market_transition_menu(in_menu, &mut wincan, keystate, r, Some(in_area));
                in_menu = menu_and_area_tup.0;
                in_area = menu_and_area_tup.1;
            }
            Some(Menu::Shop) => {
                if keystate.contains(&Keycode::Q) {
                    in_menu = None;
                }
                if keystate.contains(&Keycode::Up) {
                    store.navigate(-1);
                    thread::sleep(Duration::from_millis(80));
                }
                if keystate.contains(&Keycode::Down) {
                    store.navigate(1);
                    thread::sleep(Duration::from_millis(80));
                }
                if keystate.contains(&Keycode::Left) {
                    store.cycle(-1);
                    thread::sleep(Duration::from_millis(80));
                }
                if keystate.contains(&Keycode::Right) {
                    store.cycle(1);
                    thread::sleep(Duration::from_millis(80));
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
                // TODO: check collisions for market items
                p.update_pos_x(player_vel, (0, (BG_W - TILE_SIZE) as i32));
                p.update_pos_y(player_vel, (0, (BG_W - TILE_SIZE) as i32));
            }
        }

        // Determine part of background to draw
        let cur_bg = Rect::new(
            ((p.x() + ((p.width() / 2) as i32)) - ((CAM_W / 2) as i32))
                .clamp(0, (BG_W - CAM_W) as i32),
            ((p.y() + ((p.height() / 2) as i32)) - ((CAM_H / 2) as i32))
                .clamp(0, (BG_H - CAM_H) as i32),
            CAM_W,
            CAM_H,
        );

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
            Area::Market => {
                // TODO(branden): draw a full-size area with scrolling &c.
                // Draw tiles. All tiles in the market are grass for now.
                let tr = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
                for x in 0..BG_W / TILE_SIZE + 1 {
                    for y in 0..BG_H / TILE_SIZE + 1 {
                        let dst = Rect::new(
                            (x * TILE_SIZE) as i32,
                            (y * TILE_SIZE) as i32,
                            TILE_SIZE,
                            TILE_SIZE,
                        );
                        wincan.copy(&bg_tiles_tex, tr, dst).unwrap();
                    }
                }
                // Draw "market house".
                wincan = market_house.print_item(cur_bg.x(), cur_bg.y(), CAM_W, CAM_H, wincan);
            }
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
            Some(Menu::Shop) => {
                store.draw(&mut wincan);
            }
        }

        wincan.present();
    } // end gameloop
}

/**
 * Method to display team creditsF
 */
fn roll_credits<T>(
    window: &mut WindowCanvas,
    tc: &TextureCreator<T>,
    r: Rect,
) -> Result<(), String> {
    // paths for group images
    let img1 = "src/images/credits/jaysonCredits.png";
    let img2 = "src/images/credits/JackMCredits.png";
    let img3 = "src/images/credits/natCredits.png";
    let img4 = "src/images/credits/jacobCredits.png";
    let img5 = "src/images/credits/wesleyCredits.png";
    let img6 = "src/images/credits/jackACredits.png";
    let img7 = "src/images/credits/brandenCredits.png";
    let images = [img1, img2, img3, img4, img5, img6, img7];

    // Iterate through images; fade in and out
    for img in 0..images.len() {
        let _ = fade(window, tc.load_texture(images[img]).unwrap(), r);
    }

    Ok(())
}

// method to fade in and out
fn fade(window: &mut WindowCanvas, ms: Texture, r: Rect) -> Result<(), String> {
    // fade in
    let mut i = 0;
    while i < 254 {
        window.clear();
        window.copy(&ms, None, None)?;
        window.set_draw_color(Color::RGBA(255, 255, 255, 255 - i));
        window.fill_rect(r)?;
        window.present();
        thread::sleep(Duration::from_millis(1));
        i = i + 2;
    }

    thread::sleep(Duration::from_secs(1));

    // fade out
    i = 0;
    while i < 254 {
        window.clear();
        window.copy(&ms, None, None)?;
        window.set_draw_color(Color::RGBA(255, 255, 255, i));
        window.fill_rect(r)?;
        window.present();
        thread::sleep(Duration::from_millis(1));
        i = i + 2;
    }
    Ok(())
}
