// #[allow(dead_code)]
use crate::{perceptron::Perceptron, pnds::Dataframe};

mod perceptron;
mod pnds;

fn main() {
    let params: Vec<&str> = vec!["Col1", "Col2"];

    let data = vec![vec![1, 2], vec![3, 4]];

    let df: Dataframe<usize> = Dataframe::new(params, data);

    df.print();

    // perceptron thing here
    let perceptron: Perceptron = Perceptron::fresh(10, 0);
}
