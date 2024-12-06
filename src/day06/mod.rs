const INPUTS: &str = include_str!("./inputs.txt");

use crate::utils;
use std::collections::HashSet;

pub fn run() {
    let (mut cells, (cols, rows)) = utils::to_cells(INPUTS);

    let mut start = None;
    for y in 0..rows {
        for x in 0..cols {
            if cells[y][x] == '^' {
                start = Some(((x, y), 'U'));
            }
        }
    }
    let Some((mut pos, mut dir)) = start else {
        unreachable!()
    };
    println!("POS: {pos:?}, DIR: {dir}");

    let mut visited = HashSet::new();
    loop {
        // Current position
        let (x, y) = pos;

        // Mark as visited
        cells[y][x] = 'X';
        visited.insert(pos);
        utils::dump_cells(&cells);

        // Check in front and exit if out of boundary
        let Some(front_pos) = (match dir {
            'U' => y.checked_sub(1).map(|y| (x, y)),
            'L' => x.checked_sub(1).map(|x| (x, y)),
            'D' => Some((x, y + 1)).filter(|&(_, next_y)| next_y < rows),
            'R' => Some((x + 1, y)).filter(|&(next_x, _)| next_x < cols),
            _ => unreachable!(),
        }) else {
            println!("OUT");
            break;
        };

        let front = cells[front_pos.1][front_pos.0];

        // If front is empty, move forward
        if front != '#' {
            pos = front_pos;
            continue;
        }

        // If front is obstacle, turn right
        let (next_pos, next_dir) = match dir {
            'U' => ((x + 1, y), 'R'),
            'D' => ((x - 1, y), 'L'),
            'L' => ((x, y - 1), 'U'),
            'R' => ((x, y + 1), 'D'),
            _ => unreachable!(),
        };
        pos = next_pos;
        dir = next_dir;
    }

    let answer = visited.len();
    println!("Answer: {answer}");
}
