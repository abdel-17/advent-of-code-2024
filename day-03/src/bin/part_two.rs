use std::fs::read_to_string;

use regex::Regex;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mul_pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum = 0;
    let mut start = 0;
    while start < input.len() {
        let slice = &input[start..];
        if let Some(dont_match) = slice.find("don't()") {
            let dont_index = start + dont_match;
            sum += sum_mul(&mul_pattern, &input[start..dont_index]);

            if let Some(do_match) = input[dont_index..].find("do()") {
                let do_index = dont_index + do_match;
                start = do_index + 4;
            } else {
                break;
            }
        } else {
            sum += sum_mul(&mul_pattern, slice);
            break;
        }
    }
    println!("{}", sum);
}

fn sum_mul(mul_pattern: &Regex, slice: &str) -> isize {
    mul_pattern
        .captures_iter(slice)
        .map(|capture| {
            let a: isize = capture[1].parse().unwrap();
            let b: isize = capture[2].parse().unwrap();
            return a * b;
        })
        .sum()
}
