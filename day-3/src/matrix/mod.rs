mod matrix_obj;
mod position;

use matrix_obj::MatrixObj;
use position::Position;

use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

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

    pub fn get_adjacent_number_positions(&self) -> HashSet<Position> {
        let mut adj_pos: HashSet<Position> = HashSet::new();

        let (x_dim, y_dim) = self.get_dimensions();

        for sym_pos in &self.symbol_positions {
            let (sym_x, sym_y) = sym_pos.get();

            for x in position::get_range(sym_x, x_dim - 1) {
                for y in position::get_range(sym_y, y_dim - 1) {
                    if MatrixObj::is_number(&self.data[x][y]) {
                        adj_pos.insert(Position::new(x, y));
                    }
                }
            }
        }

        println!("{}", adj_pos.len());
        adj_pos
    }

    pub fn check_adjacent_positions_for_nums(self) {
        let adjacent_num_positions = self.get_adjacent_number_positions();

        for pos in &adjacent_num_positions {}
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
