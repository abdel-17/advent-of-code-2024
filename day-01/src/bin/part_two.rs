use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut first: Vec<isize> = Vec::new();
    let mut second_counts: HashMap<isize, isize> = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut numbers = line.split_whitespace();
        let a: isize = numbers.next().unwrap().parse().unwrap();
        let b: isize = numbers.next().unwrap().parse().unwrap();
        first.push(a);
        second_counts.insert(b, get_count(&second_counts, &b) + 1);
    }

    let similarity: isize = first.iter().map(|a| a * get_count(&second_counts, a)).sum();
    println!("{}", similarity);
}

fn get_count(counts: &HashMap<isize, isize>, value: &isize) -> isize {
    *counts.get(value).unwrap_or(&0)
}
