use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut count_safe = 0;
    for line in reader.lines() {
        let numbers: Vec<isize> = line
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let increasing = numbers[0] < numbers[1];
        let decreasing = numbers[0] > numbers[1];
        let safe = numbers
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .all(|diff| {
                (increasing && 1 <= diff && diff <= 3) || (decreasing && -3 <= diff && diff <= -1)
            });

        if safe {
            count_safe += 1;
        }
    }
    println!("{}", count_safe);
}
