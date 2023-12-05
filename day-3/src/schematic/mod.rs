mod schematic_obj;
mod position;

use schematic_obj::SchematicObj;
use position::Position;

use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

pub struct Schematic {
    matrix: Vec<Vec<SchematicObj>>,
    symbol_positions: Vec<Position>,
    row_count: usize,
    col_count: usize,
}

impl Schematic {
    pub fn build_from_text(filename: &str) -> Self {
        let file = BufReader::new(File::open(filename).unwrap());

        let lines = file.lines()
            .map(|line| line.unwrap().chars().collect::<Vec<_>>());

        let mut matrix: Vec<Vec<SchematicObj>> = Vec::new();
        let mut symbol_positions: Vec<Position> = Vec::new();

        for (x, chars) in lines.enumerate() {
            matrix.push(Vec::new());

            for (y, c) in chars.iter().enumerate() {
                let char_type = SchematicObj::determine_type(*c);

                if char_type.is_symbol() {
                    symbol_positions.push(Position::new(x, y));
                }

                matrix[x].push(char_type);
            }
        }

        let row_count = (&matrix).len();
        let col_count = (&matrix)[0].len();

        Schematic {
            matrix,
            symbol_positions,
            row_count,
            col_count,
        }
    }

    fn get_dimensions(&self) -> (usize, usize) {
        (self.row_count - 1, self.col_count - 1)
    }

    fn get_digit_positions(&self) -> Vec<Position> {
        let mut adj_pos: Vec<Position> = Vec::new();

        let (x_dim, y_dim) = self.get_dimensions();

        for sym_pos in &self.symbol_positions {
            let (sym_x, sym_y) = sym_pos.get();

            for x in position::get_range(sym_x, x_dim - 1) {
                for y in position::get_range(sym_y, y_dim - 1) {
                    if self.matrix[x][y].is_number() {
                        adj_pos.push(Position::new(x, y));
                        if y != sym_y {
                            break;
                        }
                    }
                }
            }
        }

        adj_pos
    }

    pub fn get_part_number_sum(self) -> usize {
        let mut sum: usize = 0;
        let digit_positions = self.get_digit_positions();

        for pos in &digit_positions {
            let (x, y) = pos.get();
            let (_, y_max) = self.get_dimensions();

            let num_str = build_number_str(&self.matrix[x], y, y_max);

            if let Ok(num) = num_str.parse::<usize>() {
                sum += num;
            } else {
                panic!("bug in building number string\n\tfound: {}", num_str);
            }
        }

        sum
    }
}

impl fmt::Display for Schematic {
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

fn build_number_str(row: &Vec<SchematicObj>, y_initial: usize, y_max: usize) -> String {
    let mut prev = &row[y_initial];
    let mut next;

    let mut y_prev = y_initial;
    let mut y_next = y_initial;
    let mut num_str = String::from(prev.get_value());

    let mut continue_back = true;
    let mut continue_forward = true;

    while continue_back || continue_forward {

        if continue_back && y_prev > 0 {
            y_prev -= 1;
            prev = &row[y_prev];

            let is_number = prev.is_number();

            if is_number {
                num_str = format!("{}{num_str}", prev.get_value());
            }

            if !is_number || y_prev == 0 {
                continue_back = false;
            }
        }

        if continue_forward && y_next < y_max {
            y_next += 1;
            next = &row[y_next];

            let is_number = next.is_number();

            if is_number {
                num_str.push(next.get_value());
            }

            if !is_number || y_next == y_max {
                continue_forward = false;
            }
        }
    }

    num_str
}
