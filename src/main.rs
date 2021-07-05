extern crate sdl2;

// Modules
mod crop;
mod item;
mod anim;
mod player;
mod tile;

mod inventory;
mod population;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;

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


fn check_collision(a: &Rect, b: &Rect) -> bool {
    if a.bottom() < b.top() || a.top() > b.bottom() || a.right() < b.left() || a.left() > b.right()
    {
        false
    } else {
        true
    }
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
            sub_vec.push(population::CropTile::new(
                tile::Tile::new(
                    Rect::new(
                        (TILE_SIZE as i32) * x,
                        (TILE_SIZE as i32) * y,
                        TILE_SIZE,
                        TILE_SIZE,
                    ),
                    texture_creator
                        .load_texture("src/images/grass.png")
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
                    texture_creator.load_texture("src/images/Crop_Tileset.png").unwrap(),
                    false,
                    "src/images/Crop_Tileset.png".parse().unwrap(),
                    crop::CropType::None,
                )
            )
            );
        }
        tile_vec.push(sub_vec);
    };
    let mut pop = population::Population::new(tile_vec);

    let mut menu_location = 0 ;

    let inventory_slots: Vec<item::Item> = (0..10)
        .map(|x| {
            item::Item::new(
                Rect::new(x*32 , 0 , 32, 32),
                texture_creator.load_texture("src/images/itemMenu.png").unwrap(),
                "src/images/itemMenu.png".parse().unwrap(),
                false,
            )

        })
        .collect();

    let mut inventory = inventory::Inventory::new(inventory_slots);

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
    );

    let mut item_vec = Vec::new();
    let mut crop_vec = Vec::new();

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
                    Rect::new(results[1].parse::<i32>().unwrap(),
                              results[2].parse::<i32>().unwrap(),
                              results[3].parse::<u32>().unwrap(),
                              results[4].parse::<u32>().unwrap()),
                    texture_creator.load_texture(results[5]).unwrap(),
                    results[5].parse().unwrap(),
                    results[6].parse::<bool>().unwrap(),
                ));
            } else if (results[0] == "crop") {
                crop_vec.push(
                    crop::Crop::new(
                        Rect::new(
                            results[1].parse::<i32>().unwrap() * TILE_SIZE as i32,
                            results[2].parse::<i32>().unwrap() * TILE_SIZE as i32,
                            TILE_SIZE,
                            TILE_SIZE,
                        ),
                        results[3].parse::<u8>().unwrap(),
                        texture_creator
                            .load_texture(results[4])
                            .unwrap(),
                        results[5].parse::<bool>().unwrap(),
                        results[4].parse().unwrap(),
                        results[6].parse::<crop::CropType>().unwrap(),
                    ));
            }
        }
    }


/*    let barn = item::Item::new(
        Rect::new(200, 200, 400, 320),
        texture_creator.load_texture("src/images/Barn.png").unwrap(),
        true,
    );*/

/*    let farmhs = item::Item::new(
        Rect::new(2000, 2000, 400, 320),
        texture_creator
            .load_texture("src/images/house.png")
            .unwrap(),
        true,
    );*/

    // TODO testing crop render with placeholder; remove later
/*    let mut test_crops: Vec<crop::Crop> = vec![
        crop::Crop::new(
            crop::CropType::Carrot,
            Rect::new(
                0 * TILE_SIZE as i32,
                0 * TILE_SIZE as i32,
                TILE_SIZE,
                TILE_SIZE,
            ),
            texture_creator
                .load_texture("src/images/CropPlaceholder.png")
                .unwrap(),
        ),
        crop::Crop::new(
            crop::CropType::Corn,
            Rect::new(
                1 * TILE_SIZE as i32,
                0 * TILE_SIZE as i32,
                TILE_SIZE,
                TILE_SIZE,
            ),
            texture_creator
                .load_texture("src/images/CropPlaceholder.png")
                .unwrap(),
        ),
        crop::Crop::new(
            crop::CropType::Potato,
            Rect::new(
                0 * TILE_SIZE as i32,
                1 * TILE_SIZE as i32,
                TILE_SIZE,
                TILE_SIZE,
            ),
            texture_creator
                .load_texture("src/images/CropPlaceholder.png")
                .unwrap(),
        ),
    ];*/

    // crop 2 should grow, crop 0 should not
/*    crop_vec.get_mut(2).unwrap().set_water(true);
    crop_vec.get_mut(2).unwrap().grow();
    crop_vec.get_mut(0).unwrap().grow();*/
    // TODO remove crop test ^

    // variable for sleep menu
    let mut in_menu = false;
    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {

                    //Iterates through item vector and crop vector saving their positions into a txt file
                    let mut file = match File::create("src/foo.txt") {
                        Err(why) => panic!("couldn't create foo.txt: {}", why),
                        Ok(file) => file,
                    };
                    for item in item_vec {
                        let mut output = "item;".to_owned() + &item.x().to_string() + ";" + &item.y().to_string() + ";" + &item.width().to_string()
                            + ";" + &item.height().to_string() + ";" + &item.tex_path() + ";" + &item.collision().to_string() + "\n";
                        match file.write_all(output.as_ref()) {
                            Err(why) => panic!("couldn't write to foo.txt: {}", why),
                            Ok(_) => println!("successfully wrote item to foo.txt"),
                        }
                    }
                    for crop in crop_vec {
                        let mut output = "crop;".to_owned() + &(crop.getX()/TILE_SIZE as i32).to_string() + ";" + &(crop.getY()/TILE_SIZE as i32).to_string() +
                            ";" + &crop.getStage().to_string() + ";" + &crop.getTex_path() + ";" + &crop.getWatered().to_string() + ";" + &crop.GetCropType() + "\n";
                        match file.write_all(output.as_ref()) {
                            Err(why) => panic!("couldn't write to foo.txt: {}", why),
                            Ok(_) => println!("successfully wrote crop to foo.txt"),
                        }
                    }
                    break 'gameloop
                },
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

        if in_menu {
            if keystate.contains(&Keycode::Y) {
                println!("Yes");
                for c in 0..crop_vec.len() {
                    crop_vec[c].grow();
                }
                in_menu = false;
            }
            if keystate.contains(&Keycode::N) {
                println!("No");
                in_menu = false;
            }
        }

        else {
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

            if keystate.contains(&Keycode::Num1) {
                inventory.set_selected(0);
            }
            if keystate.contains(&Keycode::Num2) {
                inventory.set_selected(1);
            }
            if keystate.contains(&Keycode::Num3) {
                inventory.set_selected(2);
            }
            if keystate.contains(&Keycode::Num4) {
                inventory.set_selected(3);
            }
            if keystate.contains(&Keycode::Num5) {
                inventory.set_selected(4);
            }
            if keystate.contains(&Keycode::Num6) {
                inventory.set_selected(5);
            }
            if keystate.contains(&Keycode::Num7) {
                inventory.set_selected(6);
            }
            if keystate.contains(&Keycode::Num8) {
                inventory.set_selected(7);
            }
            if keystate.contains(&Keycode::Num9) {
                inventory.set_selected(8);
            }
            if keystate.contains(&Keycode::Num0) {
                inventory.set_selected(9);
            }

        }
        
        let player_vel = p.set_speed((x_deltav_f,y_deltav_f));
        p.set_direction(player_vel);

        // Update player position
        // X
        p.update_pos_x(player_vel, (0, (BG_W - TILE_SIZE) as i32));

        for item in &item_vec {
            if check_collision(&p.get_pos(), &item.pos()) { 
                p.stay_still_x(player_vel, (0, (BG_W - TILE_SIZE) as i32));
                if (item.tex_path() == "src/images/house.png") {
                    in_menu = true;

                }

            } 
        }
        /*if check_collision(&p.get_pos(), &farmhs.pos())
            || check_collision(&p.get_pos(), &barn.pos())
        {
            p.stay_still_x(player_vel, (0, (BG_W - TILE_SIZE) as i32));
        }*/

        //Y
        p.update_pos_y(player_vel, (0, (BG_W - TILE_SIZE) as i32));
        for item in &item_vec {
            if check_collision(&p.get_pos(), &item.pos()){
                p.stay_still_y(player_vel, (0, (BG_W - TILE_SIZE) as i32));
                if (item.tex_path() == "src/images/house.png") {
                    in_menu = true;
                }
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
        let player_cam_pos =
            Rect::new(p.x() - cur_bg.x(), p.y() - cur_bg.y(), PLAYER_WIDTH, PLAYER_HEIGHT);

        wincan.set_draw_color(Color::BLACK);
        wincan.clear();

        // Draw tiles
        for croptile in pop.getVec().iter().flatten() {
            let x_pos = croptile.tile.x() - cur_bg.x();
            let y_pos = croptile.tile.y() - cur_bg.y();

            //Don't bother drawing any tiles that are off screen
            if x_pos > -(TILE_SIZE as i32)
                && x_pos < (CAM_W as i32)
                && y_pos > -(TILE_SIZE as i32)
                && y_pos < (CAM_H as i32)
            {
                let cur_tile = Rect::new(
                    croptile.tile.x() - cur_bg.x(),
                    croptile.tile.y() - cur_bg.y(),
                    TILE_SIZE,
                    TILE_SIZE,
                );
                wincan.copy(croptile.tile.texture(), None, cur_tile).unwrap();
            }
        }

        // Drawing item
        for item in &item_vec{
            wincan = item.print_item(cur_bg.x(), cur_bg.y, CAM_W, CAM_H, wincan);
        }



        // TODO crops will probably be stored with the tile grid
        // eventually. Change this to loop over that structure then
        for c in crop_vec.iter() {
            wincan = c.print_crop(cur_bg.x(), cur_bg.y(), wincan);
        }

        // Draw player
        let src = p.src();
        wincan.copy(p.texture(), src, player_cam_pos).unwrap();

        // Draw inventory
        inventory.draw(&mut wincan);
  
        if in_menu {
            
            let sleep_box = texture_creator.load_texture("src/images/sleep.png").unwrap();
            wincan.copy(&sleep_box, None, Rect::new(400, 400, 600, 180)).unwrap();
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
