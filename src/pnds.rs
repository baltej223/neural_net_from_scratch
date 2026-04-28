use std::fmt::Display;

// a pure dataframe where all the data is already in the memory
pub struct Dataframe<T> {
    pub rows: usize,
    pub cols: usize,
    pub params: Vec<&'static str>,
    pub data: Vec<Vec<T>>,
}

// a file with an abstraction of dataframe
pub struct LargeDataframe {
    pub data: std::fs::File,
}

impl<T: Display> Dataframe<T> {
    pub fn new(parameters: Vec<&'static str>, data: Vec<Vec<T>>) -> Dataframe<T> {
        let rows = parameters.len();

        if data.len() != rows {
            panic!("Data not consistent!");
        }

        let mut cols = 0;
        let updated_once = false;
        for row in &data {
            let row_length_determined = row.len();
            if cols != row_length_determined {
                if !updated_once {
                    cols = row_length_determined;
                } else {
                    panic!("The date provided is not consistent.");
                }
            }
        }

        let df: Dataframe<T> = Dataframe {
            rows,
            cols,
            params: parameters,
            data,
        };
        df
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
}
