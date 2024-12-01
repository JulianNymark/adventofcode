//! [Day one](https://adventofcode.com/2024/day/1) was pretty fun!
//!
//! Couldn't quite understand the "logic" behind the similarity function
//! (how it was supposed to represent similarity). But other than that
//! good stuff!
//!
//! I'm at the start of my rust journey, so this was amazing as a
//! way to learn so much!
//!
//! Below are some random notes!
//!
//! # Crazy compiler output
//!
//! Rust has no `++` operator, so it was fun to see the rust compiler include suggestions to
//! convert code from _other_ programming languages syntax into rust!... what!?
//! ```
//! count ++;
//! ```
//! The compiler suggests that it should be:
//! ```
//! count += 1;
//! ```
//!
//! On many occasions it really seems to _know_ what I wanted to do, and suggest code changes
//! to do the thing I wanted (but correctly). I don't use compilers much, but this one is amazing!
//!
//! # `.into_iter()`
//!
//! `.into_iter()` will sometimes create T, and sometimes &T (I'm not sure about when it does
//! which). So having the choices below:
//! ```
//! list2.iter().copied().filter(|e| e == value).collect() // probably the "best" here.
//! list2.iter().map(|&e| e).filter(|e| e == value).collect() // explicit dereference mapping
//! list2.iter().filter(|&e| e == value).collect() // gives type Vec<&&str>
//! ```
//! Can't easily be replaced by using `.into_iter()` over `.iter()` with the hopes of avoiding
//! the double reference `&&` you would get in all those choices. Using `.copied()` seems to
//! be the de facto way to solve the "When you have an iterator over `&T`, but you need an iterator over `T`" issue.
//!
//! # more choices
//!
//! For the code segment that splits and iterates over the input.txt contents:
//! ```
//! let mut split_lines_iter = line.split_whitespace();
//! let list_a_location = split_lines_iter.next().unwrap();
//! let list_b_location = split_lines_iter.next().unwrap();
//! ```
//! I could have gone with a loop extraction like this:
//! ```
//! for (idx, location_id) in line.split_whitespace().enumerate() {
//!      if (idx == 0) {
//!       ...
//!      }
//! }
//! ```
//! I opted for a the first more explicit extraction however.
//! I like it more since we expect / assume exactly 2 elements
//! per line.

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

/// Sort a list of strings
///
/// currently does not look into the numerical value of the string itself.
/// it's a simple string sort.
pub fn sort_string_nums(string_nums: Vec<&str>) -> Vec<&str> {
    let mut copy = string_nums.clone();
    copy.sort();
    copy
}

/// Strange algorithm for day 1 part 2.
///
/// I have no clue how this scoring system is supposed to extract "similarity" between the two
/// lists
fn wack_similarity_score_algo(list1: &Vec<&str>, list2: &Vec<&str>) -> u32 {
    let mut similarity = 0;

    for (_idx, value) in list1.into_iter().enumerate() {
        let num1 = value.parse::<i32>().unwrap();

        let filtered_list: Vec<&str> = list2.iter().copied().filter(|e| e == value).collect();
        let count: i32 = filtered_list.len().try_into().unwrap();
        similarity += count * num1;
    }

    similarity.try_into().unwrap()
}

/// Sum of Absolute Differences
///
/// Assumes both iterables are of the same length.
pub fn sad(list1: &Vec<&str>, list2: &Vec<&str>) -> u32 {
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
