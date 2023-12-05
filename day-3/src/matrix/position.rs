use std::ops::Range;

#[derive(Debug)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn get(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

pub fn get_range(val: usize, upper_bound: usize) -> Range<usize> {
    if val == 0 {
        return 0..(val + 1);
    } else if val == upper_bound {
        return (val - 1)..(upper_bound + 1);
    }

    (val - 1)..(val + 1)
}