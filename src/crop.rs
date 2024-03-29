// Imports
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use std::str::FromStr;

use crate::genes;
use crate::population::Population;
use crate::InventoryItemTrait;

// Import constant from main
use crate::{CAM_H, CAM_W, TILE_SIZE};
// use std::string::ParseError;

use rand::Rng;

/// Crop type enum
#[derive(Copy, Clone, PartialEq, Debug)]

pub enum CropType {
    None,
    Carrot,
    Corn,
    Potato,
    Lettuce,
}

/// Crop struct
pub struct Crop<'a> {
    /// Rectangle to manage crop position.
    pos: Rect,
    /// The stage of growth the crop is in, from
    /// 0 to 3.
    stage: u8,
    /// Rectangle to crop the sprite sheet to the
    /// appropriate tile.
    src: Rect,
    /// Texture of sprite sheet.
    texture: &'a Texture<'a>,
    /// Texture of rotten crop sprite sheet
    rotten_texture: &'a Texture<'a>,
    /// Boolean to hold whether plant has been
    /// watered or not.
    watered: bool,
    t: CropType,
    genes: Option<genes::Genes>,
    pollinated: bool,
    rotten: bool,
    child: Option<genes::Genes>,
}

impl<'a> Crop<'a> {
    /// Creates a new Crop instance.
    ///
    /// Uses the enum `CropType` to select a row on the crop sprite
    /// sheet.
    ///
    /// # Arguments
    /// * `t` - Enum to select type of crop
    /// * `pos` - Position of the crop. Make sure `pos % TILE_SIZE == 0`
    /// * `texture` - Sprite sheet texture
    pub fn new(
        pos: Rect,
        stage: u8,
        texture: &'a Texture<'a>,
        rotten_texture: &'a Texture<'a>,
        watered: bool,
        t: CropType,
        genes: Option<genes::Genes>,
    ) -> Crop<'a> {
        let (x, y) = match t {
            CropType::None => (0, 0),
            CropType::Carrot => (stage as u32 * TILE_SIZE, 0),
            CropType::Corn => (stage as u32 * TILE_SIZE, TILE_SIZE),
            CropType::Potato => (stage as u32 * TILE_SIZE, TILE_SIZE * 2),
            CropType::Lettuce => (stage as u32 * TILE_SIZE, TILE_SIZE * 3),
        };

        let src = Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE);

        Crop {
            pos,
            stage,
            src,
            texture,
            rotten_texture,
            watered,
            t,
            genes,
            pollinated: false,
            rotten: false,
            child: None,
        }
    }

    /// Sets a crop's `watered` variable to `w`
    pub fn set_water(&mut self, w: bool) {
        self.watered = w;
    }

    /// Set the x and y of a crop's `src` Rect
    // pub fn set_src_xy(&mut self, x: i32, y: i32) {
    //     self.src.set_x(x);
    //     self.src.set_y(y);
    // }

    /// Set a crop's texture pointer
    pub fn set_texture(&mut self, t: &'a Texture<'a>) {
        self.texture = t;
    }

    /// Set a crop's `rotten` variable
    pub fn set_rotten(&mut self, r: bool) {
        self.rotten = r;
        if r {
            self.src.set_x(0);
        }
    }

    pub fn rotten(&self) -> bool {
        self.rotten
    }

    /// Checks if a crop has been watered, then increments its
    /// stage of growth, clamping to `0..3`
    pub fn grow(&mut self) {
        if self.get_watered() && self.stage != 3 {
            // Choose random value; if it is less than a crop's
            // growth rate, let it grow
            if let Some(g) = self.get_gene(genes::GeneType::GrowthRate) {
                let mut rng = rand::thread_rng();
                let grow_check: f32 = rng.gen();
                if grow_check < g {
                    self.stage = (self.stage + 1).clamp(0, 3);
                    // Change src from sprite sheet
                    self.src.set_x(self.src.x() + (TILE_SIZE as i32));
                    if let Some(mut w) = self.get_gene(genes::GeneType::WaterRetention) {
                        let mut rng = rand::thread_rng();
                        let watered_check: f32 = rng.gen();
                        w = w / 2.0;
                        if watered_check < w {
                            self.watered = true;
                        } else {
                            // Plant requires more water after growing
                            self.watered = false;
                        }
                    }
                }
            }
        }
    }

    /// Takes ownership of a `WindowCanvas`, checks if the
    /// crop is in frame, and prints it if it is.
    ///
    /// Returns the updated `WindowCanvas`
    ///
    /// # Arguments
    /// * `x` - current x position of camera
    /// * `y` - current y position of camera
    /// * `win` - `WindowCanvas` to be updated
    pub fn print_crop(&self, x: i32, y: i32, mut win: WindowCanvas) -> WindowCanvas {
        let testx = self.get_x() - x;
        let testy = self.get_y() - y;

        if testx > -(self.get_width() as i32)
            && testx < CAM_W as i32
            && testy > -(self.get_height() as i32)
            && testy < CAM_H as i32
        {
            let crop_sub_set = Rect::new(
                self.get_x() - x,
                self.get_y() - y,
                self.get_width(),
                self.get_height(),
            );
            win.copy(self.get_texture(), self.get_src(), crop_sub_set)
                .unwrap();
            return win;
        }
        win
    }

    /// Get the value of a certain gene
    pub fn get_gene(&self, t: genes::GeneType) -> Option<f32> {
        match &self.genes {
            Some(g) => Some(g.get_gene(t)),
            _ => None,
        }
    }

    /// Get all genes; mostly for debugging
    pub fn get_all_genes(&self) -> &Option<genes::Genes> {
        &self.genes
    }

    /// Set a crop's genes
    pub fn set_genes(&mut self, g: Option<genes::Genes>) {
        self.genes = g;
    }

    pub fn set_child(&mut self, c: Option<genes::Genes>) {
        self.child = c;
    }

    pub fn get_child(&self) -> &Option<genes::Genes> {
        &self.child
    }

    /// Get a Crop's texture
    pub fn get_texture(&self) -> &Texture {
        if self.rotten {
            self.rotten_texture
        } else {
            self.texture
        }
    }

    /// Get a Crop's `src`
    pub fn get_src(&self) -> Rect {
        self.src
    }

    /// Get a Crop's position
    pub fn get_pos(&self) -> Rect {
        self.pos
    }

    pub fn set_pos(&mut self, new_pos: Rect) {
        self.pos = new_pos;
    }

    /// Get a Crop's width
    pub fn get_width(&self) -> u32 {
        self.get_pos().width()
    }

    /// Get a Crop's height
    pub fn get_height(&self) -> u32 {
        self.get_pos().height()
    }

    /// Get a Crop's x position
    pub fn get_x(&self) -> i32 {
        self.get_pos().x()
    }

    /// Get a Crop's y position
    pub fn get_y(&self) -> i32 {
        self.get_pos().y()
    }

    /// Get a Crop's watered status
    pub fn get_watered(&self) -> bool {
        self.watered
    }

    // pub fn get_tex_path(&self) -> &String {
    //     &self.tex_path
    // }

    pub fn get_stage(&self) -> u8 {
        self.stage
    }

    pub fn set_stage(&mut self, n: u8) {
        self.stage = n;
    }

    pub fn set_pollinated(&mut self, p: bool) {
        self.pollinated = p;
    }

    pub fn get_crop_type(&self) -> &str {
        match self.t {
            CropType::None => "None",
            CropType::Carrot => "Carrot",
            CropType::Corn => "Corn",
            CropType::Lettuce => "Lettuce",
            CropType::Potato => "Potato",
        }
    }

    pub fn get_crop_type_enum(&self) -> CropType {
        self.t
    }

    pub fn set_crop_type(&mut self, string: &str) {
        match string {
            "None" => self.t = CropType::None,
            "Carrot" => self.t = CropType::Carrot,
            "Corn" => self.t = CropType::Corn,
            "Lettuce" => self.t = CropType::Lettuce,
            "Potato" => self.t = CropType::Potato,
            _ => println!("invalid CropType"),
        };

        let (x, y) = match self.t {
            CropType::None => (0, 0),
            CropType::Carrot => (self.stage as u32 * TILE_SIZE, 0),
            CropType::Corn => (self.stage as u32 * TILE_SIZE, TILE_SIZE),
            CropType::Potato => (self.stage as u32 * TILE_SIZE, TILE_SIZE * 2),
            CropType::Lettuce => (self.stage as u32 * TILE_SIZE, TILE_SIZE * 3),
        };

        self.src = Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE);
    }

    pub fn set_crop_type_enum(&mut self, new_crop_type: CropType) {
        self.t = new_crop_type;
        let (x, y) = match self.t {
            CropType::None => (0, 0),
            CropType::Carrot => (self.stage as u32 * TILE_SIZE, 0),
            CropType::Corn => (self.stage as u32 * TILE_SIZE, TILE_SIZE),
            CropType::Potato => (self.stage as u32 * TILE_SIZE, TILE_SIZE * 2),
            CropType::Lettuce => (self.stage as u32 * TILE_SIZE, TILE_SIZE * 3),
        };

        self.src = Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE);
    }

    pub fn distance(&self, x: i32, y: i32) -> f32 {
        ((((self.get_x() / TILE_SIZE as i32) - x).abs() as f32).powi(2)
            + (((self.get_y() / TILE_SIZE as i32) - y).abs() as f32).powi(2))
        .sqrt()
    }

    // pub fn pollinate(&mut self, pop: &mut Population) {
    pub fn pollinate(&mut self, neighbors: Vec<(genes::Genes, f32)>) {
        // If self is already pollinated, return immediately
        if self.pollinated || self.stage != 3 {
            return;
        }
        // TODO tweak pollination prob
        let mut prob: f32 = 0.4; // Pollination probability
                                 // let x = self.get_x() / TILE_SIZE as i32;
                                 // let y = self.get_y() / TILE_SIZE as i32;
                                 // let neighbors = pop.get_neighbors(x, y);
        let mut rng = rand::thread_rng();
        let mut r: f32;
        // println!(
        //     "Checking pollen at ({}, {}) with {:?}",
        //     self.get_x() / TILE_SIZE as i32,
        //     self.get_y() / TILE_SIZE as i32,
        //     neighbors,
        // );
        for c in neighbors {
            if c.1 > 1.5 {
                // If second ring, use lower probability
                prob = 0.1;
            }
            // Pick a random number
            r = rng.gen();
            // If r < prob, pollinate
            if r < prob {
                println!(
                    "Pollinated at ({}, {})",
                    self.get_x() / TILE_SIZE as i32,
                    self.get_y() / TILE_SIZE as i32
                );
                self.set_pollinated(true);
                self.breed(&c.0);
                return;
            }
        }
    }

    /// Combine the genes of self and another crop to make child genes.
    /// For now, we just take an average and have a chance for mutation
    fn breed(&mut self, g: &genes::Genes) {
        // TODO tune mutation chance
        let mutation: f32 = 0.10; // Percent chance of mutation
        let mut v: Vec<f32> = Vec::new(); // Vector to hold f32 values for child genes
        let mut rng = rand::thread_rng();
        let mut r: f32;

        let types = vec![
            genes::GeneType::GrowthRate,
            genes::GeneType::Value,
            genes::GeneType::WaterRetention,
            genes::GeneType::PestResistance,
        ];

        for t in types {
            let mut cur: f32 = (self.genes.as_ref().unwrap().get_gene(t) + g.get_gene(t)) / 2.0;

            // Check for mutation
            r = rng.gen();
            if r < mutation {
                cur = rng.gen();
            }

            v.push(cur);
        }

        self.set_child(Some(genes::Genes::make_genes(v)));
    }

    /// Generate string to save crop to file
    pub fn to_save_string(&self) -> String {
        let mut s = String::from("crop;");
        s.push_str(((self.get_x() / TILE_SIZE as i32).to_string() + ";").as_ref());
        s.push_str(((self.get_y() / TILE_SIZE as i32).to_string() + ";").as_ref());
        s.push_str(((self.stage).to_string() + ";").as_ref());
        s.push_str(((self.watered).to_string() + ";").as_ref());
        s.push_str(((self.pollinated).to_string() + ";").as_ref());
        s.push_str((self.get_crop_type().to_owned() + ";").as_ref());
        if let Some(g) = self.genes.as_ref() {
            s.push_str(g.to_save_string().as_ref());
        }
        if let Some(c) = self.child.as_ref() {
            s.push_str(c.to_save_string().as_ref());
        } else {
            s.push_str(String::from("None;").as_ref());
        }
        // s.push_str(self.genes.as_ref().unwrap().to_save_string().as_ref());
        s.push('\n');

        s
    }

    /// Load a crop from a save string
    /// Save string format:
    /// 0. "crop" (ignore)
    /// 1. x
    /// 2. y
    /// 3. stage
    /// 4. watered
    /// 5. pollinated
    /// 6. type
    /// 7. growth rate gene
    /// 8. value gene
    /// 9. water retention gene
    /// 10. pest resistance
    /// CHILD
    /// 11. child growth rate / "None" if no child
    /// 12. child value
    /// 13. child water retention
    /// 14. child pest resistance
    pub fn from_save_string(s: &Vec<&str>, t: &'a Texture<'a>, rt: &'a Texture<'a>) -> Crop<'a> {
        let g;
        // println!("Loading from {:?}, len = {:?}", s, s.len());
        // TODO add to this as more genes are added or make from_save_string in Genes

        if s.len() > 8 {
            g = Some(genes::Genes::make_genes(vec![
                s[7].parse::<f32>().unwrap(),
                s[8].parse::<f32>().unwrap(),
                s[9].parse::<f32>().unwrap(),
                s[10].parse::<f32>().unwrap(),
            ]));
        } else {
            g = None;
        }
        let mut c = Crop::new(
            Rect::new(
                s[1].parse::<i32>().unwrap() * TILE_SIZE as i32,
                s[2].parse::<i32>().unwrap() * TILE_SIZE as i32,
                TILE_SIZE,
                TILE_SIZE,
            ),
            s[3].parse::<u8>().unwrap(),
            t,
            rt,
            s[4].parse::<bool>().unwrap(),
            s[6].parse::<CropType>().unwrap(),
            g,
        );
        c.set_pollinated(s[5].parse::<bool>().unwrap());
        if s[11] == "None" {
            c.set_child(None);
        } else {
            c.set_child(Some(genes::Genes::make_genes(vec![
                s[11].parse::<f32>().unwrap(),
                s[12].parse::<f32>().unwrap(),
                s[13].parse::<f32>().unwrap(),
                s[14].parse::<f32>().unwrap(),
            ])));
        }
        c
    }
}

impl InventoryItemTrait for Crop<'_> {
    /// Sort inventory so that you take the best item from the inventory
    /// This can be a combination of factors
    /// i.e. 2*speed + resistance
    fn get_value(&self) -> i32 {
        if let Some(g) = self.get_all_genes() {
            (g.average() * 100 as f32) as i32
        } else {
            0
        }
    }
    fn texture(&self) -> &Texture {
        &self.texture
    }
    fn src(&self) -> Rect {
        self.src
    }
    fn inventory_input(
        &self,
        square: (i32, i32),
        pop: &mut Population,
    ) -> Option<(Option<CropType>, Option<genes::Genes>, Option<genes::Genes>)> {
        if self.stage != 0 {
            return None;
        }
        let (x, y) = square;
        if pop.get_tile_with_index(x as u32, y as u32).tilled()
            && pop
                .get_crop_with_index(x as u32, y as u32)
                .get_crop_type()
                .to_owned()
                == "None"
        {
            let mut _c = pop.get_crop_with_index_mut(x as u32, y as u32);
            _c.set_crop_type_enum(self.t);
            _c.set_stage(0);
            _c.set_water(false);
            _c.set_genes(self.get_all_genes().clone());

            // Return none for right now to signal a crop was placed
            return Some((Some(CropType::None), None, None));
        }
        return None;
    }

    /// Generate string to save crop to file
    fn to_save_string(&self) -> Option<String> {
        // let mut s = String::from("crop;");
        // s.push_str(((self.get_x() / TILE_SIZE as i32).to_string() + ";").as_ref());
        // s.push_str(((self.get_y() / TILE_SIZE as i32).to_string() + ";").as_ref());
        // s.push_str(((self.stage).to_string() + ";").as_ref());
        // s.push_str(((self.watered).to_string() + ";").as_ref());
        // s.push_str(((self.pollinated).to_string() + ";").as_ref());
        // s.push_str((self.get_crop_type().to_owned() + ";").as_ref());
        // if let Some(g) = self.genes.as_ref() {
        //     s.push_str(g.to_save_string().as_ref());
        // }
        // // s.push_str(self.genes.as_ref().unwrap().to_save_string().as_ref());
        // s.push('\n');

        Some(self.to_save_string())
    }
}

impl FromStr for CropType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Carrot" => Ok(CropType::Carrot),
            "Corn" => Ok(CropType::Corn),
            "Lettuce" => Ok(CropType::Lettuce),
            "Potato" => Ok(CropType::Potato),
            _ => Err(()),
        }
    }
}
