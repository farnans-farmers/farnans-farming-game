use crate::pest;
use crate::pest::Pest;
use crate::pest::PestGeneType::AttackRate;
use rand;
use rand::Rng;
use rand_distr::{Distribution, Normal};

const MEAN: f32 = 0.5;
// TODO adjust this value as needed to balance gene generation
const STD_DEV: f32 = 0.1;
pub(crate) const POP_SIZE: usize = 500;

pub struct PestPopulation {
    pest_population: Vec<Pest>,
    avg_attack_chance: f32,
}

impl PestPopulation {
    pub fn new() -> PestPopulation {
        let mut temp = Vec::new();
        let mut z = 0.0;

        PestPopulation {
            pest_population: temp,
            avg_attack_chance: z,
        }
    }

    pub fn get_length(&self) -> usize {
        self.pest_population.len()
    }

    pub fn get_pest(&self, x: usize) -> &Pest {
        &self.pest_population[x]
    }

    pub fn get_avg_attack_chance(&self) -> f32 {
        self.avg_attack_chance
    }

    pub fn fill_pest_population(&mut self) {
        while self.pest_population.len() < POP_SIZE {
            self.pest_population.push(pest::Pest::new());
        }
    }

    pub fn find_avg_attack_chance(&mut self) {
        let mut sum = 0.0;
        for g in 0..POP_SIZE {
            sum = sum + self.pest_population[g].get_pest_gene(AttackRate);
        }
        self.avg_attack_chance = sum / POP_SIZE as f32;
    }

    pub fn add_pest(&mut self, p: Pest) {
        self.pest_population.push(p);
    }

    pub fn next_generation(&mut self) {
        let mut rng = rand::thread_rng();
        let mut temp = Vec::new();
        //print!("outside loop");
        for g in 0..POP_SIZE {
            let n = (self.pest_population[g].get_fitness() * 100.0).round();
            //print!("{}", n);
            for i in 0..n as i32 {
                //print!("loop1");
                let test_pest = self.pest_population[g].clone();
                temp.push(test_pest);
            }
        }
        //print!("{}", temp.len());

        for i in 0..POP_SIZE {
            let pest0: &Pest = temp.get(rng.gen_range(0..temp.len())).unwrap();
            let pest1: &Pest = temp.get(rng.gen_range(0..temp.len())).unwrap();
            let mut pest3 = Pest::make_pest(pest0.breed_pests(pest1));
            pest3.recalc_fitness();
            pest3.mutate_pest();
            self.pest_population[i] = pest3;
        }

        self.find_avg_attack_chance();
    }

    pub fn kill_pest(&mut self, i: usize) {
        self.pest_population.remove(i);
    }
}
