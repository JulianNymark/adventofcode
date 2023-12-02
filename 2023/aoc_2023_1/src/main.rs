use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt");

    println!("{}", file_contents.unwrap())
}
