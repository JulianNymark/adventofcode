//! # `"{:?}"` can be used to debug! (calls not Display, but Debug... trait?)
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(score_report(vec![7, 6, 4, 2, 1]), 0)
    }

    #[test]
    fn test_2() {
        assert_eq!(score_report(vec![1, 2, 7, 8, 9]), 1)
    }

    #[test]
    fn test_3() {
        assert_eq!(score_report(vec![9, 7, 6, 2, 1]), 1)
    }

    #[test]
    fn test_4() {
        assert_eq!(score_report(vec![1, 3, 2, 4, 5]), 1)
    }

    #[test]
    fn test_5() {
        assert_eq!(score_report(vec![8, 6, 4, 4, 1]), 1)
    }

    #[test]
    fn test_6() {
        assert_eq!(score_report(vec![1, 3, 6, 7, 9]), 0)
    }

    #[test]
    fn test_7() {
        assert_eq!(score_report(vec![8, 3, 6, 7, 9]), 4)
    }

    #[test]
    fn test_8() {
        assert_eq!(score_report(dampen_report(vec![8, 3, 6, 7, 9])), 2)
    }

    #[test]
    fn test_9() {
        assert_eq!(dampen_report(vec![8, 3, 6, 7, 9]), vec![8, 6, 7, 9])
    }

    #[test]
    fn test_10() {
        assert_eq!(dampen_report(vec![1, 2, 9]), vec![1, 2])
    }

    #[test]
    fn test_11() {
        assert_eq!(dampen_report(vec![1, 2, 3, 2, 1]), vec![1, 2, 2, 1])
    }

    #[test]
    fn test_12() {
        assert_eq!(dampen_report(vec![1, 2, 10, 11, 12]), vec![1, 2, 11, 12])
    }

    #[test]
    fn test_13() {
        assert_eq!(dampen_report(vec![1, 2, 3, 4, 4]), vec![1, 2, 3, 4])
    }

    #[test]
    fn test_14() {
        assert_eq!(dampen_report(vec![1, 1, 2, 3, 4, 4]), vec![1, 2, 3, 4, 4])
    }

    #[test]
    fn test_15() {
        assert_eq!(
            dampen_report(vec![40, 41, 42, 45, 47, 51, 53, 60]),
            vec![40, 41, 42, 45, 47, 53, 60]
        )
    }

    #[test]
    fn test_16() {
        assert_eq!(score_report(vec![72, 72, 74, 72, 75, 78, 80, 86]), 3)
    }

    #[test]
    fn test_17() {
        assert_eq!(
            score_report(dampen_report(vec![72, 72, 74, 72, 75, 78, 80, 86])),
            2
        )
    }
}

type Report = Vec<i32>;

/// will remove the first bad level if found
fn dampen_report(report: Report) -> Report {
    let mut dir = 0;

    for (idx, level) in report[..report.len() - 1].iter().enumerate() {
        let next_level = report[idx + 1];

        let diff = next_level - level;

        let mut skip = false;

        if diff.abs() > 3 || diff == 0 {
            skip = true;
        }

        if diff > 0 {
            if dir == -1 {
                return [report[..idx].to_vec(), report[idx + 1..].to_vec()].concat();
            }
            if dir == 0 {
                dir = 1
            }
        } else if diff < 0 {
            if dir == 1 {
                return [report[..idx].to_vec(), report[idx + 1..].to_vec()].concat();
            }
            if dir == 0 {
                dir = -1
            }
        }
        if skip {
            return [report[..idx + 1].to_vec(), report[idx + 2..].to_vec()].concat();
        }
    }
    return report;
}

/// check if a list of numbers follows some rules
/// returns a badness score
///
/// assumes a vec of at least 2 numbers.
///
/// 555 ... x ... 565: could check all 10 manually
fn score_report(report: Report) -> i32 {
    let mut badnesss = 0;
    let mut dir = 0;

    for (idx, level) in report[..report.len() - 1].iter().enumerate() {
        let next_level = report[idx + 1];

        let diff = next_level - level;

        if diff > 0 {
            if dir == -1 {
                badnesss += 1;
            }
            if diff.abs() > 3 {
                badnesss += 1;
            }
            if dir == 0 {
                dir = 1
            }
        } else if diff < 0 {
            if dir == 1 {
                badnesss += 1;
            }
            if diff.abs() > 3 {
                badnesss += 1;
            }
            if dir == 0 {
                dir = -1
            }
        } else {
            badnesss += 1;
        }
    }
    return badnesss;
}

fn get_reports(file_name: &str) -> Vec<Report> {
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

    reports
}

fn part_1() {
    let reports = get_reports("./input.txt");

    let mut safe_count = 0;

    for report in reports {
        if score_report(report) == 0 {
            safe_count += 1;
        }
    }

    println!("part 1: {}", safe_count);
}

fn part_2() {
    let reports = get_reports("./input.txt");

    let mut safe_count = 0;

    for report in reports {
        if score_report(dampen_report(report)) == 0 {
            safe_count += 1;
        }
    }

    println!("part 2: {}", safe_count);
}

fn main() {
    part_1();
    part_2();
}
