use crossover_method::CrossoverMethod;
pub use genotype::*;
pub use individual::Individual;
use mutation_method::MutationMethod;
use rand::RngCore;
use selection_method::SelectionMethod;

use crate::stats::Stats;

pub mod crossover_method;
mod genotype;
mod individual;
pub mod mutation_method;
pub mod selection_method;
pub mod stats;

#[derive(Debug, Clone)]
pub struct GeneticAlgorithm<S, C, M> {
    pub selection_method: S,
    pub crossover_method: C,
    pub mutation_method: M,
}

impl<S, C, M> GeneticAlgorithm<S, C, M>
where
    S: SelectionMethod,
    C: CrossoverMethod,
    M: MutationMethod,
{
    pub fn new(selection_method: S, crossover_method: C, mutation_method: M) -> Self {
        Self {
            selection_method,
            crossover_method,
            mutation_method,
        }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> (Vec<I>, Stats)
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (
            (0..population.len())
                .map(|_| {
                    let parent_a = self.selection_method.select(rng, population).genotype();
                    let parent_b = self.selection_method.select(rng, population).genotype();
                    let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);
                    self.mutation_method.mutate(rng, &mut child);
                    I::create(child)
                })
                .collect(),
            Stats::new(population),
        ) // let stats = Stats::new(new_population);
        // (new_population, stats)
    }
}

#[cfg(test)]
mod tests {
    use crate::genotype::Genotype;

    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    enum TestIndividual {
        WithGenotype { genotype: Genotype },
        WithFitness { fitness: f32 },
    }

    impl TestIndividual {
        fn new(fitness: f32) -> Self {
            Self::WithFitness { fitness }
        }
    }

    impl Individual for TestIndividual {
        fn fitness(&self) -> f32 {
            match self {
                Self::WithFitness { fitness } => *fitness,
                Self::WithGenotype { genotype } => genotype.iter().sum(),
            }
        }

        fn genotype(&self) -> &crate::genotype::Genotype {
            match self {
                Self::WithFitness { .. } => panic!("Not supported for TestIndividual with Fitness"),
                Self::WithGenotype { genotype } => genotype,
            }
        }

        fn create(genotype: Genotype) -> Self {
            Self::WithGenotype { genotype }
        }
    }

    mod evolution {
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        use crate::{
            GeneticAlgorithm, crossover_method::UniformCrossover, individual::Individual,
            mutation_method::GaussianMutation, selection_method::RouletteWheelSelection,
        };

        use super::TestIndividual;

        fn individual(genes: &[f32]) -> TestIndividual {
            TestIndividual::create(genes.iter().cloned().collect())
        }

        #[test]
        fn evolve() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());

            let ga = GeneticAlgorithm::new(
                RouletteWheelSelection,
                UniformCrossover,
                GaussianMutation::new(0.5, 0.5),
            );

            let mut population = vec![
                individual(&[1.0, 1.0, 1.0, 1.0, 1.0]),
                individual(&[1.0, 1.0, 2.0, 2.0, 1.0]),
                individual(&[1.0, 3.0, 3.0, 1.0, 3.0]),
                individual(&[3.0, 4.0, 1.0, 4.0, 2.0]),
            ];

            for _ in 0..10 {
                (population, _) = ga.evolve(&mut rng, &population);
            }

            let expected_population = vec![
                individual(&[2.440106, 3.443046, 1.4234216, 2.4990084, 1.2892904]),
                individual(&[3.052356, 3.2864828, 2.2035873, 3.1747146, 3.6921844]),
                individual(&[2.2522352, 3.1377535, 1.1986562, 2.4990084, 4.1539874]),
                individual(&[2.3075106, 3.6501513, 1.9449277, 1.5516641, 2.8120282]),
            ];

            assert_eq!(expected_population, population);
        }
    }

    mod selection {
        use std::collections::BTreeMap;

        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        use crate::{
            individual::Individual,
            selection_method::{RouletteWheelSelection, SelectionMethod},
            tests::TestIndividual,
        };

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

    mod crossover {
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        use crate::{
            crossover_method::{CrossoverMethod, UniformCrossover},
            genotype::Genotype,
        };

        #[test]
        fn uniform_crossover() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let parent_a: Genotype = (0..=100).map(|i| i as f32).collect();
            let parent_b: Genotype = (0..=100).map(|i| -i as f32).collect();

            let child = UniformCrossover.crossover(&mut rng, &parent_a, &parent_b);
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

    mod mutation {

        mod gaussian_mutation {
            use rand::SeedableRng;
            use rand_chacha::ChaCha8Rng;

            use crate::{
                genotype::Genotype,
                mutation_method::{GaussianMutation, MutationMethod},
            };

            fn actual(chance: f32, coeff: f32) -> Vec<f32> {
                let mut rng = ChaCha8Rng::from_seed(Default::default());
                let mut child = Genotype {
                    genes: vec![1.0, 2.0, 3.0, 4.0, 5.0],
                };
                GaussianMutation::new(chance, coeff).mutate(&mut rng, &mut child);
                child.into_iter().collect()
            }

            mod given_zero_chance {
                fn actual(coeff: f32) -> Vec<f32> {
                    super::actual(0.0, coeff)
                }

                mod and_zero_coeff {
                    use super::actual;

                    #[test]
                    fn does_not_change_the_genotype() {
                        let actual = actual(0.0);
                        let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];
                        approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
                    }
                }
                mod and_nonzero_coeff {
                    use super::actual;

                    #[test]
                    fn does_not_change_the_genotype() {
                        let actual = actual(0.5);
                        let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];
                        approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
                    }
                }
            }
            mod given_fifty_fifty_chance {

                fn actual(coeff: f32) -> Vec<f32> {
                    super::actual(0.5, coeff)
                }

                mod and_zero_coeff {
                    use super::actual;

                    #[test]
                    fn does_not_change_the_genotype() {
                        let actual = actual(0.0);
                        let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];
                        approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
                    }
                }
                mod and_nonzero_coeff {
                    use super::actual;
                    #[test]
                    fn slightly_changes_the_genotype() {
                        let actual = actual(0.5);
                        let expected = vec![1.0, 1.7756249, 3.0, 4.1596804, 5.0];
                        approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
                    }
                }
            }
            mod given_max_chance {

                fn actual(coeff: f32) -> Vec<f32> {
                    super::actual(1.0, coeff)
                }

                mod and_zero_coeff {
                    use super::actual;
                    #[test]
                    fn does_not_change_the_genotype() {
                        let actual = actual(0.0);
                        let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];
                        approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
                    }
                }
                mod and_nonzero_coeff {
                    use super::actual;
                    #[test]
                    fn entirely_changes_the_genotype() {
                        let actual = actual(0.7);
                        let expected = vec![1.6363442, 2.162691, 2.685875, 3.9307175, 4.4941673];
                        approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
                    }
                }
            }
        }
    }
}
