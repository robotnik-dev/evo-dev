use rand::RngCore;

use crate::neuron::Neuron;

#[derive(Debug, Clone)]
pub struct Layer {
    pub(crate) neurons: Vec<Neuron>,
}

impl Layer {
    #[allow(dead_code)]
    pub(crate) fn new(neurons: Vec<Neuron>) -> Self {
        assert!(!neurons.is_empty());

        assert!(
            neurons
                .iter()
                .all(|neuron| neuron.weights.len() == neurons[0].weights.len())
        );

        Self { neurons }
    }
    pub(crate) fn random(rng: &mut dyn RngCore, input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(rng, input_size))
            .collect();
        Self { neurons }
    }

    pub fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();

        Self { neurons }
    }

    pub(crate) fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::*;

    #[test]
    fn random() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let layer = Layer::random(&mut rng, 4, 3);
        let expected_biases: Vec<f32> = vec![-0.6255188, -0.5351684, -0.19277143];

        approx::assert_relative_eq!(
            layer
                .neurons
                .iter()
                .map(|n| n.bias)
                .collect::<Vec<f32>>()
                .as_slice(),
            expected_biases.as_ref()
        );
    }

    #[test]
    fn propagate() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let layer = Layer::random(&mut rng, 4, 3);
        let inputs = vec![0.2, 0.5, 1.0, 0.8];
        let expected = &[0.60026526, 0.0, 0.0];

        approx::assert_relative_eq!(layer.propagate(inputs).as_slice(), expected.as_ref());
    }
}
