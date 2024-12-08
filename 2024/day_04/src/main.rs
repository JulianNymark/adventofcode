use grid::Grid;

fn main() {
    let lines = utils::lines("./input.txt");
    let x_dim = lines[0].len();
    let _y_dim = lines.len();

    let mut grid: Grid<char> = Grid::from_vec(lines.join("").chars().collect(), x_dim);

    let element = grid.get_mut(42, 42);
    *element.unwrap() = 'b';

    print_grid(&grid);
}

fn print_grid(grid: &Grid<char>) {
    for row in grid.iter_rows() {
        println!("{}", row.collect::<String>());
    }
}
