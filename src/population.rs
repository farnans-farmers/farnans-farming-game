use crate::crop::Crop;
use crate::genes;
use crate::tile::Tile;
use crate::{BOTTOM_TILE_BOUND, RIGHT_TILE_BOUND, TILE_SIZE};

//Struct used to combine tile and crop structs into one for easy storage into the vector
pub struct CropTile<'a> {
    pub tile: Tile<'a>,
    pub crop: Crop<'a>,
}

impl<'a> CropTile<'a> {
    pub fn new(tile: Tile<'a>, crop: Crop<'a>) -> CropTile<'a> {
        CropTile { tile, crop }
    }

    pub fn set_crop(&mut self, c: Crop<'a>) {
        self.crop = c;
    }
}

pub struct Population<'a> {
    crop_tile_vec: Vec<Vec<CropTile<'a>>>,
}

impl<'a> Population<'a> {
    pub fn new(crop_tile_vec: Vec<Vec<CropTile<'a>>>) -> Population<'a> {
        Population { crop_tile_vec }
    }

    //Lends out the whole vector
    pub fn get_vec(&self) -> &Vec<Vec<CropTile>> {
        &self.crop_tile_vec
    }

    pub fn get_vec_mut(&mut self) -> &mut Vec<Vec<CropTile<'a>>> {
        &mut self.crop_tile_vec
    }

    //Lends out Tile struct at given x, y map coordinates

    pub fn get_tile(&self, x: i32, y: i32) -> &Tile {
        &self.crop_tile_vec[(x / TILE_SIZE as i32) as usize][(y / TILE_SIZE as i32) as usize].tile
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
        &self.crop_tile_vec[(x / TILE_SIZE as i32) as usize][(y / TILE_SIZE as i32) as usize].crop
    }

    //Lends out Crop struct at given x, y index
    pub fn get_crop_with_index(&self, x: u32, y: u32) -> &Crop {
        &self.crop_tile_vec[x as usize][y as usize].crop
    }

    pub fn get_crop_with_index_mut(&mut self, x: u32, y: u32) -> &mut Crop<'a> {
        &mut self.crop_tile_vec[x as usize][y as usize].crop
    }

    pub fn set_crop_with_index(&mut self, x: u32, y: u32, mut tar_crop: Crop<'a>) {
        tar_crop.set_pos(self.crop_tile_vec[x as usize][y as usize].crop.get_pos());
        self.crop_tile_vec[x as usize][y as usize].crop = tar_crop;
    }

    pub fn update_all_plants(&self) {}

    pub fn plant_seed(&self) {}

    pub fn destroy_plant(&self) {}

    /// Returns an array of neighboring crops, sorted by distance from
    /// (x,y)
    pub fn get_neighbors(&self, x: i32, y: i32) -> Vec<(genes::Genes, f32)> {
        let mut v: Vec<&Crop> = Vec::new();
        // Loop through nearest rings
        for col in (x - 2).clamp(0, RIGHT_TILE_BOUND)..(x + 2).clamp(0, RIGHT_TILE_BOUND) {
            for row in (y - 2).clamp(0, BOTTOM_TILE_BOUND)..(y + 2).clamp(0, BOTTOM_TILE_BOUND) {
                // Don't let a plant pollinate itself
                if col == x && row == y {
                    continue;
                }
                let c = self.get_crop_with_index(col as u32, row as u32);
                if c.get_crop_type_enum() != crate::crop::CropType::None && c.get_stage() == 3 {
                    v.push(c);
                }
            }
        }
        // Sort vector
        v.sort_by_cached_key(|k| (k.distance(x, y) * 100.0) as i32);
        // Extract clones of genes and distances
        let mut r: Vec<(genes::Genes, f32)> = Vec::new();
        for i in v {
            r.push((
                i.get_all_genes().as_ref().unwrap().clone(),
                i.distance(x, y),
            ));
        }
        r
    }

    // pub fn pollinate(&self, x: i32, y: i32) {
    //     // let mut c =
    // }
}
