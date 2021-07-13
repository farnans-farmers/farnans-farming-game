use crate::{crop, item, population, tile, BG_H, BG_W, TILE_SIZE};
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use std::fs::File;
use std::io::{Read, Write};

pub fn load_market<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
) -> (population::Population<'a>, Vec<item::Item<'a>>) {
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
                    crop::CropType::None,
                    None,
                ),
            ));
        }
        tile_vec.push(sub_vec);
    }

    let mut pop = population::Population::new(tile_vec);
    let mut market_item_vec = Vec::new();
    let mut market_file = File::open("src/market_data.txt").expect("Can't open save market_file");
    let mut market_contents = String::new();

    market_file
        .read_to_string(&mut market_contents)
        .expect("Can't read market_file");
    print!("{}", market_contents);
    for line in market_contents.lines() {
        let results: Vec<&str> = line.split(";").collect();
        if (results[0] == "item") {
            market_item_vec.push(item::Item::new(
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
        }
    }
    return (pop, market_item_vec);
}

pub fn load_home<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
) -> (population::Population<'a>, Vec<item::Item<'a>>) {
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
                    crop::CropType::None,
                    None,
                ),
            ));
        }
        tile_vec.push(sub_vec);
    }
    let mut pop = population::Population::new(tile_vec);

    let mut home_item_vec = Vec::new();
    //let mut crop_vec: Vec<crop::Crop> = Vec::new();

    {
        let mut home_file = File::open("src/home_data.txt").expect("Can't open save home_file");
        let mut home_contents = String::new();
        home_file
            .read_to_string(&mut home_contents)
            .expect("Can't read home_file");
        print!("{}", home_contents);
        for line in home_contents.lines() {
            let results: Vec<&str> = line.split(";").collect();
            if (results[0] == "item") {
                home_item_vec.push(item::Item::new(
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
                    .setCrop(crop::Crop::from_save_string(
                        &results,
                        texture_creator
                            .load_texture("src/images/Crop_Tileset.png")
                            .unwrap(),
                    ));
                // If crop is present, set tile as tilled
                if results[5]
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
    return (pop, home_item_vec);
}

// TODO try saving via serialization
pub fn save_home(pop: population::Population, item_vec: Vec<item::Item>) {
    let mut file_to_save = match File::create("src/home_data.txt") {
        Err(why) => panic!("couldn't create home_data.txt: {}", why),
        Ok(file_to_save) => file_to_save,
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
        match file_to_save.write_all(output.as_ref()) {
            Err(why) => panic!("couldn't write to home_data.txt: {}", why),
            Ok(_) => println!("successfully wrote item to home_data.txt"),
        }
    }

    for _x in 0..((BG_W / TILE_SIZE) as i32 + 1) {
        for _y in 0..((BG_H / TILE_SIZE) as i32 + 1) {
            let _c = pop.get_crop_with_index(_x as u32, _y as u32);
            match _c.get_crop_type() {
                "None" => {}
                _ => {
                    // let output = "crop;".to_owned()
                    //     + &(_c.get_x() / TILE_SIZE as i32).to_string()
                    //     + ";"
                    //     + &(_c.get_y() / TILE_SIZE as i32).to_string()
                    //     + ";"
                    //     + &_c.get_stage().to_string()
                    //     + ";"
                    //     + &_c.get_tex_path()
                    //     + ";"
                    //     + &_c.get_watered().to_string()
                    //     + ";"
                    //     + &_c.get_crop_type()
                    //     + "\n";
                    let output = _c.to_save_string();
                    match file_to_save.write_all(output.as_ref()) {
                        Err(why) => {
                            panic!("couldn't write to home_data.txt: {}", why)
                        }
                        Ok(_) => {
                            println!("successfully wrote crop to home_data.txt")
                        }
                    }
                }
            }
        }
    }
}
