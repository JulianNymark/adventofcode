use std::collections::HashSet;

use grid::Grid;

fn main() {
    part_1();
    part_2();
}

fn part_1() -> i32 {
    let lines = utils::lines("./input.txt");
    let x_dim = lines[0].len();
    let _y_dim = lines.len();

    let grid: Grid<char> = Grid::from_vec(lines.join("").chars().collect(), x_dim);

    let count = iter_thru_grid(&grid, "xmas");
    println!("part 1: {}", count);
    count
}

/// ? .. 1937
fn part_2() -> i32 {
    let lines = utils::lines("./input.txt");
    let x_dim = lines[0].len();
    let _y_dim = lines.len();

    let grid: Grid<char> = Grid::from_vec(lines.join("").chars().collect(), x_dim);

    let count = iter_thru_grid(&grid, "x-mas");
    println!("part 2: {}", count);
    count
}

fn iter_thru_grid(grid: &Grid<char>, variant: &str) -> i32 {
    let mut unique_locations: HashSet<(i32, i32)> = HashSet::new();
    let mut count = 0;

    for (row_idx, row) in grid.iter_rows().enumerate() {
        for (col_idx, char) in row.enumerate() {
            if variant == "xmas" {
                if *char == 'X' {
                    count += sweep_rotate_xmas_count(grid, row_idx, col_idx);
                }
            } else if variant == "x-mas" {
                if *char == 'M' {
                    sweep_rotate_x_mas_count(&mut unique_locations, grid, row_idx, col_idx);
                }
            }
        }
    }
    if count > 0 {
        return count;
    }

    unique_locations.len().try_into().unwrap()
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
fn sweep_rotate_xmas_count(grid: &Grid<char>, row_idx: usize, col_idx: usize) -> i32 {
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
            count += 1;
        }
    }
    count
}

fn sweep_rotate_x_mas_count(
    unique_locations: &mut HashSet<(i32, i32)>,
    grid: &Grid<char>,
    row_idx: usize,
    col_idx: usize,
) {
    for (idx, direction) in DIRECTIONS.iter().enumerate() {
        let x = TryInto::<i32>::try_into(row_idx).unwrap();
        let y = TryInto::<i32>::try_into(col_idx).unwrap();

        let dir_1 = (x + (direction.0), y + (direction.1));
        let dir_2 = (x + (direction.0 * 2), y + (direction.1 * 2));

        let sample_a = grid.get(dir_1.0, dir_1.1).unwrap_or(&'\0');
        let sample_s = grid.get(dir_2.0, dir_2.1).unwrap_or(&'\0');
        if *sample_a == 'A' && *sample_s == 'S' {
            let potential_location = dir_1;
            let orthogonal_dir = orthogonal(DIRECTIONS, idx);
            let ortho_dir_1 = (
                potential_location.0 + orthogonal_dir.0,
                potential_location.1 + orthogonal_dir.1,
            );
            let ortho_dir_mirror = (
                potential_location.0 + (orthogonal_dir.0 * -1),
                potential_location.1 + (orthogonal_dir.1 * -1),
            );

            let sample_ortho_1 = grid.get(ortho_dir_1.0, ortho_dir_1.1).unwrap_or(&'\0');
            let sample_ortho_mirror = grid
                .get(ortho_dir_mirror.0, ortho_dir_mirror.1)
                .unwrap_or(&'\0');

            if (*sample_ortho_1 == 'M' && *sample_ortho_mirror == 'S')
                || (*sample_ortho_1 == 'S' && *sample_ortho_mirror == 'M')
            {
                unique_locations.insert(potential_location);
            }
        }
    }
}

fn orthogonal(directions: [(i32, i32); 8], index: usize) -> (i32, i32) {
    return directions[(index + 2) % directions.len()];
}

fn _print_grid(grid: &Grid<char>) {
    for row in grid.iter_rows() {
        println!("{}", row.collect::<String>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orthogonal_works() {
        assert_eq!(orthogonal(DIRECTIONS, 3), (-1, 1));
        assert_eq!(orthogonal(DIRECTIONS, 0), (1, 0));
        assert_eq!(orthogonal(DIRECTIONS, 7), (1, -1));
    }

    #[test]
    fn part_1_correct() {
        assert_eq!(part_1(), 2554)
    }

    #[test]
    fn hashset_works_the_way_i_think() {
        let mut unique_locations: HashSet<(i32, i32)> = HashSet::new();
        unique_locations.insert((1, 2));
        unique_locations.insert((2, 3));
        unique_locations.insert((2, 3));
        unique_locations.insert((3, 4));

        assert_eq!(unique_locations.len(), 3);
    }

    #[test]
    fn given_example_part_2() {
        let test_data = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ];
        let test_vec = test_data.iter().flatten().cloned().collect();

        let grid = Grid::from_vec(test_vec, test_data[0].len());
        let count = iter_thru_grid(&grid, "x-mas");

        assert_eq!(count, 9);
    }

    #[test]
    fn given_example_part_2_2() {
        let test_data = vec![
            vec!['M', 'A', 'S'],
            vec!['S', 'A', 'M'],
            vec!['M', 'A', 'S'],
        ];
        let test_vec = test_data.iter().flatten().cloned().collect();

        let grid = Grid::from_vec(test_vec, test_data[0].len());
        let count = iter_thru_grid(&grid, "x-mas");

        assert_eq!(count, 1);
    }
}
