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
    let re_plain = Regex::new(r"[0-9]").unwrap();

    let reversed_line = reverse(line);

    let first_match = re_plain.find(line).unwrap();
    let last_match = re_plain.find(reversed_line.as_str()).unwrap();

    let first_num = first_match.as_str().parse::<i32>().unwrap();
    let last_num = last_match.as_str().parse::<i32>().unwrap();

    let calibration_value = (first_num * 10) + last_num;

    calibration_value
}

fn funky_christmas_match(line: &str) -> i32 {
    let re_plain = Regex::new(r"[0-9]").unwrap();
    let re_human_numbers = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();

    // inspired from docs: https://docs.rs/regex/latest/regex/struct.Regex.html#fallibility
    fn replace_all<E>(
        re: &Regex,
        haystack: &str,
        replacement: impl Fn(&regex::Captures) -> Result<String, E>,
    ) -> Result<String, E> {
        let mut new = String::with_capacity(haystack.len());
        let mut last_match = 0;
        for caps in re.captures_iter(haystack) {
            let m = caps.get(0).unwrap();
            new.push_str(&haystack[last_match..m.start()]);
            new.push_str(&replacement(&caps)?);
            last_match = m.end();
        }
        new.push_str(&haystack[last_match..]);
        Ok(new)
    }

    let replacement = |caps: &regex::Captures| -> Result<String, &'static str> {
        match &caps[0] {
            "one" => return Ok("1".to_string()),
            "two" => return Ok("2".to_string()),
            "three" => return Ok("3".to_string()),
            "four" => return Ok("4".to_string()),
            "five" => return Ok("5".to_string()),
            "six" => return Ok("6".to_string()),
            "seven" => return Ok("7".to_string()),
            "eight" => return Ok("8".to_string()),
            "nine" => return Ok("9".to_string()),
            _ => {
                println!("got {}", &caps[0]);
                Err("failed to parse & match a capture group")
            }
        }
    };

    let dehumanified_line = replace_all(&re_human_numbers, line, &replacement).unwrap();
    println!("dehumanified_line: {}", dehumanified_line);
    let reversed_line = reverse(&dehumanified_line);

    let first_match = re_plain
        .find(&dehumanified_line)
        .unwrap()
        .as_str()
        .to_string();
    let last_match = re_plain
        .find(reversed_line.as_str())
        .unwrap()
        .as_str()
        .to_string();

    let first_num = first_match.as_str().parse::<i32>().unwrap();
    let last_num = last_match.as_str().parse::<i32>().unwrap();

    println!("extremities: {} | {}", first_num, last_num);

    let calibration_value = (first_num * 10) + last_num;

    calibration_value
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect::<String>()
}
