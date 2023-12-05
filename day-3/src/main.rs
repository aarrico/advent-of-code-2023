mod matrix;

use crate::matrix::Matrix;

fn main() {
    let matrix = Matrix::build_from_text("input.txt");

    println!("{:?}", matrix.get_dimensions());
}
