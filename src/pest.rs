use crate::crop::{Crop, CropType};
use crate::genes::GeneType::PestResistance;
use rand;
use rand::Rng;
use rand_distr::{Distribution, Normal};

const MEAN: f32 = 0.5;
const STD_DEV: f32 = 0.1;
const MUTATE_RATE: f32 = 0.02;

pub enum PestGeneType {
    AttackRate,
    BreedSpeed,
    DmgToCarrot,
    DmgToCorn,
    DmgToPotato,
    DmgToLettuce,
}

struct PestGene {
    pest_gene_type: PestGeneType,
    value: f32,
}

impl PestGene {
    fn new(t: PestGeneType, value: f32) -> PestGene {
        PestGene {
            pest_gene_type: t,
            value: value,
        }
    }
}

pub struct Pest {
    pest_genes: Vec<PestGene>,
    fitness: f32,
}

impl Pest {
    pub fn new() -> Pest {
        let normal = Normal::new(MEAN, STD_DEV).unwrap();
        let almost_normal = Normal::new(0.2, STD_DEV).unwrap();
        let attack_rate = almost_normal.sample(&mut rand::thread_rng()).clamp(0.0, 1.0);
        let breed_speed = normal.sample(&mut rand::thread_rng()).clamp(0.0, 1.0);
        let carrot_dmg = normal.sample(&mut rand::thread_rng()).clamp(0.0, 1.0);
        let corn_dmg = normal.sample(&mut rand::thread_rng()).clamp(0.0, 1.0);
        let potato_dmg = normal.sample(&mut rand::thread_rng()).clamp(0.0, 1.0);
        let lettuce_dmg = normal.sample(&mut rand::thread_rng()).clamp(0.0, 1.0);
        let fitness_value =
            (attack_rate + breed_speed + carrot_dmg + corn_dmg + potato_dmg + lettuce_dmg) / 6.0;
        Pest {
            pest_genes: vec![
                PestGene::new(PestGeneType::AttackRate, attack_rate),
                PestGene::new(PestGeneType::BreedSpeed, breed_speed),
                PestGene::new(PestGeneType::DmgToCarrot, carrot_dmg),
                PestGene::new(PestGeneType::DmgToCorn, corn_dmg),
                PestGene::new(PestGeneType::DmgToPotato, potato_dmg),
                PestGene::new(PestGeneType::DmgToLettuce, lettuce_dmg),
            ],
            fitness: fitness_value,
        }
    }

    pub fn get_fitness(&self) -> f32 {
        self.fitness
    }

    /*
       Makes a pest with a f32 vector
       Vector must be arranged in the correct order
       vector[0]: AttackRate
       vector[1]: BreedSpeed
       vector[2]: DmgToCarrot
       vector[3]: DmgToCorn
       vector[4]: DmgToPotato
       vector[5]: DmgToLettuce
       vector[6]: fitness
    */
    pub fn make_pest(v: Vec<f32>) -> Pest {
        Pest {
            pest_genes: vec![
                PestGene::new(PestGeneType::AttackRate, *v.get(0).unwrap()),
                PestGene::new(PestGeneType::BreedSpeed, *v.get(1).unwrap()),
                PestGene::new(PestGeneType::DmgToCarrot, *v.get(2).unwrap()),
                PestGene::new(PestGeneType::DmgToCorn, *v.get(3).unwrap()),
                PestGene::new(PestGeneType::DmgToPotato, *v.get(4).unwrap()),
                PestGene::new(PestGeneType::DmgToLettuce, *v.get(5).unwrap()),
            ],
            fitness: *v.get(6).unwrap(),
        }
    }

    /*
       Breeds two pests and outputs a vector of f32
       Genes are picked entirely randomly from the two parents
       Should be called on parent 1 with parent 2 as an argument
       To turn the f32 vector into a new pest call make_pest()
    */
    pub fn breed_pests(&self, p: &Pest) -> Vec<f32> {
        let mut temp = Vec::new();
        let mut rng = rand::thread_rng();
        let mut sum = 0.0;
        for i in 0..6 {
            let r: i32 = rng.gen_range(0..2);
            if r == 0 {
                temp.push(self.pest_genes.get(i).unwrap().value);
                sum += self.pest_genes.get(i).unwrap().value;
            } else {
                temp.push(p.pest_genes.get(i).unwrap().value);
                sum += self.pest_genes.get(i).unwrap().value;
            }
        }
        temp.push(sum / 6.0);

        temp
    }

    /*
       Goes through the pests genes and mutating them at random
    */
    pub fn mutate_pest(&mut self) {
        let mut rng = rand::thread_rng();

        for i in 0..6 {
            let r: f32 = rng.gen_range(0.0..1.0);
            if r < MUTATE_RATE {
                let normal = Normal::new(self.pest_genes[i].value, STD_DEV).unwrap();
                self.pest_genes[i].value = normal.sample(&mut rand::thread_rng()).clamp(0.0, 1.0);
                self.recalc_fitness();
            }
        }
    }

    /*
       Just recalculates a pests fitness incase of a mutation
    */
    pub fn recalc_fitness(&mut self) {
        let attack_rate = self.pest_genes[0].value;
        let breed_speed = self.pest_genes[1].value;
        let carrot_dmg = self.pest_genes[2].value;
        let corn_dmg = self.pest_genes[3].value;
        let potato_dmg = self.pest_genes[4].value;
        let lettuce_dmg = self.pest_genes[5].value;
        self.fitness =
            (attack_rate + breed_speed + carrot_dmg + corn_dmg + potato_dmg + lettuce_dmg) / 6.0;
    }

    /*
       Just returns the value of a specific gene of this pest
    */
    pub fn get_pest_gene(&self, t: PestGeneType) -> f32 {
        match t {
            PestGeneType::AttackRate => self.pest_genes[0].value,
            PestGeneType::BreedSpeed => self.pest_genes[1].value,
            PestGeneType::DmgToCarrot => self.pest_genes[2].value,
            PestGeneType::DmgToCorn => self.pest_genes[3].value,
            PestGeneType::DmgToPotato => self.pest_genes[4].value,
            PestGeneType::DmgToLettuce => self.pest_genes[5].value,
        }
    }

    /*
       Generates a save string
    */
    pub fn to_save_string(&self) -> String {
        let mut s = String::new();
        for g in &self.pest_genes {
            s.push_str(((g.value).to_string() + ";").as_ref());
        }
        s.push_str((self.fitness.to_string()).as_ref());
        s.push('\n');
        s
    }

    /*
       Creates a new pest from a save string
    */
    pub fn from_save_string(v: Vec<&str>) -> Pest {
        Pest {
            pest_genes: vec![
                PestGene::new(PestGeneType::AttackRate, v[0].parse::<f32>().unwrap()),
                PestGene::new(PestGeneType::BreedSpeed, v[1].parse::<f32>().unwrap()),
                PestGene::new(PestGeneType::DmgToCarrot, v[2].parse::<f32>().unwrap()),
                PestGene::new(PestGeneType::DmgToCorn, v[3].parse::<f32>().unwrap()),
                PestGene::new(PestGeneType::DmgToPotato, v[4].parse::<f32>().unwrap()),
                PestGene::new(PestGeneType::DmgToLettuce, v[5].parse::<f32>().unwrap()),
            ],
            fitness: v[6].parse::<f32>().unwrap(),
        }
    }

    /// Takes in a crop, returns the value of the corresponding attack gene
    pub fn attack_crop(&self, c: &mut Crop) -> f32 {
        let v;
        let a = c.get_crop_type_enum();
        match a {
            CropType::Carrot => v = self.pest_genes[2].value,
            CropType::Corn => v = self.pest_genes[3].value,
            CropType::Potato => v = self.pest_genes[4].value,
            CropType::Lettuce => v = self.pest_genes[5].value,
            _ => v = 0.0,
        }

        return v;
    }

    /*
       Copies this pest
    */
    pub fn clone(&self) -> Pest {
        Pest {
            pest_genes: vec![
                PestGene::new(
                    PestGeneType::AttackRate,
                    self.pest_genes.get(0).unwrap().value,
                ),
                PestGene::new(
                    PestGeneType::BreedSpeed,
                    self.pest_genes.get(1).unwrap().value,
                ),
                PestGene::new(
                    PestGeneType::DmgToCarrot,
                    self.pest_genes.get(2).unwrap().value,
                ),
                PestGene::new(
                    PestGeneType::DmgToCorn,
                    self.pest_genes.get(3).unwrap().value,
                ),
                PestGene::new(
                    PestGeneType::DmgToPotato,
                    self.pest_genes.get(4).unwrap().value,
                ),
                PestGene::new(
                    PestGeneType::DmgToLettuce,
                    self.pest_genes.get(5).unwrap().value,
                ),
            ],
            fitness: self.fitness,
        }
    }
}
