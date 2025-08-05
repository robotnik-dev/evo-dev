mod layer;
mod neuron;

pub(crate) use self::{layer::*, neuron::*};
use rand::RngCore;

#[derive(Debug)]
struct Network {
    layers: Vec<Layer>,
}

impl Network {
    fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();
        Self { layers }
    }

    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(&inputs))
    }
}
