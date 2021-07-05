use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use crate::tile::Tile;
use crate::crop::Crop;
use crate::{CAM_H, CAM_W, TILE_SIZE};

//Struct used to combine tile and crop structs into one for easy storage into the vector
pub struct CropTile<'a> {
    pub tile: Tile<'a>,
    pub crop: Crop<'a>,
}

impl<'a> CropTile<'a> {

    pub fn new(tile: Tile<'a>, crop: Crop<'a>) -> CropTile<'a> {
        CropTile {
            tile,
            crop,
        }
    }

}

pub struct Population<'a> {
    CropTile_Vec: Vec< Vec < CropTile<'a> > >,
}

impl<'a> Population<'a> {

    pub fn new(CropTile_Vec: Vec<Vec<CropTile<'a>>>) -> Population {
        Population {
            CropTile_Vec,
        }
    }

    //Lends out the whole vector
    pub fn getVec(&self) -> &Vec< Vec < CropTile > > {
        &self.CropTile_Vec
    }

    //Lends out Tile struct at given x, y map coordinates
    pub fn getTile(&self, x: i32, y: i32) -> &Tile {
        &self.CropTile_Vec[(x/TILE_SIZE as i32) as usize][(y/TILE_SIZE as i32) as usize].tile
    }

    //Lends out Tile struct at given x, y index
    pub fn getTileWithIndex(&self, x: u32, y: u32) -> &Tile {
        &self.CropTile_Vec[x as usize][y as usize].tile
    }

    //Lends out Crop struct at given x, y map coordinates
    pub fn getCrop(&self, x: i32, y: i32) -> &Crop {
        &self.CropTile_Vec[(x/TILE_SIZE as i32) as usize][(y/TILE_SIZE as i32) as usize].crop
    }

    //Lends out Crop struct at given x, y index
    pub fn getCropWithIndex(&self, x: u32, y: u32) -> &Crop {
        &self.CropTile_Vec[x as usize][y as usize].crop
    }

    pub fn updateAllPlants(&self) {

    }

    pub fn plantSeed(&self) {

    }

    pub fn destroyPlant(&self) {

    }
}