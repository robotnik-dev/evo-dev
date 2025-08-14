mod animal;
mod animal_individual;
mod brain;
mod eye;
mod food;
mod world;

pub use animal::*;
use animal_individual::*;
use eye::*;
pub use food::*;
pub use genetic_algorithm::{
    self as ga, crossover_method::UniformCrossover, mutation_method::GaussianMutation,
    selection_method::RouletteWheelSelection, stats::Stats,
};
use nalgebra::{Rotation2, wrap};
use neural_network as nn;
use rand::{Rng, RngCore};
pub use world::*;

const GENERATION_LENGTH: usize = 2500;

#[derive(Debug)]
pub struct Simulation {
    world: World,
    ga: ga::GeneticAlgorithm<RouletteWheelSelection, UniformCrossover, GaussianMutation>,
    pub age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng);
        let ga = ga::GeneticAlgorithm::new(
            RouletteWheelSelection,
            UniformCrossover,
            GaussianMutation::new(0.01, 0.3),
        );
        Self { world, ga, age: 0 }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<Stats> {
        self.handle_collision(rng);
        self.process_brains();
        self.handle_movement();

        self.age += 1;
        if self.age > GENERATION_LENGTH {
            Some(self.evolve(rng))
        } else {
            None
        }
    }

    pub fn train(&mut self, rng: &mut dyn RngCore) -> Stats {
        loop {
            if let Some(stats) = self.step(rng) {
                return stats;
            }
        }
    }
    fn evolve(&mut self, rng: &mut dyn RngCore) -> Stats {
        self.age = 0;
        let current_population: Vec<_> = self
            .world
            .animals
            .iter()
            .map(AnimalIndividual::from_animal)
            .collect();

        let (evolved_population, stats) = self.ga.evolve(rng, &current_population);

        self.world.animals = evolved_population
            .into_iter()
            .map(|individual| individual.into_animal(rng))
            .collect();

        for food in &mut self.world.foods {
            food.position = rng.random();
        }
        stats
    }
    pub fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            let vision =
                animal
                    .eye
                    .process_vision(animal.position(), animal.rotation(), &self.world.foods);
            let output = animal.brain.propagate(vision);
            let speed = output[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = output[1].clamp(-ROT_ACCEL, ROT_ACCEL);

            animal.speed = (animal.speed + speed).clamp(-SPEED_MIN, SPEED_MAX);
            animal.rotation = Rotation2::new(animal.rotation.angle() + rotation);
        }
    }

    pub fn handle_movement(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * nalgebra::Vector2::new(0.0, animal.speed);
            animal.position.x = wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = wrap(animal.position.y, 0.0, 1.0);
        }
    }

    pub fn handle_collision(&mut self, rng: &mut dyn RngCore) {
        for food in &mut self.world.foods {
            for animal in &mut self.world.animals {
                let col_margin = 0.01;
                let collision = nalgebra::distance(&food.position(), &animal.position())
                    <= animal.size() + col_margin;
                if collision {
                    animal.eaten += 1;
                    food.position = rng.random();
                }
            }
        }
    }
}
