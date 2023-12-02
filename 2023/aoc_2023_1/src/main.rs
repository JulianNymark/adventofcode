use regex::Regex;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();

    let re = Regex::new(r"[0-9]").unwrap();

    for line in file_contents.lines() {
        let first_match = re.find(line).unwrap();
        // let reversed_line = reverse(&line);
        let last_match = re.find(reverse(line)).unwrap();
        // println!("{line}");
        println!("first_match: {}", first_match.as_str());
        println!("last_match: {}", last_match.as_str());
    }

    // println!("{}", file_contents.unwrap());
}

fn reverse(s: &str) -> &str {
    s.chars().rev().collect::<String>().as_str()
}
