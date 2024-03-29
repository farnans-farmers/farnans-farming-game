// Imports
use rand;
use rand_distr::{Distribution, Normal};

const MEAN: f32 = 0.5;
// TODO adjust this value as needed to balance gene generation
const STD_DEV: f32 = 0.1;

/// Gene type enum
#[derive(Copy, Clone, Debug)]
pub enum GeneType {
    GrowthRate,
    Value,
    WaterRetention,
    PestResistance,
}

#[derive(Debug)]
struct Gene {
    gene_type: GeneType,
    value: f32,
}

/// Genes struct
#[derive(Debug)]
pub struct Genes {
    genes: Vec<Gene>,
}

impl Genes {
    /// Generate new Genes using random values following a
    /// Normal Distribution
    pub fn new() -> Genes {
        let normal = Normal::new(MEAN, STD_DEV).unwrap();
        let growth_var = normal.sample(&mut rand::thread_rng()).clamp(0.0, 1.0);
        let water_ret_var = normal.sample(&mut rand::thread_rng()).clamp(0.0, 1.0);
        let pest_resist_var = normal.sample(&mut rand::thread_rng()).clamp(0.0, 1.0);
        let value_var = (growth_var + water_ret_var + pest_resist_var) / 3.0;
        Genes {
            genes: vec![
                Gene::new(GeneType::GrowthRate, growth_var),
                Gene::new(GeneType::Value, value_var),
                Gene::new(GeneType::WaterRetention, water_ret_var),
                Gene::new(GeneType::PestResistance, pest_resist_var),
            ],
        }
    }

    pub fn make_genes(v: Vec<f32>) -> Genes {
        Genes {
            genes: vec![
                Gene::new(GeneType::GrowthRate, *v.get(0).unwrap()),
                Gene::new(GeneType::Value, *v.get(1).unwrap()),
                Gene::new(GeneType::WaterRetention, *v.get(2).unwrap()),
                Gene::new(GeneType::PestResistance, *v.get(3).unwrap()),
            ],
        }
    }

    /// Get the value of a specific gene
    pub fn get_gene(&self, t: GeneType) -> f32 {
        match t {
            GeneType::GrowthRate => self.genes.get(0).unwrap().value,
            GeneType::Value => self.genes.get(1).unwrap().value,
            GeneType::WaterRetention => self.genes.get(2).unwrap().value,
            GeneType::PestResistance => self.genes.get(3).unwrap().value,
        }
    }

    pub fn average(&self) -> f32 {
        let mut sum = 0.0;
        let mut count = 0;
        for g in &self.genes {
            sum += g.value;
            count += 1;
        }
        sum / (count as f32)
    }

    pub fn to_save_string(&self) -> String {
        let mut s = String::new();
        for g in &self.genes {
            s.push_str(((g.value).to_string() + ";").as_ref());
        }
        s
    }

    pub fn num_genes(&self) -> usize {
        self.genes.len()
    }
}

impl Gene {
    fn new(t: GeneType, value: f32) -> Gene {
        Gene {
            gene_type: t,
            value: value,
        }
    }
}

impl std::fmt::Display for GeneType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GeneType::GrowthRate => write!(f, "GrowthRate"),
            GeneType::Value => write!(f, "Value"),
            GeneType::WaterRetention => write!(f, "WaterRetention"),
            GeneType::PestResistance => write!(f, "PestResistance"),
        }
    }
}

impl std::fmt::Display for Gene {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GeneType: {}, value: {}", self.gene_type, self.value)
    }
}

impl std::fmt::Display for Genes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.genes.iter().fold(Ok(()), |result, gene| {
            result.and_then(|_| writeln!(f, "{}", gene))
        })
    }
}

impl std::clone::Clone for Genes {
    fn clone(&self) -> Genes {
        Genes {
            genes: vec![
                Gene::new(GeneType::GrowthRate, self.get_gene(GeneType::GrowthRate)),
                Gene::new(GeneType::Value, self.get_gene(GeneType::Value)),
                Gene::new(
                    GeneType::WaterRetention,
                    self.get_gene(GeneType::WaterRetention),
                ),
                Gene::new(
                    GeneType::PestResistance,
                    self.get_gene(GeneType::PestResistance),
                ),
            ],
        }
    }
}
