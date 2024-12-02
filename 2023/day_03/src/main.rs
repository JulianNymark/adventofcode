use regex::Regex;
use std::collections::HashMap;

fn main() {
    part_1("./input.txt");
}

pub trait ValueMapStore {
    fn set_value(&mut self, x: i32, y: i32, value: String);
    fn get_value(&self, x: i32, y: i32) -> String;
}

struct PartsMap {
    data: HashMap<String, String>,
}

impl ValueMapStore for PartsMap {
    fn set_value(&mut self, x: i32, y: i32, value: String) {
        self.data.insert(format!("{},{}", x, y), value);
    }
    fn get_value(&self, x: i32, y: i32) -> String {
        self.data.get(&format!("{},{}", x, y)).unwrap().to_string()
    }
}

fn mark_neighbors(part_map: &PartsMap, processed_map: &PartsMap, x: i32, y: i32) -> Vec<i32> {
    for y_offset in -1..1 {
        for x_offset in -1..1 {
            let neighbor = part_map.get_value(x + x_offset, y + y_offset);
            // if neighbor is a number,
            // walk it to leftmost number
            // check if already processed (lookup in separate map)
            // mark that x,y in a separate map (processed)
            println!("neighbor: {}", neighbor)
        }
    }
    vec![0, 1]
}

fn part_1(file: &str) {
    let mut parts_map = PartsMap {
        data: HashMap::new(),
    };
    let mut processed_map = PartsMap {
        data: HashMap::new(),
    };

    use std::fs;
    let file_contents = fs::read_to_string(file).unwrap();

    for (y, line) in file_contents.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            parts_map.set_value(
                x.try_into().unwrap(),
                y.try_into().unwrap(),
                character.to_string(),
            );
        }
    }

    let re_number = Regex::new(r"[^0-9.]").unwrap();

    for y in 0..139 {
        for x in 0..139 {
            let value = parts_map.get_value(x, y);
            if re_number.is_match(value.as_str()) {
                // println!("re_number: {} at {},{}", value, x, y);
                mark_neighbors(&parts_map, &processed_map, x, y);
            }
        }
    }

    // iterate and sum all marked neighbors (look ahead -> x10 each)

    // check neighbors?
    println!("{}", parts_map.get_value(5, 0));
}
