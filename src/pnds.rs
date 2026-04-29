use std::{fmt::Display, fs, path::Path};

// a pure dataframe where all the data is already in the memory
pub struct Dataframe<T> {
    pub rows: usize,
    pub cols: usize,
    pub params: Vec<String>,
    pub data: Vec<Vec<T>>,
}

// a file with an abstraction of dataframe
pub struct LargeDataframe {
    pub data: std::fs::File,
}

impl<T> Dataframe<T>
where
    T: std::str::FromStr + Clone,
    T::Err: std::fmt::Debug,
    T: std::fmt::Debug,
    T: std::fmt::Display,
{
    pub fn new(parameters: Vec<String>, data: Vec<Vec<T>>) -> Dataframe<T> {
        let rows = data.len();
        let cols = parameters.len();

        for row in &data {
            if row.len() != cols {
                panic!("Data not consistent!");
            }
        }

        Dataframe {
            rows,
            cols,
            params: parameters,
            data,
        }
    }

    pub fn print(&self) {
        // print the params
        let params = &self.params;

        for param in params {
            print!("{param},");
        }
        println!();

        for row in 0..self.rows {
            for col in 0..self.cols {
                print!("{},", self.data[row][col]);
            }
            println!();
        }
    }
    pub fn from_csv<P: AsRef<Path>>(path: P) -> Self {
        let csv_data = fs::read_to_string(path).expect("File Read Error!");
        let rows: Vec<&str> = csv_data
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect();
        let params: Vec<String> = rows[0].split(",").map(String::from).collect();

        let rows_length = rows.len();
        let cols_length = params.len();

        let mut data: Vec<Vec<T>> = vec![];
        for i in 1..rows_length {
            let mut inner_vec: Vec<T> = vec![];
            let parsed_row: Vec<T> = rows[i]
                .split(",")
                .map(|x| x.trim().parse::<T>().unwrap())
                .collect();

            // Here
            // println!("{:?}", parsed_row);
            // println!("Expected cols: {}", cols_length);
            // ----

            for value in parsed_row {
                inner_vec.push(value);
            }
            data.push(inner_vec);
        }

        Dataframe::new(params, data)
    }
}
