pub fn to_cells(inputs: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let cells = inputs
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let cols = cells[0].len();
    let rows = cells.len();

    (cells, (cols, rows))
}
