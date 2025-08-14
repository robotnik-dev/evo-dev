use rand::{Rng, RngCore};

use crate::genotype::Genotype;

pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Genotype);
}

#[derive(Debug, Clone)]
pub struct GaussianMutation {
    /// Probability of changing a gene
    /// - 0.0 no gene will be touched
    /// - 1.0 all genes will be touched
    chance: f32,

    /// Magnitude of that change
    /// - 0.0
    /// - 3.0 touched genes will be +-= coeff changed
    coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!((0.0..=1.0).contains(&chance));

        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Genotype) {
        for gene in child.iter_mut() {
            let sign = if rng.random_bool(0.5) { -1.0 } else { 1.0 };

            if rng.random_bool(self.chance as f64) {
                *gene += sign * self.coeff * rng.random::<f32>();
            }
        }
    }
}

