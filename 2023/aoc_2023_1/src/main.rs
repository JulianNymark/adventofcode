use regex::Regex;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();

    let re = Regex::new(r"[0-9]").unwrap();

    let mut total = 0;

    for line in file_contents.lines() {
        let reversed_line = reverse(line);

        let first_match = re.find(line).unwrap();
        let last_match = re.find(reversed_line.as_str()).unwrap();

        let first_num = first_match.as_str().parse::<u32>().unwrap();
        let last_num = last_match.as_str().parse::<u32>().unwrap();

        let calibration_value = (first_num * 10) + last_num;

        total += calibration_value;
    }

    println!("{}", total);
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect::<String>()
}
