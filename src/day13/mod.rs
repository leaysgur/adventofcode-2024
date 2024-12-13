const INPUTS: &str = include_str!("./inputs.txt");

pub fn run() {
    let machines = parse_machines(INPUTS);
    // println!("{machines:?}");

    let mut tokens = 0;
    'm: for machine in machines {
        println!("M: {machine:?}");
        for push_a in 1..=100 {
            for pusb_b in 1..=100 {
                let ((ax, ay), (bx, by)) = machine.0;
                let (px, py) = machine.1;

                let dx = ax * push_a + bx * pusb_b;
                let dy = ay * push_a + by * pusb_b;

                if dx == px && dy == py {
                    let token = push_a * 3 + pusb_b * 1;
                    println!("Get: Ax{push_a}, Bx{pusb_b} = {token} tokens");
                    tokens += token;
                    continue 'm;
                }
            }
        }
    }

    let answer = tokens;
    println!("Answer: {answer}");
}

type XY = (u64, u64);
fn parse_machines(inputs: &str) -> Vec<((XY, XY), XY)> {
    inputs
        .trim()
        .split("\n\n")
        .map(|machine| {
            fn parse_x_and_y(input: &str, pat: &str) -> (u64, u64) {
                let input = input.split_once(": ").unwrap().1;
                let (x, y) = input.split_once(", ").unwrap();
                let x = x.split_once(pat).unwrap().1;
                let y = y.split_once(pat).unwrap().1;
                (x.parse().unwrap(), y.parse().unwrap())
            }

            let machine = machine.splitn(3, "\n").collect::<Vec<&str>>();
            let (ax, ay) = parse_x_and_y(machine[0], "+");
            let (bx, by) = parse_x_and_y(machine[1], "+");
            let (px, py) = parse_x_and_y(machine[2], "=");

            (((ax, ay), (bx, by)), (px, py))
        })
        .collect::<Vec<_>>()
}
