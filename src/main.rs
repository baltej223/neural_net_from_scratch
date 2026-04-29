// #[allow(dead_code)]
// use crate::{perceptron::Perceptron, pnds::Dataframe};
// use std::path::PathBuf;

use crate::neuralnet::MLP;

mod neuralnet;
mod perceptron;
mod pnds;

fn main() {
    // let params: Vec<String> = vec![String::from("Col1"), String::from("Col2")];
    //
    // let data = vec![vec![1, 2], vec![3, 4]];
    //
    // let df: Dataframe<usize> = Dataframe::new(params, data);
    //
    // df.print();
    //
    // // perceptron thing here
    // let perceptron: Perceptron = Perceptron::fresh(10, 0.0);
    //
    // let csv_data: Dataframe<f64> = Dataframe::from_csv(PathBuf::from("data.csv"));
    // csv_data.print();

    let net: MLP = MLP::construct(100, vec![64, 32], 30);
}
