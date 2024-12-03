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

#[cfg(test)]
mod tests {
    use super::*;

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
}
