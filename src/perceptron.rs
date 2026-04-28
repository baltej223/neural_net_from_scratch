pub struct Perceptron {
    pub number_of_inputs: usize,
    pub node_weights: Vec<i64>,
    pub threshold: usize,
    // with activation fn begin : if sum > threshold == true;
}

impl Perceptron {
    pub fn fresh(number_of_inputs: usize, threshold: usize) -> Perceptron {
        Perceptron {
            number_of_inputs,
            node_weights: vec![],
            threshold: 0,
        }
    }
}
