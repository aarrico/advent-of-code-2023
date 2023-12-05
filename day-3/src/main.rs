mod schematic;

use std::time::{Instant};

use schematic::{Schematic};

fn main() {
    let matrix = Schematic::build_from_text("input.txt");

    let start = Instant::now();
    let sum = matrix.get_part_number_sum();
    let duration = start.elapsed();
    println!("part 1 \n\tsum: {} \n\telapsed time: {}", sum, duration.as_micros());
}
