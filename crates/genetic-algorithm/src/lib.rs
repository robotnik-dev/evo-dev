use individual::Individual;
use rand::RngCore;
use selection_method::SelectionMethod;

mod individual;
mod selection_method;

#[derive(Debug)]
pub struct GeneticAlgorithm<S> {
    pub selection_method: S,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(selection_method: S) -> Self {
        Self { selection_method }
    }

    pub fn process<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method.select(rng, population);
                let parent_b = self.selection_method.select(rng, population);
                // crossover
                // mutation
                todo!()
            })
            .collect()
    }
}
