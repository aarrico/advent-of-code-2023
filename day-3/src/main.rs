mod matrix;

use crate::matrix::Matrix;

// fn check_adjacent_values(symbols_positions: &Vec<()>)

fn main() {
    let matrix = Matrix::build_from_text("input.txt");

    println!("{:?}", matrix.get_dimensions());
}
