use grid::Grid;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let lines = utils::lines("./input.txt");
    let x_dim = lines[0].len();
    let _y_dim = lines.len();

    let mut grid: Grid<char> = Grid::from_vec(lines.join("").chars().collect(), x_dim);

    let count = iter_thru_grid(&grid);
    println!("part 1: {}", count);
}

fn part_2() {
    let lines = utils::lines("./input.txt");
    let x_dim = lines[0].len();
    let _y_dim = lines.len();

    let mut grid: Grid<char> = Grid::from_vec(lines.join("").chars().collect(), x_dim);

    let count = iter_thru_grid(&grid);
    println!("part 1: {}", count);
}

fn iter_thru_grid(grid: &Grid<char>) -> i32 {
    let mut count = 0;
    for (row_idx, row) in grid.iter_rows().enumerate() {
        for (col_idx, char) in row.enumerate() {
            if *char == 'X' {
                count += sweep_rotate_xmas_check(grid, row_idx, col_idx);
            }
        }
    }
    count
}

const DIRECTIONS: [(i32, i32); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

/// check all 8 directions on grid N, NE, E, SE, S, SW, W, NW
fn sweep_rotate_xmas_check(grid: &Grid<char>, row_idx: usize, col_idx: usize) -> i32 {
    let mut count = 0;
    for direction in DIRECTIONS {
        let x = TryInto::<i32>::try_into(row_idx).unwrap();
        let y = TryInto::<i32>::try_into(col_idx).unwrap();

        let sample_m = grid
            .get(x + (direction.0), y + (direction.1))
            .unwrap_or(&'\0');
        let sample_a = grid
            .get(x + (direction.0 * 2), y + (direction.1 * 2))
            .unwrap_or(&'\0');
        let sample_s = grid
            .get(x + (direction.0 * 3), y + (direction.1 * 3))
            .unwrap_or(&'\0');
        if *sample_m == 'M' && *sample_a == 'A' && *sample_s == 'S' {
            // println!("XMAS found at {},{}", row_idx, col_idx);
            count += 1;
        }
    }
    count
}

fn _print_grid(grid: &Grid<char>) {
    for row in grid.iter_rows() {
        println!("{}", row.collect::<String>());
    }
}
