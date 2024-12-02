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

fn check_valid(_report: Report) -> bool {
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

fn part_2() {
    todo!();
}

fn main() {
    part_1();
    part_2();
}
