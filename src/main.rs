extern crate sdl2;

use std::time::Duration;
use std::thread;

use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;

use sdl2::render::WindowCanvas;

use sdl2::render::BlendMode;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;

const VSYNC: bool = true;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const TITLE: &str = "Farnan's Farmers";

fn main() {
    
    let sdl_cxt = sdl2::init().unwrap();
    let video_subsys = sdl_cxt.video().unwrap();

    let window = video_subsys.window(TITLE, WIDTH, HEIGHT)
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let wincan = window.into_canvas().accelerated();

    // Check if we should lock to vsync
    let wincan = if VSYNC {
        wincan.present_vsync()
    }
    else {
        wincan
    };
    
    let mut wincan = wincan.build()
        .map_err(|e| e.to_string())
        .unwrap();


    wincan.set_blend_mode(BlendMode::Blend);
    let texture_creator = wincan.texture_creator();
    let r = Rect::new((0) as i32, (0) as i32, WIDTH, HEIGHT);
    wincan.set_draw_color(Color::RGBA(255, 255, 255, 255));
    wincan.clear();

    // Roll group credits
    let _ = roll_credits(&mut wincan, texture_creator, r);

    // paths for group images
    // let img1 = "src/images/jaysonCredits.png";
    // let img2 = "src/images/JackMCredits.png";
    // let img3 = "src/images/natCredits.png";
    // let img4 = "src/images/jacobCredits.png";
    // let img5 = "src/images/wesleyCredits.png";
    // let img6 = "src/images/jackACredits.png";
    // let img7 = "src/images/brandenCredits.png";
    // let images = [img1, img2, img3, img4, img5, img6, img7];

    // // itterate through images and display fade 
    // for img in 0..images.len(){
    //     let _ = fade(&mut wincan, texture_creator.load_texture(images[img]).unwrap(), r);
    // }
    // thread::sleep(Duration::from_millis(300));

    
}

/**
 * Method to display team credits
 */
fn roll_credits<T>(window: &mut WindowCanvas, tc: TextureCreator<T>, r: Rect) -> Result<(), String> {
    // paths for group images
    let img1 = "src/images/jaysonCredits.png";
    let img2 = "src/images/JackMCredits.png";
    let img3 = "src/images/natCredits.png";
    let img4 = "src/images/jacobCredits.png";
    let img5 = "src/images/wesleyCredits.png";
    let img6 = "src/images/jackACredits.png";
    let img7 = "src/images/brandenCredits.png";
    let images = [img1, img2, img3, img4, img5, img6, img7];

    // Iterate through images; fade in and out
    for img in 0..images.len() {
        let _ = fade(window, tc.load_texture(images[img]).unwrap(), r);
    }

    Ok(())

}


// method to fade in and out 
fn fade(window: &mut WindowCanvas, ms: Texture, r:  Rect) -> Result<(), String> {
	// fade in 
	let mut i = 0;
	while i < 254 {
		window.clear();
		window.copy(&ms, None, None)?;
		window.set_draw_color(Color::RGBA(255, 255, 255, 255-i));
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

