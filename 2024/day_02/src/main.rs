//! # `"{:?}"` can be used to debug! (calls not Display, but Debug... trait?)

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
        assert_eq!(score_report(vec![1, 3, 2, 4, 5]), 2)
    }

    #[test]
    fn test_5() {
        assert_eq!(score_report(vec![8, 6, 4, 4, 1]), 2)
    }

    #[test]
    fn test_6() {
        assert_eq!(score_report(vec![1, 3, 6, 7, 9]), 0)
    }

    #[test]
    fn test_7() {
        assert_eq!(score_report(vec![8, 3, 6, 7, 9]), 2)
    }

    #[test]
    fn test_16() {
        assert_eq!(score_report(vec![72, 72, 74, 72, 75, 78, 80, 86]), 6)
    }
}

type Report = Vec<i32>;

/// will remove the first bad level if found
fn _dampen_report(report: Report) -> Report {
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
fn _score_report(report: Report) -> i32 {
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

fn score_report(report: Report) -> i32 {
    let mut last_seen = report[0];
    let mut direction = utils::dir(last_seen, report[1]);
    let mut last_direction = direction;
    let mut badness = 0;

    if direction == 0 {
        badness += 1;
    }

    for &level in &report[1..] {
        direction = utils::dir(last_seen, level);

        let abs_diff = (level - last_seen).abs();

        if direction != last_direction {
            badness += 1;
        } else if abs_diff > 3 {
            badness += 1;
        } else if direction == 0 {
            badness += 1;
        }

        last_seen = level;
        last_direction = direction;
    }

    badness
}

fn part_1() {
    let reports = utils::lines_i32("./input.txt");

    let mut safe_count = 0;

    for report in reports {
        if score_report(report) == 0 {
            safe_count += 1;
        }
    }

    println!("part 1: {}", safe_count);
}

fn part_2() {
    let reports = utils::lines_i32("./input.txt");

    let mut safe_count = 0;

    for report in reports {
        let variants = utils::variants_drop(&report);

        if utils::min(&variants.iter().map(|x| score_report(x.to_vec())).collect()) == 0 {
            safe_count += 1;
        }
    }

    println!("part 2: {}", safe_count);
}

fn main() {
    part_1();
    part_2();
}
