use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("{}", solve_part_one(&input));
}

fn solve_part_one(input: &str) -> isize {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let mut before_rules: HashMap<isize, HashSet<isize>> = HashMap::new();
    for rule in rules.lines() {
        let (x, y) = rule.split_once("|").unwrap();
        before_rules
            .entry(y.parse().unwrap())
            .or_insert_with(|| HashSet::new())
            .insert(x.parse().unwrap());
    }

    return updates
        .lines()
        .map(|line| -> Vec<isize> { line.split(",").map(|x| x.parse().unwrap()).collect() })
        .filter(|update| {
            update.iter().enumerate().all(|(i, current)| {
                update[i + 1..].iter().all(|next| {
                    before_rules
                        .get(current)
                        .map_or(true, |before| !before.contains(next))
                })
            })
        })
        .map(|update| update[update.len() / 2])
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        let input = [
            "47|53",
            "97|13",
            "97|61",
            "97|47",
            "75|29",
            "61|13",
            "75|53",
            "29|13",
            "97|29",
            "53|29",
            "61|53",
            "97|53",
            "61|29",
            "47|13",
            "75|47",
            "97|75",
            "47|61",
            "75|61",
            "47|29",
            "75|13",
            "53|13",
            "",
            "75,47,61,53,29",
            "97,61,53,29,13",
            "75,29,13",
            "75,97,47,61,53",
            "61,13,29",
            "97,13,75,29,47",
        ]
        .join("\n");
        assert_eq!(solve_part_one(&input), 143);
    }
}
