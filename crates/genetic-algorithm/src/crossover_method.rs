use rand::RngCore;

use crate::genotype::Genotype;

pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Genotype,
        parent_b: &Genotype,
    ) -> Genotype;
}

pub struct UniformCrossover;

impl CrossoverMethod for UniformCrossover {
    fn crossover(
            &self,
            rng: &mut dyn RngCore,
            parent_a: &Genotype,
            parent_b: &Genotype,
        ) -> Genotype {
        todo!()
    }
}
