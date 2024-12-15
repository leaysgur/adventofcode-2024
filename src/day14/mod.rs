// const INPUTS: &str = include_str!("./_inputs.txt");
// const W: i32 = 11;
// const H: i32 = 7;
const INPUTS: &str = include_str!("./inputs.txt");
const W: i32 = 101;
const H: i32 = 103;

pub fn run() {
    let mut robots = parse_inputs(INPUTS);
    println!("Robots: {:?}", robots);

    for _ in 1..=100 {
        for robot in &mut robots {
            robot.0 .0 += robot.1 .0;
            robot.0 .1 += robot.1 .1;
        }
    }

    for robot in &mut robots {
        robot.0 .0 = robot.0 .0 % W;
        robot.0 .1 = robot.0 .1 % H;

        if robot.0 .0 < 0 {
            robot.0 .0 += W;
        }
        if robot.0 .1 < 0 {
            robot.0 .1 += H;
        }
    }
    println!("Robots: {:?}", robots);

    // let mut grid = vec![vec!['.'; W as usize]; H as usize];
    // for robot in &robots {
    //     if grid[robot.0 .1 as usize][robot.0 .0 as usize] == '.' {
    //         grid[robot.0 .1 as usize][robot.0 .0 as usize] = '1';
    //     } else {
    //         grid[robot.0 .1 as usize][robot.0 .0 as usize] = (grid[robot.0 .1 as usize]
    //             [robot.0 .0 as usize]
    //             .to_digit(10)
    //             .unwrap()
    //             + 1)
    //         .to_string()
    //         .chars()
    //         .next()
    //         .unwrap();
    //     }
    // }
    // crate::utils::debug_cells(&grid);

    // [TL, TR, BL, BR]
    let mut quadrant = [0; 4];
    // exactly in the middle (horizontally or vertically) don't count as being in any quadrant
    let (dx, dy) = (W / 2, H / 2);
    for robot in &robots {
        let (x, y) = robot.0;

        if x < dx && y < dy {
            quadrant[0] += 1;
        }
        if dx < x && y < dy {
            quadrant[1] += 1;
        }
        if x < dx && dy < y {
            quadrant[2] += 1;
        }
        if dx < x && dy < y {
            quadrant[3] += 1;
        }
    }
    println!("Quadrant: {:?}", quadrant);

    let answer = quadrant.iter().fold(1, |acc, x| acc * x);
    println!("Answer: {answer}");
}

fn parse_inputs(inputs: &str) -> Vec<((i32, i32), (i32, i32))> {
    inputs
        .trim()
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(" ").unwrap();

            let (px, py) = p.split_once("=").unwrap().1.split_once(",").unwrap();
            let (px, py) = (px.parse().unwrap(), py.parse().unwrap());

            let (vx, vy) = v.split_once("=").unwrap().1.split_once(",").unwrap();
            let (vx, vy) = (vx.parse().unwrap(), vy.parse().unwrap());

            ((px, py), (vx, vy))
        })
        .collect()
}
