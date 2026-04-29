pub enum Activation {
    Step,
    Sigmoid,
    ReLU,
}

pub struct Perceptron {
    pub number_of_inputs: usize,
    pub node_weights: Vec<f64>,
    pub bias: f64,
    pub activation: Activation,
}

impl Perceptron {
    pub fn fresh(number_of_inputs: usize, bias: f64) -> Perceptron {
        Perceptron {
            number_of_inputs,
            node_weights: vec![],
            bias,
            activation: Activation::ReLU,
        }
    }
    pub fn create(number_of_inputs: usize, bias: f64, activation: Activation) -> Perceptron {
        Perceptron {
            number_of_inputs,
            node_weights: vec![],
            bias,
            activation,
        }
    }
}
