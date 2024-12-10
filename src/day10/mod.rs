const INPUTS: &str = include_str!("./inputs.txt");

use crate::utils;
use std::collections::HashSet;

pub fn run() {
    println!("{INPUTS}");

    let (cells, (rows, cols)) = utils::to_cells(INPUTS);

    let mut map = vec![];
    let mut starts = vec![];
    for y in 0..rows {
        let mut row = vec![];
        for x in 0..cols {
            let c = cells[y][x];
            let c = c.to_digit(10).unwrap();

            if c == 0 {
                starts.push((x, y));
            }
            row.push(c);
        }
        map.push(row);
    }
    println!("STARTS: {starts:?}");

    let mut answer = 0;
    for start in starts {
        let mut top = HashSet::new();
        let mut stack = vec![start];

        while let Some((x, y)) = stack.pop() {
            let h = map[y][x];

            if h == 9 {
                top.insert((x, y));
                continue;
            }

            let check_up = 0 < y;
            let check_down = y < rows - 1;
            let check_left = 0 < x;
            let check_right = x < cols - 1;

            if check_up && map[y - 1][x] == h + 1 {
                stack.push((x, y - 1));
            }
            if check_down && map[y + 1][x] == h + 1 {
                stack.push((x, y + 1));
            }
            if check_left && map[y][x - 1] == h + 1 {
                stack.push((x - 1, y));
            }
            if check_right && map[y][x + 1] == h + 1 {
                stack.push((x + 1, y));
            }
        }

        println!("FOUND! {top:?}");
        answer += top.len();
    }

    println!("Answer: {answer}");
}
