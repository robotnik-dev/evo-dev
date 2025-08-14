mod layer;
mod layer_topology;
mod neuron;

use std::iter::once;

pub use self::{layer::Layer, layer_topology::*};
use rand::RngCore;

#[derive(Debug, Clone)]
pub struct Network {
    pub(crate) layers: Vec<Layer>,
}

impl Network {
    #[allow(dead_code)]
    pub(crate) fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    /// Note: the first item in layers is the input size for the actual first layer, so:
    ///```ignore
    /// # use neural_network::layer_topology::*;
    ///
    /// Layer::random(
    ///   vec![
    ///     LayerTopology { neurons: 3},
    ///     LayerTopology { neurons: 2},
    ///     LayerTopology { neurons: 1}
    ///   ]
    /// );
    /// ```
    /// means that the there are two layers:
    /// - the first with 3 inputs and 2 outputs
    /// - the second with 2 inputs and 1 output!
    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();
        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .copied()
    }

    pub fn from_weights(layers: &[LayerTopology], weights: impl IntoIterator<Item = f32>) -> Self {
        assert!(layers.len() > 1);

        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|layers| Layer::from_weights(layers[0].neurons, layers[1].neurons, &mut weights))
            .collect();

        if weights.next().is_some() {
            panic!("got too many weights");
        }

        Self { layers }
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use crate::neuron::Neuron;

    use super::*;

    #[test]
    fn random() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let topology = vec![
            LayerTopology { neurons: 3 },
            LayerTopology { neurons: 2 },
            LayerTopology { neurons: 1 },
        ];
        let network = Network::random(&mut rng, &topology);

        assert_eq!(network.layers.len(), 2);

        let expected_biases_layer_1: Vec<f32> = vec![-0.6255188, 0.5238805];
        approx::assert_relative_eq!(
            network.layers[0]
                .neurons
                .iter()
                .map(|n| n.bias)
                .collect::<Vec<f32>>()
                .as_slice(),
            expected_biases_layer_1.as_ref()
        );

        let expected_biases_layer_2: Vec<f32> = vec![-0.102499485];
        approx::assert_relative_eq!(
            network.layers[1]
                .neurons
                .iter()
                .map(|n| n.bias)
                .collect::<Vec<f32>>()
                .as_slice(),
            expected_biases_layer_2.as_ref()
        );
    }

    #[test]
    fn propagate() {
        let layers = (
            Layer::new(vec![
                Neuron::new(0.0, vec![-0.5, -0.4, -0.3]),
                Neuron::new(0.0, vec![-0.2, -0.1, 0.0]),
            ]),
            Layer::new(vec![Neuron::new(0.0, vec![-0.5, 0.5])]),
        );
        let network = Network::new(vec![layers.0.clone(), layers.1.clone()]);

        let actual = network.propagate(vec![0.5, 0.6, 0.7]);
        let expected = layers.1.propagate(layers.0.propagate(vec![0.5, 0.6, 0.7]));

        approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
    }
}
