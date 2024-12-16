const INPUTS: &str = include_str!("./inputs.txt");

use crate::utils;
use std::collections::HashMap;

pub fn run() {
    let (cells, (rows, cols)) = utils::to_cells(INPUTS);

    let mut start = None;
    let mut end = None;
    for y in 0..rows {
        for x in 0..cols {
            if cells[y][x] == 'S' {
                start = Some((x, y));
            }
            if cells[y][x] == 'E' {
                end = Some((x, y));
            }
        }
    }
    let Some(start) = start else { unreachable!() };
    let Some(end) = end else { unreachable!() };

    let mut goals = vec![];
    let mut todo = vec![];
    let mut seen = HashMap::new();

    todo.push((0, (start, 'R')));
    seen.insert((start, 'R'), 0);

    while let Some((pt, (pos, dir))) = todo.pop() {
        if pos == end {
            goals.push(pt);
            continue;
        }

        let left = dir_counter_clockwise(dir);
        let right = dir_clockwise(dir);
        let next = [
            (pos_next(pos, dir), dir, pt + 1),
            (pos, left, pt + 1000),
            (pos, right, pt + 1000),
        ];

        for (next_pos, next_dir, next_pt) in next {
            if cells[next_pos.1][next_pos.0] != '#' {
                let key = (next_pos, next_dir);
                let existing = *seen.get(&key).unwrap_or(&u32::MAX);

                if existing > next_pt {
                    todo.push((next_pt, key));
                    seen.insert(key, next_pt);
                }
            }
        }
    }

    goals.sort_unstable();

    let answer = goals[0];
    println!("Answer: {answer}");
}

fn dir_counter_clockwise(c: char) -> char {
    match c {
        'U' => 'L',
        'L' => 'D',
        'D' => 'R',
        'R' => 'U',
        _ => unreachable!(),
    }
}

fn dir_clockwise(c: char) -> char {
    match c {
        'U' => 'R',
        'R' => 'D',
        'D' => 'L',
        'L' => 'U',
        _ => unreachable!(),
    }
}

fn pos_next((x, y): (usize, usize), c: char) -> (usize, usize) {
    match c {
        'U' => (x, y - 1),
        'L' => (x - 1, y),
        'D' => (x, y + 1),
        'R' => (x + 1, y),
        _ => unreachable!(),
    }
}
