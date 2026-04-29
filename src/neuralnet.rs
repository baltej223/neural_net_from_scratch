use crate::perceptron::{Activation, Perceptron};

struct Layer {
    neurons: Vec<Perceptron>,
}
impl Layer {
    pub fn new() -> Layer {
        Layer {
            neurons: Vec::new(),
        }
    }
}

struct Network {
    layers: Vec<Layer>,
}
impl Network {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }
}

pub struct MLP {
    network: Network,
    input_size: usize,
    hidden_sizes: Vec<usize>,
    output_size: usize,
}

impl MLP {
    pub fn construct(input_size: usize, hidden_sizes: Vec<usize>, output_size: usize) -> MLP {
        let mut network = Network { layers: vec![] };

        let mut input_layer = Layer::new();
        for _ in 0..input_size {
            let n = Perceptron::create(input_size, 0.0, Activation::ReLU);
            input_layer.neurons.push(n);
        }
        network.layers.push(input_layer);

        let mut prev_layer_size = input_size;
        let mut hidden_layer = Layer::new();
        for hidden_layer_size in hidden_sizes.clone() {
            for _ in 0..hidden_layer_size {
                let n = Perceptron::create(prev_layer_size, 0.0, Activation::ReLU);
                hidden_layer.neurons.push(n);
            }
            prev_layer_size = hidden_layer_size;
        }
        network.layers.push(hidden_layer);

        let mut output_layer = Layer::new();
        for _ in 0..output_size {
            let n = Perceptron::create(prev_layer_size, 0.0, Activation::ReLU);
            output_layer.neurons.push(n);
        }
        network.layers.push(output_layer);

        MLP {
            input_size,
            hidden_sizes,
            output_size,
            network,
        }
    }
}
