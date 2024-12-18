const INPUTS: &str = include_str!("./inputs.txt");
const COLS: usize = 71;
const ROWS: usize = 71;
const LIMIT: usize = 1024;

use crate::utils;
use std::collections::{HashMap, VecDeque};

pub fn run() {
    let bytes = INPUTS
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut cells = vec![vec!['.'; COLS]; ROWS];

    for (idx, &(x, y)) in bytes.iter().enumerate() {
        if LIMIT <= idx {
            break;
        }
        cells[y][x] = '#';
    }
    utils::debug_cells(&cells);

    let mut todo = VecDeque::new();
    let mut seen = HashMap::new();

    todo.push_back((0, 0));
    seen.insert((0, 0), 0);

    while let Some(pos) = todo.pop_front() {
        let answer = seen[&pos];
        if pos == (COLS - 1, ROWS - 1) {
            println!("Answer: {answer}");
            break;
        }

        let (x, y) = pos;
        let next = [
            y.checked_sub(1).map(|y| (x, y)),
            x.checked_sub(1).map(|x| (x, y)),
            Some((x, y + 1)).filter(|&(_, next_y)| next_y < ROWS),
            Some((x + 1, y)).filter(|&(next_x, _)| next_x < COLS),
        ];
        for next_pos in next.into_iter().flatten() {
            if cells[next_pos.1][next_pos.0] == '#' {
                continue;
            }
            if seen.contains_key(&next_pos) {
                continue;
            }

            seen.insert(next_pos, answer + 1);
            todo.push_back(next_pos);
        }
        // println!("{todo:?}");
    }
}
