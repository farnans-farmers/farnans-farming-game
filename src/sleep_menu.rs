use crate::player::Player;
// Module for sleeping menu and code.
use crate::population::Population;
use crate::Menu;
use crate::BG_H;
use crate::BG_W;
use crate::TILE_SIZE;

use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use sdl2::render::WindowCanvas;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;

//Imported to see if it's a bug night or not
use rand::Rng;

pub fn start_sleep_menu(
    mut in_menu: Option<Menu>,
    wincan: &mut WindowCanvas,
    keystate: HashSet<Keycode>,
    player: &mut Player,
    pop: &mut Population,
    r: Rect,
) -> Option<Menu> {
    let texture_creator = wincan.texture_creator();
    if keystate.contains(&Keycode::Y) {
        //Player has selected yes

        //Generate a random number between 1 and 5 (inclusive). a 5 is a bug night.
        let mut rng = rand::thread_rng();
        let bug_night_result = rng.gen_range(0..6);

        //Cut to black and then fade into night scene
        if bug_night_result != 5 {
            let mut i = 0;
            while i < 254 {
                wincan
                    .copy(
                        &texture_creator
                            .load_texture("src/images/sleeping_screen.png")
                            .unwrap(),
                        None,
                        None,
                    )
                    .unwrap();
                wincan.set_draw_color(Color::RGBA(0, 0, 0, 255 - i));
                wincan.fill_rect(r).unwrap();
                wincan.present();
                thread::sleep(Duration::from_millis(1));
                i = i + 2;
            }
        } else {
            let mut i = 0;
            while i < 254 {
                wincan
                    .copy(
                        &texture_creator
                            .load_texture("src/images/sleeping_screen_spiders.png")
                            .unwrap(),
                        None,
                        None,
                    )
                    .unwrap();
                wincan.set_draw_color(Color::RGBA(0, 0, 0, 255 - i));
                wincan.fill_rect(r).unwrap();
                wincan.present();
                thread::sleep(Duration::from_millis(1));
                i = i + 2;
            }
        }

        //The fading code is ripped out of the method because I wanted
        // the growing to happen while the player could not see the screen.
        // Grow crops
        for _x in 0..((BG_W / TILE_SIZE) as i32 + 1) {
            for _y in 0..((BG_H / TILE_SIZE) as i32 + 1) {
                if bug_night_result == 5 {}

                let mut _c = pop.get_crop_with_index_mut(_x as u32, _y as u32);
                match _c.get_crop_type() {
                    "None" => {}
                    _ => {
                        _c.grow();
                    }
                }
                //_c.set_water(false);
                // Set tile watered to false
                if pop
                    .get_crop_with_index_mut(_x as u32, _y as u32)
                    .get_watered()
                    == false
                {
                    pop.get_tile_with_index_mut(_x as u32, _y as u32)
                        .set_water(false);
                } else {
                    pop.get_tile_with_index_mut(_x as u32, _y as u32)
                        .set_water(true);
                }
            }
        }

        // Eat dinner.
        let hunger = player.dinner();
        if hunger > 0 {
            println!("how sad, your are still {} hungery", hunger)
        }

        // fade to white because the sun is coming up
        if bug_night_result != 5 {
            let mut i = 0;
            while i < 254 {
                wincan
                    .copy(
                        &texture_creator
                            .load_texture("src/images/sleeping_screen.png")
                            .unwrap(),
                        None,
                        None,
                    )
                    .unwrap();
                wincan.set_draw_color(Color::RGBA(255, 255, 255, i));
                wincan.fill_rect(r).unwrap();
                wincan.present();
                thread::sleep(Duration::from_millis(1));
                i = i + 2;
            }
        } else {
            let mut i = 0;
            while i < 254 {
                wincan
                    .copy(
                        &texture_creator
                            .load_texture("src/images/sleeping_screen_spiders.png")
                            .unwrap(),
                        None,
                        None,
                    )
                    .unwrap();
                wincan.set_draw_color(Color::RGBA(255, 255, 255, i));
                wincan.fill_rect(r).unwrap();
                wincan.present();
                thread::sleep(Duration::from_millis(1));
                i = i + 2;
            }
        }

        in_menu = None;
        return in_menu;
    } else if keystate.contains(&Keycode::N) {
        //Player has chosen not to sleep
        in_menu = None;
        return in_menu;
    } else {
        return in_menu;
    }
}
