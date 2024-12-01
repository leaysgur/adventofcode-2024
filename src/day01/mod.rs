const INPUTS: &str = include_str!("./inputs.txt");

pub fn run() {
    let (mut left_items, mut right_items): (Vec<u32>, Vec<u32>) = INPUTS
        .lines()
        .filter_map(|line| {
            let mut nums = line
                .split_whitespace()
                .filter_map(|v| v.parse::<u32>().ok());
            Some((nums.next()?, nums.next()?))
        })
        .unzip();

    left_items.sort_unstable();
    right_items.sort_unstable();

    let answer = left_items
        .iter()
        .zip(right_items.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum::<u32>();
    println!("Answer: {answer}");
}
