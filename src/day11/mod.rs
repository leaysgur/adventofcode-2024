const INPUTS: &str = include_str!("./inputs.txt");

pub fn run() {
    let mut stones = INPUTS
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    println!("Initial: {stones:?}");

    for _i in 1..=25 {
        let mut new_stones = vec![];
        for stone in &stones {
            match stone.as_str() {
                "0" => {
                    new_stones.push("1".to_string());
                }
                _ if stone.len() % 2 == 0 => {
                    let s1 = &stone[..stone.len() / 2];
                    let s2 = &stone[stone.len() / 2..];
                    new_stones.push(s1.to_string());
                    new_stones.push(str_to_int(&s2).to_string());
                }
                _ => {
                    let s = str_to_int(stone) * 2024;
                    new_stones.push(s.to_string());
                }
            }
        }
        stones = new_stones;
        // println!("After {_i} blink: {stones:?}");
    }

    let answer = stones.len();
    println!("Answer: {answer}");
}

fn str_to_int(s: &str) -> i64 {
    s.parse::<i64>().unwrap()
}
