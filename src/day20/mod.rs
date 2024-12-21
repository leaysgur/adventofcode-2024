const INPUTS: &str = include_str!("./_inputs.txt");

use crate::utils;
use std::collections::HashMap;

pub fn run() {
    let (cells, (rows, cols)) = utils::to_cells(INPUTS);

    let mut start = (0, 0);
    let mut end = (0, 0);
    for y in 0..rows {
        for x in 0..cols {
            if cells[y][x] == 'S' {
                start = (x, y);
            }
            if cells[y][x] == 'E' {
                end = (x, y);
            }
        }
    }
    println!("S: {start:?}, E: {end:?}");

    // println!("Answer: {answer}");
}
