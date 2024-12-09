const INPUTS: &str = include_str!("./inputs.txt");

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Item {
    Block(u64),
    Space,
}

pub fn run() {
    // println!("{INPUTS}");
    // > 2333133121414131402
    // 2 block, 3 spacs, 3 blocks, ...
    let chars = INPUTS.trim().chars().collect::<Vec<_>>();

    // > 00...111...2...333.44.5555.6666.777.888899
    let mut map: Vec<Item> = vec![];
    let mut is_block = true;
    let mut id = 0;
    for d in chars {
        let d = d.to_digit(10).unwrap();
        if is_block {
            for _ in 0..d {
                map.push(Item::Block(id));
            }
            id += 1;
        } else {
            for _ in 0..d {
                map.push(Item::Space);
            }
        }
        is_block = !is_block;
    }
    print_map(&map);

    // Swap left most '.' with right most non '.' char
    // > 0099811188827773336446555566..............
    loop {
        let first_space_idx = map.iter().position(|&c| c == Item::Space).unwrap();
        let last_file_idx = map.iter().rposition(|&c| c != Item::Space).unwrap();

        if last_file_idx <= first_space_idx {
            break;
        }

        map.swap(first_space_idx, last_file_idx);
    }
    print_map(&map);

    let answer = map
        .iter()
        .enumerate()
        .map(|(idx, item)| match item {
            Item::Block(id) => idx as u64 * *id,
            Item::Space => 0,
        })
        .sum::<u64>();
    println!("Answer: {answer}");
}

fn print_map(map: &[Item]) {
    for item in map {
        match item {
            Item::Block(id) => print!("{id}"),
            Item::Space => print!("."),
        }
    }
    println!();
}
