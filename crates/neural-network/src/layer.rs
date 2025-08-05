use crate::Neuron;
use rand::RngCore;

pub(crate) struct LayerTopology {
    pub(crate) neurons: usize,
}

#[derive(Debug)]
pub(crate) struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    pub(crate) fn random(rng: &mut dyn RngCore, input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(rng, input_size))
            .collect();
        Self { neurons }
    }

    pub(crate) fn propagate(&self, inputs: &[f32]) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(inputs))
            .collect()
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
        let layer = Layer::random(&mut rng, 4, 3);

        // approx::assert_relative_eq!(layer.bias, -0.6255188);
        // approx::assert_relative_eq!(
        //     layer.weights.as_slice(),
        //     &[0.67383933, 0.81812596, 0.26284885, 0.5238805].as_ref()
        // );
    }

    #[test]
    fn propagate() {
        todo!()
    }
}
