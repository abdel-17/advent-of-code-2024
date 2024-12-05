use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut first: Vec<isize> = Vec::new();
    let mut second: Vec<isize> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut numbers = line.split_whitespace();
        let a: isize = numbers.next().unwrap().parse().unwrap();
        let b: isize = numbers.next().unwrap().parse().unwrap();
        first.push(a);
        second.push(b);
    }

    first.sort();
    second.sort();

    let total_distance: usize = zip(first, second).map(|(a, b)| a.abs_diff(b)).sum();
    println!("{}", total_distance);
}
