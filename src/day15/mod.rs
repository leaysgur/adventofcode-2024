const INPUTS: &str = include_str!("./inputs.txt");

use crate::utils;

pub fn run() {
    let (map, movements) = INPUTS.trim().split_once("\n\n").unwrap();

    let (mut cells, (rows, cols)) = utils::to_cells(map);
    let movements = movements
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => unreachable!(),
        })
        .collect::<Vec<(i32, i32)>>();

    utils::debug_cells(&cells);
    // println!("{movements:?}");

    let mut current: (i32, i32) = (0, 0);
    for y in 0..rows {
        for x in 0..cols {
            if cells[y][x] == '@' {
                current = (x as i32, y as i32);
                break;
            }
        }
    }
    println!("Start: {current:?}");

    for movement in movements {
        // utils::dump_cells(&cells);

        let (cx, cy) = current;
        let (nx, ny) = (current.0 + movement.0, current.1 + movement.1);

        let nc = cells[ny as usize][nx as usize];

        // If wall, continue
        if nc == '#' {
            continue;
        }

        // If space, move
        if nc == '.' {
            cells[cy as usize][cx as usize] = '.';
            cells[ny as usize][nx as usize] = '@';
            current = (nx, ny);
            continue;
        }

        // Otherwise, boxes may be pushed
        let (mut dx, mut dy) = (nx + movement.0, ny + movement.1);
        loop {
            let dc = cells[dy as usize][dx as usize];

            if dc == '#' {
                break;
            }

            if dc == '.' {
                cells[cy as usize][cx as usize] = '.';
                cells[ny as usize][nx as usize] = '@';
                cells[dy as usize][dx as usize] = 'O';
                current = (nx, ny);
                break;
            }

            dx += movement.0;
            dy += movement.1;
        }
    }
    utils::debug_cells(&cells);

    // Count box distances
    let mut answer = 0;
    for y in 0..rows {
        for x in 0..cols {
            if cells[y][x] == 'O' {
                let distance = 100 * y + x;
                answer += distance;
            }
        }
    }
    println!("Answer: {answer}");
}
