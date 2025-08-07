use crate::individual::Individual;
use rand::{RngCore, seq::IndexedRandom};

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

pub struct RouletteWheelSelection;

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("got an empty population!")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::*;

    struct TestIndividual {
        fitness: f32,
    }

    impl TestIndividual {
        fn new(fitness: f32) -> Self {
            Self { fitness }
        }
    }

    impl Individual for TestIndividual {
        fn fitness(&self) -> f32 {
            self.fitness
        }
    }

    #[test]
    fn roulette_wheel_selection() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let population = vec![
            TestIndividual::new(1.0),
            TestIndividual::new(2.0),
            TestIndividual::new(3.0),
            TestIndividual::new(4.0),
        ];
        let mut actual_histogram = BTreeMap::new();
        for _ in 0..1000 {
            let fitness = RouletteWheelSelection
                .select(&mut rng, &population)
                .fitness() as i32;
            *actual_histogram.entry(fitness).or_insert(0) += 1;
        }
        let exptected_histogram = BTreeMap::from_iter([(1, 102), (2, 198), (3, 301), (4, 399)]);

        assert_eq!(actual_histogram, exptected_histogram);
    }
}
