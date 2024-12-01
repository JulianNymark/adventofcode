use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(sad(&vec!["1", "2"], &vec!["5", "7"]), 9);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            sort_string_nums(vec!["5", "1", "3", "0"]),
            vec!["0", "1", "3", "5"]
        )
    }

    #[test]
    fn test_example() {
        assert_eq!(
            sad(
                &sort_string_nums(vec!["3", "4", "2", "1", "3", "3",]),
                &sort_string_nums(vec!["4", "3", "5", "3", "9", "3",])
            ),
            11
        )
    }

    #[test]
    fn test_3() {
        assert_eq!(
            wack_similarity_score_algo(
                &vec!["3", "4", "2", "1", "3", "3",],
                &vec!["4", "3", "5", "3", "9", "3",]
            ),
            31
        )
    }
}

fn sort_string_nums(string_nums: Vec<&str>) -> Vec<&str> {
    let mut copy = string_nums.clone();
    copy.sort();
    copy
}

fn wack_similarity_score_algo(list1: &Vec<&str>, list2: &Vec<&str>) -> u32 {
    let mut similarity = 0;

    for (_idx, value) in list1.into_iter().enumerate() {
        let num1 = value.parse::<i32>().unwrap();
        let mut count = 0;

        for (_idx2, value2) in list2.into_iter().enumerate() {
            let num2 = value2.parse::<i32>().unwrap();

            if num1 == num2 {
                count += 1;
            }
        }
        similarity += count * num1;
    }

    similarity.try_into().unwrap()
}

/*
SAD == Sum of Absolute Differences
ASSUMPTION: both iterables must be of the same length
*/
fn sad(list1: &Vec<&str>, list2: &Vec<&str>) -> u32 {
    let mut sum = 0;

    for (idx, value) in list1.into_iter().enumerate() {
        let num1 = value.parse::<i32>().unwrap();
        let num2 = list2[idx].parse::<i32>().unwrap();
        sum += (num1 - num2).abs();
    }
    return sum.try_into().unwrap();
}

fn part_1() {
    let file_name = "./input.txt";
    let input = fs::read_to_string(file_name)
        .expect(format!("expected to read file {}", file_name).as_str());

    let mut list_a: Vec<&str> = Vec::new();
    let mut list_b: Vec<&str> = Vec::new();

    for line in input.split("\n") {
        // variant 1: loop
        /*
        for (idx, location_id) in line.split_whitespace().enumerate() {
        println!("({}) {}", idx, location_id);
        //     if (idx == 0) {
        //      ...
        //     }
        }
        */

        // variant 2: more explicit extraction
        // I like this one since we expect / assume 2 elements
        // per line

        // WHY: the last entry will be None (from split on last `\n` in file)
        if line != "" {
            let mut split_lines_iter = line.split_whitespace();
            let list_a_location = split_lines_iter.next().unwrap();
            let list_b_location = split_lines_iter.next().unwrap();

            list_a.push(list_a_location);
            list_b.push(list_b_location);
        }
    }

    println!(
        "part 1: {}",
        sad(&sort_string_nums(list_a), &sort_string_nums(list_b))
    );
}

fn part_2() {
    let file_name = "./input.txt";
    let input = fs::read_to_string(file_name)
        .expect(format!("expected to read file {}", file_name).as_str());

    let mut list_a: Vec<&str> = Vec::new();
    let mut list_b: Vec<&str> = Vec::new();

    for line in input.split("\n") {
        // WHY: the last entry will be None (from split on last `\n` in file)
        if line != "" {
            let mut split_lines_iter = line.split_whitespace();
            let list_a_location = split_lines_iter.next().unwrap();
            let list_b_location = split_lines_iter.next().unwrap();

            list_a.push(list_a_location);
            list_b.push(list_b_location);
        }
    }

    println!("part 2: {}", wack_similarity_score_algo(&list_a, &list_b));
}

fn main() {
    part_1();
    part_2();
}
