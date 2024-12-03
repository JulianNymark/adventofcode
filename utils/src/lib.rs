use std::fs;

type Lines<T> = Vec<Vec<T>>;

pub fn lines(file_name: &str) -> Vec<String> {
    let input = fs::read_to_string(file_name)
        .expect(format!("expected to read file {}", file_name).as_str());

    let mut lines: Vec<String> = Vec::new();

    for line in input.split("\n") {
        // WHY: the last entry will be None (from split on last `\n` in file)
        if line != "" {
            lines.push(line.to_string());
        }
    }

    lines
}

pub fn lines_string(file_name: &str) -> Lines<String> {
    let input = fs::read_to_string(file_name)
        .expect(format!("expected to read file {}", file_name).as_str());

    let mut lines: Lines<String> = Vec::new();

    for line in input.split("\n") {
        // WHY: the last entry will be None (from split on last `\n` in file)
        if line != "" {
            let split_lines_iter = line.split_whitespace();
            lines.push(split_lines_iter.map(|x| x.to_string()).collect());
        }
    }

    lines
}

pub fn lines_i32(file_name: &str) -> Lines<i32> {
    let input = fs::read_to_string(file_name)
        .expect(format!("expected to read file {}", file_name).as_str());

    let mut lines: Lines<i32> = Vec::new();

    for line in input.split("\n") {
        // WHY: the last entry will be None (from split on last `\n` in file)
        if line != "" {
            let split_lines_iter = line.split_whitespace();
            let mut report: Vec<i32> = Vec::new();
            for value in split_lines_iter {
                let num = value.parse::<i32>().unwrap();
                report.push(num);
            }
            lines.push(report);
        }
    }

    lines
}

/// returns a directional value based on the difference between two numbers
/// returns 1 if rising, 0 if same, -1 if falling
pub fn dir(old: i32, new: i32) -> i32 {
    if new - old > 0 {
        return 1;
    } else if new - old < 0 {
        return -1;
    } else {
        return 0;
    }
}

pub fn min(collection: &Vec<i32>) -> i32 {
    let mut min = collection[0];
    for &value in &collection[1..] {
        if value < min {
            min = value;
        }
    }
    min
}

pub fn max(collection: &Vec<i32>) -> i32 {
    let mut max = collection[0];
    for &value in &collection[1..] {
        if value > max {
            max = value;
        }
    }
    max
}

/// generate variants when dropping n elements from a collection
///
/// limitations: currently only drops 1
pub fn variants_drop(collection: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut variants = Vec::new();
    for (idx, _element) in collection[..collection.len()].iter().enumerate() {
        let variant: Vec<i32> = vec![
            collection[..idx].into_iter().cloned().collect::<Vec<i32>>(),
            collection[idx + 1..]
                .into_iter()
                .cloned()
                .collect::<Vec<i32>>(),
        ]
        .concat::<_>();
        variants.push(variant.to_vec());
    }
    variants
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants_drop_works() {
        assert_eq!(
            variants_drop(&vec![1, 2, 3]),
            vec![vec![2, 3], vec![1, 3], vec![1, 2]]
        )
    }

    #[test]
    fn lines_works() {
        assert_eq!(lines("./test.txt"), vec!["123 456", "345 678"]);
    }

    #[test]
    fn lines_i32_works() {
        assert_eq!(
            lines_i32("./test.txt"),
            vec![vec![123, 456], vec![345, 678]]
        );
    }

    #[test]
    fn lines_string_works() {
        assert_eq!(
            lines_string("./test.txt"),
            vec![vec!["123", "456"], vec!["345", "678"]]
        );
    }

    #[test]
    fn dir_works() {
        assert_eq!(dir(69, 420), 1);
        assert_eq!(dir(69, 69), 0);
        assert_eq!(dir(420, 69), -1);
    }
}
