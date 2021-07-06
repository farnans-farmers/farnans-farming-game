use crate::crop::Crop;
use crate::tile::Tile;
use crate::{CAM_H, CAM_W, TILE_SIZE};
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

//Struct used to combine tile and crop structs into one for easy storage into the vector
pub struct Crop_Tile<'a> {
    pub tile: Tile<'a>,
    pub crop: Crop<'a>,
}


impl<'a> Crop_Tile<'a> {

    pub fn new(tile: Tile<'a>, crop: Crop<'a>) -> Crop_Tile<'a> {
        Crop_Tile {
            tile,
            crop,
        }

    }

    pub fn setCrop(&mut self, c: Crop<'a>) {
        self.crop = c;
    }
}

pub struct Population<'a> {

    crop_tile_vec: Vec< Vec < Crop_Tile<'a> > >,
}

impl<'a> Population<'a> {

    pub fn new(crop_tile_vec: Vec<Vec<Crop_Tile<'a>>>) -> Population {
        Population {
            crop_tile_vec,
        }
    }

    //Lends out the whole vector
    pub fn get_vec(&self) -> &Vec< Vec < Crop_Tile > > {
        &self.crop_tile_vec

    }

    pub fn get_vec_mut(&mut self) -> &mut Vec<Vec<Crop_Tile<'a>>> {
        &mut self.crop_tile_vec
    }

    //Lends out Tile struct at given x, y map coordinates

    pub fn get_tile(&self, x: i32, y: i32) -> &Tile {
        &self.crop_tile_vec[(x/TILE_SIZE as i32) as usize][(y/TILE_SIZE as i32) as usize].tile

    }

    //Lends out Tile struct at given x, y index
    pub fn get_tile_with_index(&self, x: u32, y: u32) -> &Tile {
        &self.crop_tile_vec[x as usize][y as usize].tile
    }

    pub fn get_tile_with_index_mut(&mut self, x: u32, y: u32) -> &mut Tile<'a> {
        &mut self.crop_tile_vec[x as usize][y as usize].tile
    }

    //Lends out Crop struct at given x, y map coordinates

    pub fn get_crop(&self, x: i32, y: i32) -> &Crop {
        &self.crop_tile_vec[(x/TILE_SIZE as i32) as usize][(y/TILE_SIZE as i32) as usize].crop

    }

    //Lends out Crop struct at given x, y index
    pub fn get_crop_with_index(&self, x: u32, y: u32) -> &Crop {
        &self.crop_tile_vec[x as usize][y as usize].crop
    }

    pub fn get_crop_with_index_mut(&mut self, x: u32, y: u32) -> &mut Crop<'a> {
        &mut self.crop_tile_vec[x as usize][y as usize].crop
    }

    pub fn update_all_plants(&self) {

    }

    pub fn plant_seed(&self) {

    }

    pub fn destroy_plant(&self) {

    }



}

