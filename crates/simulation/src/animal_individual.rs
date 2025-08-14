use crate::*;

pub struct AnimalIndividual {
    fitness: f32,
    genotype: ga::Genotype,
}

impl ga::Individual for AnimalIndividual {
    fn create(genotype: ga::Genotype) -> Self {
        Self {
            fitness: 0.0,
            genotype,
        }
    }

    fn genotype(&self) -> &ga::Genotype {
        &self.genotype
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}

impl AnimalIndividual {
    pub fn from_animal(animal: &Animal) -> Self {
        Self {
            fitness: animal.eaten as f32,
            genotype: animal.brain.as_genotype(),
        }
    }

    pub fn into_animal(self, rng: &mut dyn RngCore) -> Animal {
        Animal::from_genotype(self.genotype, rng)
    }
}
