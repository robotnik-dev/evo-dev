use rand::{Rng, RngCore};

#[derive(Debug)]
pub(crate) struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    pub(crate) fn random(rng: &mut dyn RngCore, input_size: usize) -> Self {
        let bias = rng.random_range(-1.0..=1.0);
        let weights = (0..input_size)
            .map(|_| rng.random_range(-1.0..=1.0))
            .collect();
        Self { bias, weights }
    }

    pub(crate) fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(self.weights.len(), inputs.len());
        let output = self
            .weights
            .iter()
            .zip(inputs)
            .map(|(weight, input)| weight * input)
            .sum::<f32>();

        (output + self.bias).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::*;

    #[test]
    fn random() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);

        approx::assert_relative_eq!(neuron.bias, -0.6255188);
        approx::assert_relative_eq!(
            neuron.weights.as_slice(),
            &[0.67383933, 0.81812596, 0.26284885, 0.5238805].as_ref()
        );
    }

    #[test]
    fn propagate() {
        let neuron = Neuron {
            bias: 0.5,
            weights: vec![-0.3, 0.8],
        };

        assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]), 0.0);
        assert_relative_eq!(
            neuron.propagate(&[0.5, 1.0]),
            (-0.3 * 0.5) + (0.8 * 1.0) + 0.5
        );
    }
}
