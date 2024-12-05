use std::fs::read_to_string;

use regex::Regex;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mul_pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let sum: isize = mul_pattern
        .captures_iter(&input)
        .map(|capture| {
            let a: isize = capture[1].parse().unwrap();
            let b: isize = capture[2].parse().unwrap();
            return a * b;
        })
        .sum();
    println!("{}", sum);
}
