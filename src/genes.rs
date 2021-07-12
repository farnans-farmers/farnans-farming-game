// Imports
use rand;
use rand_distr::{Distribution, Normal};

const MEAN: f32 = 0.5;
// TODO adjust this value as needed to balance gene generation
const STD_DEV: f32 = 0.2;

/// Gene type enum
pub enum GeneType {
    GrowthRate,
    Value,
}

struct Gene {
    gene_type: GeneType,
    value: f32,
}

/// Genes struct
pub struct Genes {
    genes: Vec<Gene>,
}

impl Genes {
    /// Generate new Genes using random values following a
    /// Normal Distribution
    pub fn new() -> Genes {
        let normal = Normal::new(MEAN, STD_DEV).unwrap();
        Genes {
            genes: vec![
                Gene::new(GeneType::GrowthRate, normal.sample(&mut rand::thread_rng())),
                Gene::new(GeneType::Value, normal.sample(&mut rand::thread_rng())),
            ],
        }
    }

    pub fn get_gene(&self, t: GeneType) -> f32 {
        match t {
            GeneType::GrowthRate => self.genes.get(0).unwrap().value,
            GeneType::Value => self.genes.get(1).unwrap().value,
        }
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
        }
    }
}

impl std::fmt::Display for Gene {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GeneType: {}, value: {}", self.gene_type, self.value)
    }
}

// impl std::fmt::Display for Vec<Gene> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

//     }
// }

impl std::fmt::Display for Genes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.genes.iter().fold(Ok(()), |result, gene| {
            result.and_then(|_| writeln!(f, "{}", gene))
        })
    }
}
