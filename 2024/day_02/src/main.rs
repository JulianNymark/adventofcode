//! # `"{:?}"` can be used to debug! (calls not Display, but Debug... trait?)
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(check_valid(vec![7, 6, 4, 2, 1]), true)
    }

    #[test]
    fn test_2() {
        assert_eq!(check_valid(vec![1, 2, 7, 8, 9]), false)
    }

    #[test]
    fn test_3() {
        assert_eq!(check_valid(vec![9, 7, 6, 2, 1]), false)
    }

    #[test]
    fn test_4() {
        assert_eq!(check_valid(vec![1, 3, 2, 4, 5]), false)
    }

    #[test]
    fn test_5() {
        assert_eq!(check_valid(vec![8, 6, 4, 4, 1]), false)
    }

    #[test]
    fn test_6() {
        assert_eq!(check_valid(vec![1, 3, 6, 7, 9]), true)
    }
}

type Report = Vec<i32>;

enum Direction {
    Up,
    Down,
}

/// check if a list of numbers follows some rules
///
/// assumes a vec of at least 2 numbers.
fn check_valid(report: Report) -> bool {
    let dir: Direction;

    if report[0] == report[1] || (report[0] - report[1]).abs() > 3 {
        return false;
    }

    if report[1] > report[0] {
        dir = Direction::Up;
    } else {
        dir = Direction::Down;
    }

    let mut last_seen = report[1];

    // really don't like this way of doing it
    for &value in &report[2..] {
        if value > last_seen {
            match dir {
                Direction::Up => {
                    let diff = value - last_seen;
                    if diff > 3 || diff <= 0 {
                        return false;
                    }
                }
                _ => return false,
            }
        } else if value < last_seen {
            match dir {
                Direction::Down => {
                    let diff = last_seen - value;
                    if diff > 3 || diff <= 0 {
                        return false;
                    }
                }
                _ => return false,
            }
        } else {
            return false;
        }
        last_seen = value;
    }

    true
}

fn part_1() {
    let file_name = "./input.txt";
    let input = fs::read_to_string(file_name)
        .expect(format!("expected to read file {}", file_name).as_str());

    let mut reports: Vec<Report> = Vec::new();

    for line in input.split("\n") {
        // WHY: the last entry will be None (from split on last `\n` in file)
        if line != "" {
            let split_lines_iter = line.split_whitespace();
            let mut report: Report = Vec::new();
            for value in split_lines_iter {
                let num = value.parse::<i32>().unwrap();
                report.push(num);
            }
            reports.push(report);
        }
    }

    let mut safe_count = 0;
    for report in reports {
        if check_valid(report) {
            safe_count += 1;
        }
    }

    println!("part 1: {}", safe_count);
}

/* fn part_2() {
    todo!();
} */

fn main() {
    part_1();
    // part_2();
}
