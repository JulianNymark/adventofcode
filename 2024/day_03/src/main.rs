fn main() {
    part_1();
}

fn part_1() {
    let lines = utils::lines("./input.txt");
    let mulsum = mull_it_over(lines.iter().map(|x| x.as_str()).collect());

    println!("part 1: {}", mulsum);
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

fn mull_it_over(lines: Vec<&str>) -> i32 {
    lines.iter().map(|x| mul_sum_line(x)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

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
            mull_it_over(vec![
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
                "mul(2,4)"
            ]),
            169
        )
    }
}
