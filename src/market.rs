use crate::Area;
use crate::Menu;
use crate::BG_H;
use crate::BG_W;
use crate::TILE_SIZE;
use crate::{item, population, Animation, CAM_H, CAM_W};

use crate::player::Player;

use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use sdl2::render::WindowCanvas;

use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use std::time::Instant;
//use anim::Animation;

pub fn start_market_transition_menu<'a>(
    mut in_menu: Option<Menu>,
    wincan: &mut WindowCanvas,
    keystate: HashSet<Keycode>,
    r: Rect,
    in_area: Option<Area>,
) -> (Option<Menu>, Area) {
    let texture_creator = wincan.texture_creator();

    if let Some(Area::Home) = in_area {
        if keystate.contains(&Keycode::Y) {
            // Go to market. First fade to white.
            let alphas: Vec<u8> = (0..=255).collect();
            let dt = Duration::from_secs_f64(2.0 / (alphas.len() as f64));
            let mut blank = Animation::new(alphas, dt, Instant::now());
            blank.set_freezing();
            while blank.current_index() < 255 {
                let tex = texture_creator
                    .load_texture("src/images/traveling_screen.png")
                    .unwrap();
                wincan.copy(&tex, None, None).unwrap();
                wincan.set_draw_color(Color::RGBA(255, 255, 255, *blank.tick()));
                wincan.fill_rect(r).unwrap();
                wincan.present();
                thread::sleep(Duration::from_millis(15));
            }
            // Gone to market.
            in_menu = None;
            return (in_menu, Area::Market);
            //previously the actual changing of the area would occur here, but I have moved that outside of this file and placed it right below the function call.
        } else if keystate.contains(&Keycode::N) {
            in_menu = None;
            return (in_menu, Area::Home);
        } else {
            //allows menu to stay on screen until either y or n is chosen
            return (in_menu, Area::Home);
        }
    } else {
        if keystate.contains(&Keycode::Y) {
            //let texture_creator = wincan.texture_creator();
            // Go home. First fade to white.
            let alphas: Vec<u8> = (0..=255).collect();
            let dt = Duration::from_secs_f64(2.0 / (alphas.len() as f64));
            let mut blank = Animation::new(alphas, dt, Instant::now());
            blank.set_freezing();
            while blank.current_index() < 255 {
                let tex = texture_creator
                    .load_texture("src/images/traveling_screen.png")
                    .unwrap();
                wincan.copy(&tex, None, None).unwrap();
                wincan.set_draw_color(Color::RGBA(255, 255, 255, *blank.tick()));
                wincan.fill_rect(r).unwrap();
                wincan.present();
                thread::sleep(Duration::from_millis(15));
            }
            // Go home.
            in_menu = None;
            return (in_menu, Area::Home);
            //previously the actual changing of the area would occur here, but I have moved that outside of this file and placed it right below the function call.
        } else if keystate.contains(&Keycode::N) {
            in_menu = None;
            return (in_menu, Area::Market);
        } else {
            //allows menu to stay on screen until either y or n is chosen
            return (in_menu, Area::Market);
        }
    }
}

pub fn update_market_pos(
    p: &mut Player,
    m_item_vec: &Vec<item::Item>,
    player_vel: (i32, i32),
    in_menu: &mut Option<Menu>,
) {
    p.update_pos_x(player_vel, (0, (BG_W - TILE_SIZE) as i32));
    for item in m_item_vec {
        if p.check_collision(&item.pos()) {
            p.stay_still_x(player_vel, (0, (BG_W - TILE_SIZE) as i32));
            if item.tex_path() == "src/images/marketstall.png" {
                *in_menu = Some(Menu::Shop);
            } else if item.tex_path() == "src/images/go_home.png" {
                *in_menu = Some(Menu::ToHome)
            }
        }
    }

    //Y
    p.update_pos_y(player_vel, (0, (BG_W - TILE_SIZE) as i32));
    for item in m_item_vec {
        if p.check_collision(&item.pos()) {
            p.stay_still_y(player_vel, (0, (BG_W - TILE_SIZE) as i32));
            if item.tex_path() == "src/images/marketstall.png" {
                *in_menu = Some(Menu::Shop);
            } else if item.tex_path() == "src/images/go_home.png" {
                *in_menu = Some(Menu::ToHome)
            }
        }
    }
}

pub fn background_to_draw(p: &Player) -> Rect {
    Rect::new(
        ((p.x() + ((p.width() / 2) as i32)) - ((CAM_W / 2) as i32)).clamp(0, (BG_W - CAM_W) as i32),
        ((p.y() + ((p.height() / 2) as i32)) - ((CAM_H / 2) as i32))
            .clamp(0, (BG_H - CAM_H) as i32),
        CAM_W,
        CAM_H,
    )
}

pub fn draw_market(
    mut wincan: WindowCanvas,
    m_pop: &population::Population,
    cur_bg: &Rect,
    m_item_vec: &Vec<item::Item>,
) -> WindowCanvas {
    let texture_creator = wincan.texture_creator();

    let grass_texture = texture_creator
        .load_texture("src/images/Background_Tileset.png")
        .unwrap();
    for crop_tile in m_pop.get_vec().iter().flatten() {
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
                .copy(&grass_texture, crop_tile.tile.src(), cur_tile)
                .unwrap();
        }
    }
    // Drawing item
    for item in m_item_vec {
        wincan = item.print_item(cur_bg.x(), cur_bg.y, CAM_W, CAM_H, wincan);
    }

    return wincan;
}
