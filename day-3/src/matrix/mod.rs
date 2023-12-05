mod matrix_obj;
mod position;

use matrix_obj::MatrixObj;
use position::Position;

use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct Matrix {
    data: Vec<Vec<MatrixObj>>,
    symbol_positions: Vec<Position>,
}

impl Matrix {
    pub fn build_from_text(filename: &str) -> Self {
        let file = BufReader::new(File::open(filename).unwrap());

        let lines = file.lines()
            .map(|line| line.unwrap().chars().collect::<Vec<_>>());

        let mut data: Vec<Vec<MatrixObj>> = Vec::new();
        let mut symbol_positions: Vec<Position> = Vec::new();

        for (x, chars) in lines.enumerate() {
            data.push(Vec::new());

            for (y, c) in chars.iter().enumerate() {
                let char_type = MatrixObj::determine_type(*c);

                if matches!(char_type.is_symbol(), true) {
                    symbol_positions.push(Position::new(x, y))
                }

                data[x].push(char_type);
            }
        }

        Matrix {
            data,
            symbol_positions,
        }
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.data.len(), self.data[0].len())
    }

    pub fn check_adjacent_values(self,) {
        let (x_dim, y_dim) = self.get_dimensions();
        for pos in &self.symbol_positions {
            println!("{:?}", &pos.get_adjacent_positions(x_dim - 1, y_dim - 1))
        }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }

        write!(f, "")
    }
}
