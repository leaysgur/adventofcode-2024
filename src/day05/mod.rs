const INPUTS: &str = include_str!("./inputs.txt");

use std::collections::{HashMap, HashSet};

pub fn run() {
    let (p1, p2) = INPUTS.split_once("\n\n").unwrap();
    // println!("{p1}");

    let mut updates: Vec<Vec<i32>> = vec![];
    for line in p2.lines() {
        updates.push(line.split(',').map(|x| x.parse().unwrap()).collect());
    }

    let mut answer = 0;
    'o: for update in &updates {
        let related = update.iter().cloned().collect::<HashSet<_>>();

        let mut ordering_rules = vec![];
        for line in p1.lines() {
            let (x, y) = line.split_once('|').unwrap();
            let (x, y) = (x.parse().unwrap(), y.parse().unwrap());

            if related.contains(&x) && related.contains(&y) {
                ordering_rules.push((x, y));
            }
        }

        let page_index_map = update
            .iter()
            .enumerate()
            .map(|(i, x)| (*x, i as i32))
            .collect::<HashMap<_, _>>();

        for (x, y) in &ordering_rules {
            if page_index_map.get(&x) >= page_index_map.get(&y) {
                continue 'o;
            }
        }

        let midx = update.len() / 2;
        answer += update[midx];
    }

    println!("Answer: {answer}");
}
