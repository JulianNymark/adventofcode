use regex::Regex;
use std::fs;

fn main() {
    part_1("input.txt");
    part_2("input.txt");
}

fn part_1(file: &str) {
    let file_contents = fs::read_to_string(file).unwrap();

    let mut total = 0;

    for line in file_contents.lines() {
        let calibration_value = plain_christmas_match(line);
        total += calibration_value;
    }

    println!("part 1: {}", total);
}

fn part_2(file: &str) {
    let file_contents = fs::read_to_string(file).unwrap();

    let mut total = 0;

    for line in file_contents.lines() {
        let calibration_value = funky_christmas_match(line);
        total += calibration_value;
    }

    println!("part 2: {}", total)
}

fn plain_christmas_match(line: &str) -> i32 {
    let reversed_line = reverse(line);

    let re_plain = Regex::new(r"[0-9]").unwrap();
    let first_match = re_plain.find(line).unwrap();
    let last_match = re_plain.find(reversed_line.as_str()).unwrap();

    let first_num = first_match.as_str().parse::<i32>().unwrap();
    let last_num = last_match.as_str().parse::<i32>().unwrap();

    let calibration_value = (first_num * 10) + last_num;

    calibration_value
}

fn funky_christmas_match(line: &str) -> i32 {
    let reversed_line = reverse(line);

    let re_funky = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let re_funky_reverse =
        Regex::new(r"[0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();

    let first_match = re_funky.find(line).unwrap();
    let last_match = re_funky_reverse.find(reversed_line.as_str()).unwrap();

    let first_dehumanified = dehumanify_string(first_match.as_str());
    let last_dehumanified = dehumanify_string(last_match.as_str());

    let first_num = first_dehumanified.unwrap().parse::<i32>().unwrap();
    let last_num = last_dehumanified.unwrap().parse::<i32>().unwrap();

    let calibration_value = (first_num * 10) + last_num;

    calibration_value
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn dehumanify_string(s: &str) -> Result<String, String> {
    let m = match s {
        "one" | "eno" => "1",
        "two" | "owt" => "2",
        "three" | "eerht" => "3",
        "four" | "ruof" => "4",
        "five" | "evif" => "5",
        "six" | "xis" => "6",
        "seven" | "neves" => "7",
        "eight" | "thgie" => "8",
        "nine" | "enin" => "9",
        _ => s,
    };
    Ok(m.to_string())
}
