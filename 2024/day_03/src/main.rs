fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let lines = utils::lines("./input.txt");
    let mulsum = mull_over_lines(lines.iter().map(|x| x.as_str()).collect());

    println!("part 1: {}", mulsum);
}

fn part_2() {
    let lines = utils::lines("./input.txt");
    let single_line = lines.concat(); // WHY: treat all lines as one program
    let stripped_donts = strip_donts(single_line.as_str());
    let mulsum = mul_sum_line(stripped_donts.as_str());

    println!("part 2: {}", mulsum);
}

fn strip_donts(line: &str) -> String {
    let re = regex::Regex::new(r"(^|do\(\)).*?($|don't\(\))").unwrap();

    let mut fields: Vec<String> = vec![];

    for (s1, [_, _]) in re.captures_iter(line).map(|caps| caps.extract()) {
        let concatted = s1.to_string();
        fields.push(concatted);
    }

    fields.concat()
}

fn extract_muls(line: &str) -> Vec<(i32, i32)> {
    let re = regex::Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut fields: Vec<(i32, i32)> = vec![];
    for (_, [num1, num2]) in re.captures_iter(line).map(|caps| caps.extract()) {
        fields.push((num1.parse().unwrap(), num2.parse().unwrap()))
    }

    fields
}

fn mul_sum_line(line: &str) -> i32 {
    extract_muls(line).iter().map(|x| x.0 * x.1).sum()
}

fn mull_over_lines(lines: Vec<&str>) -> i32 {
    lines.iter().map(|x| mul_sum_line(x)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_donts() {}

    #[test]
    fn extract_muls_1() {
        assert_eq!(
            extract_muls("mul(2,4)..mul()..mul(,)..mul(2)..mul(2,)..mul(3,5)"),
            vec![(2, 4), (3, 5)]
        )
    }

    #[test]
    fn extract_muls_2() {
        assert_eq!(
            extract_muls("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            vec![(2, 4), (5, 5), (11, 8), (8, 5)]
        )
    }

    #[test]
    fn mul_sum_line_1() {
        assert_eq!(
            mul_sum_line("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        )
    }

    #[test]
    fn mull_it_over_1() {
        assert_eq!(
            mull_over_lines(vec![
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
                "mul(2,4)"
            ]),
            169
        )
    }
}
