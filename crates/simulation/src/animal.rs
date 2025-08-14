use std::f32::consts::FRAC_PI_2;

use crate::{brain::Brain, eye::Eye};
use genetic_algorithm::Genotype;
use nalgebra as na;
use rand::{Rng, RngCore};

pub(crate) const SPEED_MIN: f32 = 0.001;
pub(crate) const SPEED_MAX: f32 = 0.005;
pub(crate) const SPEED_ACCEL: f32 = 0.2;
pub(crate) const ROT_ACCEL: f32 = FRAC_PI_2;

#[derive(Debug)]
pub struct Animal {
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) size: f32,
    pub(crate) eye: Eye,
    pub(crate) brain: Brain,
    pub(crate) eaten: usize,
}

impl Animal {
    pub fn new(eye: Eye, brain: Brain, rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.random(),
            rotation: rng.random(),
            speed: 0.002,
            size: rng.random_range(0.005..0.01),
            eye,
            brain,
            eaten: 0,
        }
    }
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::random(rng, &eye);
        Self::new(eye, brain, rng)
    }

    pub(crate) fn from_genotype(genotype: Genotype, rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::from_genotype(genotype, &eye);

        Self::new(eye, brain, rng)
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
    pub fn size(&self) -> f32 {
        self.size
    }
}
