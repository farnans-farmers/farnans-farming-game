use crate::Animation;
use crate::Area;
use crate::Menu;
use crate::BG_H;
use crate::BG_W;
use crate::TILE_SIZE;

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
//use anim::Animation;

pub fn start_market_transition_menu(
    mut in_menu: Option<Menu>,
    wincan: &mut WindowCanvas,
    keystate: HashSet<Keycode>,
    r: Rect,
    mut in_area: Option<Area>,
) -> (Option<Menu>, Area) {
    let texture_creator = wincan.texture_creator();
    if keystate.contains(&Keycode::Y) {
        //let texture_creator = wincan.texture_creator();
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
}
