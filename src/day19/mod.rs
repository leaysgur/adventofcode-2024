const INPUTS: &str = include_str!("./inputs.txt");

use std::collections::HashSet;

pub fn run() {
    let (available, patterns) = INPUTS.split_once("\n\n").unwrap();
    let available = available.split(", ").collect::<HashSet<_>>();
    println!("{available:?}");
    let patterns = patterns.lines().collect::<Vec<_>>();
    println!("{patterns:?}");

    let answer = patterns
        .iter()
        .filter(|pattern| check(pattern.to_string(), 0, &available))
        .count();

    println!("Answer: {answer}");
}

fn check(pattern: String, start: usize, available: &HashSet<&str>) -> bool {
    if pattern.len() <= start {
        return true;
    }

    // Try longer strings first
    for idx in (start + 1..=pattern.len()).rev() {
        if !available.contains(&pattern[start..idx]) {
            continue;
        }
        if !check(pattern.clone(), idx, available) {
            continue;
        }

        return true;
    }

    false
}
