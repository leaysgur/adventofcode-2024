const INPUTS: &str = include_str!("./inputs.txt");

use crate::utils;

// If you find `X`, check `XMAS` around it.
// S  S  S
//  A A A
//   MMM
// SAMXMAS
//   MMM
//  A A A
// S  S  S
pub fn run() {
    // println!("{INPUTS}");
    let (cells, (cols, rows)) = utils::to_cells(INPUTS);

    let mut answer = 0;
    for y in 0..rows {
        for x in 0..cols {
            let c = cells[y][x];

            if c != 'X' {
                continue;
            }

            // Check around if inside the boundary:
            let check_up = 3 <= y;
            let check_down = y + 3 < rows;
            let check_left = 3 <= x;
            let check_right = x + 3 < cols;

            if check_up {
                if [
                    cells[y][x],
                    cells[y - 1][x],
                    cells[y - 2][x],
                    cells[y - 3][x],
                ] == ['X', 'M', 'A', 'S']
                {
                    answer += 1;
                }
            }
            if check_up && check_right {
                if [
                    cells[y][x],
                    cells[y - 1][x + 1],
                    cells[y - 2][x + 2],
                    cells[y - 3][x + 3],
                ] == ['X', 'M', 'A', 'S']
                {
                    answer += 1;
                }
            }
            if check_right {
                if [
                    cells[y][x],
                    cells[y][x + 1],
                    cells[y][x + 2],
                    cells[y][x + 3],
                ] == ['X', 'M', 'A', 'S']
                {
                    answer += 1;
                }
            }
            if check_right && check_down {
                if [
                    cells[y][x],
                    cells[y + 1][x + 1],
                    cells[y + 2][x + 2],
                    cells[y + 3][x + 3],
                ] == ['X', 'M', 'A', 'S']
                {
                    answer += 1;
                }
            }
            if check_down {
                if [
                    cells[y][x],
                    cells[y + 1][x],
                    cells[y + 2][x],
                    cells[y + 3][x],
                ] == ['X', 'M', 'A', 'S']
                {
                    answer += 1;
                }
            }
            if check_down && check_left {
                if [
                    cells[y][x],
                    cells[y + 1][x - 1],
                    cells[y + 2][x - 2],
                    cells[y + 3][x - 3],
                ] == ['X', 'M', 'A', 'S']
                {
                    answer += 1;
                }
            }
            if check_left {
                if [
                    cells[y][x],
                    cells[y][x - 1],
                    cells[y][x - 2],
                    cells[y][x - 3],
                ] == ['X', 'M', 'A', 'S']
                {
                    answer += 1;
                }
            }
            if check_left && check_up {
                if [
                    cells[y][x],
                    cells[y - 1][x - 1],
                    cells[y - 2][x - 2],
                    cells[y - 3][x - 3],
                ] == ['X', 'M', 'A', 'S']
                {
                    answer += 1;
                }
            }
        }
    }

    println!("Answer: {answer}");
}
