const INPUTS: &str = include_str!("./inputs.txt");

use crate::utils;
use std::collections::HashSet;

pub fn run() {
    let (cells, (rows, cols)) = utils::to_cells(INPUTS);

    let mut regions = vec![];
    let mut pos_checked = HashSet::new();
    'outer: for y in 0..rows {
        for x in 0..cols {
            if pos_checked.len() == rows * cols {
                break 'outer;
            }
            if pos_checked.contains(&(x, y)) {
                continue;
            }

            let pos = find_same_region_pos((x, y), &cells, (cols, rows));
            for (x, y) in &pos {
                pos_checked.insert((*x, *y));
            }
            println!("Found region: {} {:?}", cells[y][x], pos);

            let area = count_area(&pos);
            let perimeter = count_perimeter(&pos, &cells, (cols, rows));
            println!("  area: {area}, perimeter: {perimeter}",);

            regions.push((area, perimeter));
        }
    }

    let answer = regions
        .iter()
        .map(|(area, perimeter)| area * perimeter)
        .sum::<usize>();
    println!("Answer: {answer}");
}

fn find_same_region_pos(
    start: (usize, usize),
    cells: &Vec<Vec<char>>,
    (rows, cols): (usize, usize),
) -> Vec<(usize, usize)> {
    let mut seen = HashSet::new();

    let mut stack = vec![start];
    while let Some((x, y)) = stack.pop() {
        seen.insert((x, y));

        let check_up = 0 < y;
        let check_down = y < rows - 1;
        let check_left = 0 < x;
        let check_right = x < cols - 1;

        if check_up && cells[y - 1][x] == cells[y][x] && !seen.contains(&(x, y - 1)) {
            stack.push((x, y - 1));
        }
        if check_down && cells[y + 1][x] == cells[y][x] && !seen.contains(&(x, y + 1)) {
            stack.push((x, y + 1));
        }
        if check_left && cells[y][x - 1] == cells[y][x] && !seen.contains(&(x - 1, y)) {
            stack.push((x - 1, y));
        }
        if check_right && cells[y][x + 1] == cells[y][x] && !seen.contains(&(x + 1, y)) {
            stack.push((x + 1, y));
        }
    }

    seen.into_iter().collect::<Vec<_>>()
}

fn count_area(pos: &Vec<(usize, usize)>) -> usize {
    pos.len()
}

fn count_perimeter(
    pos: &Vec<(usize, usize)>,
    cells: &Vec<Vec<char>>,
    (rows, cols): (usize, usize),
) -> usize {
    let mut perimeter = 0;

    for &(x, y) in pos {
        let check_up = 0 < y;
        let check_down = y < rows - 1;
        let check_left = 0 < x;
        let check_right = x < cols - 1;

        if !(check_up && cells[y - 1][x] == cells[y][x]) {
            perimeter += 1;
        }
        if !(check_down && cells[y + 1][x] == cells[y][x]) {
            perimeter += 1;
        }
        if !(check_left && cells[y][x - 1] == cells[y][x]) {
            perimeter += 1;
        }
        if !(check_right && cells[y][x + 1] == cells[y][x]) {
            perimeter += 1;
        }
    }

    perimeter
}
