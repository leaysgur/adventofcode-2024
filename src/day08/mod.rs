const INPUTS: &str = include_str!("./inputs.txt");

use crate::utils;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct AntennaPair {
    a1: (usize, usize),
    a2: (usize, usize),
}
impl AntennaPair {
    // ```
    // ..........
    // ...#......
    // ..........
    // ....a.....
    // ..........
    // .....a....
    // ..........
    // ......#...
    // ..........
    // ```
    // Postions of `#` are antinodes.
    // If positon is out of bounds, it is ignored.
    fn antinodes(&self, (cols, rows): (usize, usize)) -> Vec<(usize, usize)> {
        let mut antinodes = vec![];

        let (x1, y1) = self.a1;
        let (x2, y2) = self.a2;

        let dx = x2 as isize - x1 as isize;
        let dy = y2 as isize - y1 as isize;
        let nx1 = x1 as isize + 2 * dx;
        let ny1 = y1 as isize + 2 * dy;

        if 0 <= nx1 && nx1 < cols as isize && 0 <= ny1 && ny1 < rows as isize {
            antinodes.push((nx1 as usize, ny1 as usize));
        }

        let dx2 = x1 as isize - x2 as isize;
        let dy2 = y1 as isize - y2 as isize;
        let nx2 = x2 as isize + 2 * dx2;
        let ny2 = y2 as isize + 2 * dy2;

        if 0 <= nx2 && nx2 < cols as isize && 0 <= ny2 && ny2 < rows as isize {
            antinodes.push((nx2 as usize, ny2 as usize));
        }

        antinodes
    }
}

pub fn run() {
    let (mut cells, (cols, rows)) = utils::to_cells(INPUTS);

    let mut freqs: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut pairs: Vec<AntennaPair> = vec![];
    for y in 0..rows {
        for x in 0..cols {
            let c = cells[y][x];
            if c == '.' {
                continue;
            }

            if let Some(seen) = freqs.get(&c) {
                for antenna in seen {
                    pairs.push(AntennaPair {
                        a1: (x, y),
                        a2: *antenna,
                    });
                }
            }

            freqs.entry(c).or_default().push((x, y));
        }
    }

    let mut locations = HashSet::new();
    for pair in pairs {
        for (x, y) in pair.antinodes((cols, rows)) {
            cells[y][x] = '#';
            utils::dump_cells(&cells);
            locations.insert((x, y));
        }
    }

    let answer = locations.len();
    println!("Answer: {answer}");
}
