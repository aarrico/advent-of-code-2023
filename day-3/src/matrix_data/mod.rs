mod matrix_obj;
mod position;

use matrix_obj::MatrixObj;
use position::Position;

use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

pub struct MatrixData {
    matrix: Vec<Vec<MatrixObj>>,
    symbol_positions: HashSet<Position>,
}

impl MatrixData {
    pub fn build_from_text(filename: &str) -> Self {
        let file = BufReader::new(File::open(filename).unwrap());

        let lines = file.lines()
            .map(|line| line.unwrap().chars().collect::<Vec<_>>());

        let mut matrix: Vec<Vec<MatrixObj>> = Vec::new();
        let mut symbol_positions: HashSet<Position> = HashSet::new();

        for (x, chars) in lines.enumerate() {
            matrix.push(Vec::new());

            for (y, c) in chars.iter().enumerate() {
                let char_type = MatrixObj::determine_type(*c);

                if matches!(char_type.is_symbol(), true) {
                    symbol_positions.insert(Position::new(x, y));
                }

                matrix[x].push(char_type);
            }
        }

        MatrixData {
            matrix,
            symbol_positions,
        }
    }

    fn get_dimensions(&self) -> (usize, usize) {
        (self.matrix.len(), self.matrix[0].len())
    }

    fn get_digit_positions(&self) -> HashSet<Position> {
        let mut adj_pos: HashSet<Position> = HashSet::new();

        let (x_dim, y_dim) = self.get_dimensions();

        for sym_pos in &self.symbol_positions {
            let (sym_x, sym_y) = sym_pos.get();

            for x in position::get_range(sym_x, x_dim - 1) {
                for y in position::get_range(sym_y, y_dim - 1) {
                    if MatrixObj::is_number(&self.matrix[x][y]) {
                        adj_pos.insert(Position::new(x, y));
                    }
                }
            }
        }

        adj_pos
    }

    pub fn get_part_number_sum(self) -> usize {
        let mut sum: usize = 0;
        let digit_positions = self.get_digit_positions();

        for pos in &digit_positions {}

        sum
    }
}

impl fmt::Display for MatrixData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.matrix {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }

        write!(f, "")
    }
}
