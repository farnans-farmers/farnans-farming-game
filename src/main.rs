extern crate sdl2;

// Modules
mod player;
mod tile;
mod item;

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

const VSYNC: bool = true;
// Camera dimensions
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
// Background dimensions
const BG_W: u32 = 3000;
const BG_H: u32 = 3000;
const TITLE: &str = "Farnan's Farmers";
pub const TILE_SIZE: u32 = 80;  // Make this public so we can import it elsewhere
const SPEED_LIMIT: i32 = 5;
const ACCEL_RATE: i32 = 1;

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
    // let _ = roll_credits(&mut wincan, texture_creator, r);

    // paths for group images
    // let img1 = "images/jaysonCredits.png";
    // let img2 = "images/JackMCredits.png";
    // let img3 = "images/natCredits.png";
    // let img4 = "images/jacobCredits.png";
    // let img5 = "images/wesleyCredits.png";
    // let img6 = "images/jackACredits.png";
    // let img7 = "images/brandenCredits.png";
    // let images = [img1, img2, img3, img4, img5, img6, img7];

    // // itterate through images and display fade
    // for img in 0..images.len(){
    //     let _ = fade(&mut wincan, texture_creator.load_texture(images[img]).unwrap(), r);
    // }
    // thread::sleep(Duration::from_millis(300));

    // TODO remove background
    let temp_bg = texture_creator.load_texture("images/temp_bg.png").unwrap();
    let mut event_pump = sdl_cxt.event_pump().unwrap();
    let mut x_vel = 0;
    let mut y_vel = 0;

    let mut tile_vec = Vec::new();
    for x in 0..((BG_W/TILE_SIZE) as i32)+1{
        let mut sub_vec = Vec::new(); 
        for y in 0..((BG_H/TILE_SIZE) as i32)+1{
            sub_vec.push(
                tile::Tile::new(
                    Rect::new((TILE_SIZE as i32)*x,(TILE_SIZE as i32)*y,TILE_SIZE,TILE_SIZE),
                    texture_creator.load_texture("images/grass.png").unwrap(),
                )
            );
        }
        tile_vec.push(sub_vec);
    }

    let mut p = player::Player::new(
        Rect::new(
            (BG_W / 2 - TILE_SIZE / 2) as i32,
            (BG_H / 2 - TILE_SIZE / 2) as i32,
            TILE_SIZE,
            TILE_SIZE,
        ),
        texture_creator
            .load_texture("images/placeholder.png")
            .unwrap(),
    );

    let barn = item::Barn::new(
        Rect::new(
            200,
            200,
            400,
            320,
        ),
        texture_creator
            .load_texture("images/Barn.png").unwrap(),
        true,
    );

    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'gameloop,
                _ => {}
            }
        }

        let keystate: HashSet<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let mut x_deltav = 0;
        let mut y_deltav = 0;
        // Change directions using WASD
        if keystate.contains(&Keycode::W) {
            y_deltav -= ACCEL_RATE;
        }
        if keystate.contains(&Keycode::A) {
            x_deltav -= ACCEL_RATE;
        }
        if keystate.contains(&Keycode::S) {
            y_deltav += ACCEL_RATE;
        }
        if keystate.contains(&Keycode::D) {
            x_deltav += ACCEL_RATE;
        }
        // Update player velocity
        x_deltav = resist(x_vel, x_deltav);
        y_deltav = resist(y_vel, y_deltav);
        x_vel = (x_vel + x_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);
        y_vel = (y_vel + y_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);

        // Update player position
        p.update_pos(
            (x_vel, y_vel),
            (0, (BG_W - TILE_SIZE) as i32),
            (0, (BG_H - TILE_SIZE) as i32),
        );

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
            TILE_SIZE,
            TILE_SIZE,
        );

        wincan.set_draw_color(Color::BLACK);
        wincan.clear();

        // Draw tiles
        for tile in tile_vec.iter().flatten(){
            let x_pos = tile.x()-cur_bg.x();
            let y_pos = tile.y()-cur_bg.y();

            //Don't bother drawing any tiles that are off screen
            if x_pos > -(TILE_SIZE as i32) && x_pos < (CAM_W as i32) && y_pos > -(TILE_SIZE as i32) && y_pos < (CAM_H as i32){
                let cur_tile = Rect::new(
                    tile.x()-cur_bg.x(),
                    tile.y()-cur_bg.y(),
                    TILE_SIZE,
                    TILE_SIZE,
                );
                wincan.copy(tile.texture(), None, cur_tile);
            }

        }

        barn.printItem(cur_bg.x(), cur_bg.y, CAM_W, CAM_H, &wincan);
        /*let testx = barn.x() - cur_bg.x();
        let testy = barn.x() - cur_bg.y();
        // Draw barn
        if testx > -(barn.width() as i32) && testx < (CAM_W as i32) &&
        testy > -(barn.height() as i32) && testy < (CAM_W as i32) {
            let barnSubSet = Rect::new(
                barn.x() - cur_bg.x(),
                barn.y() - cur_bg.y(),
                barn.width(),
                barn.height(),
            );
            wincan.copy(barn.texture(), None, barnSubSet);
        }*/

        // Draw player
        wincan.copy(p.texture(), p.src(), player_cam_pos).unwrap();
        wincan.present();
        
    } // end gameloop
}

/**
 * Method to display team credits
 */
fn roll_credits<T>(
    window: &mut WindowCanvas,
    tc: TextureCreator<T>,
    r: Rect,
) -> Result<(), String> {
    // paths for group images
    let img1 = "images/jaysonCredits.png";
    let img2 = "images/JackMCredits.png";
    let img3 = "images/natCredits.png";
    let img4 = "images/jacobCredits.png";
    let img5 = "images/wesleyCredits.png";
    let img6 = "images/jackACredits.png";
    let img7 = "images/brandenCredits.png";
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

fn resist(vel: i32, deltav: i32) -> i32 {
    if deltav == 0 {
        if vel > 0 {
            -1
        } else if vel < 0 {
            1
        } else {
            deltav
        }
    } else {
        deltav
    }
}
