/// Returns: (cells, (cols, rows))
pub fn to_cells(inputs: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let cells = inputs
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let cols = cells[0].len();
    let rows = cells.len();

    (cells, (cols, rows))
}

pub fn debug_cells(cells: &Vec<Vec<char>>) {
    for row in cells {
        for c in row {
            print!("{c}");
        }
        println!();
    }
    println!();
}

pub fn dump_cells(cells: &Vec<Vec<char>>) {
    // Wait and wipe!
    std::thread::sleep(std::time::Duration::from_millis(16));
    print!("\x1Bc");

    for row in cells {
        for c in row {
            print!("{c}");
        }
        println!();
    }
    println!();
}
