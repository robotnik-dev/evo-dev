use rand::{Rng, RngCore};

use crate::genotype::Genotype;

pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Genotype,
        parent_b: &Genotype,
    ) -> Genotype;
}

#[derive(Debug, Clone)]
pub struct UniformCrossover;

impl CrossoverMethod for UniformCrossover {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Genotype,
        parent_b: &Genotype,
    ) -> Genotype {
        assert_eq!(parent_a.len(), parent_b.len());

        parent_a
            .iter()
            .zip(parent_b.iter())
            .map(|(&a, &b)| if rng.random_bool(0.5) { a } else { b })
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::*;

    #[test]
    fn uniform_crossover() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let parent_a = &(0..=100).map(|i| i as f32).collect();
        let parent_b = &(0..=100).map(|i| -i as f32).collect();

        let child = UniformCrossover.crossover(&mut rng, parent_a, parent_b);
        // Number of genes different between `child` and `parent_a`
        let diff_a = parent_a
            .iter()
            .zip(child.iter())
            .filter(|(p, c)| p != c)
            .count();
        // Number of genes different between `child` and `parent_b`
        let diff_b = parent_b
            .iter()
            .zip(child.iter())
            .filter(|(p, c)| p != c)
            .count();
        assert!(diff_a + diff_b == 100);
        assert_eq!(diff_a, 49);
        assert_eq!(diff_b, 51);
    }
}
