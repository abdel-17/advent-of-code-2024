use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("{}", solve_part_two(&input));
}

fn solve_part_two(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let columns = grid[0].len();
    return (1..rows - 1)
        .map(|i| (1..columns - 1).filter(|j| is_xmas(&grid, i, *j)).count())
        .sum();
}

fn is_xmas(grid: &Vec<Vec<char>>, cx: usize, cy: usize) -> bool {
    // M| |S
    //  |A|
    // M| |S
    return is_mas(grid[cx - 1][cy - 1], grid[cx][cy], grid[cx + 1][cy + 1])
        && is_mas(grid[cx - 1][cy + 1], grid[cx][cy], grid[cx + 1][cy - 1]);
}

fn is_mas(c0: char, c1: char, c2: char) -> bool {
    if c0 == 'M' && c1 == 'A' && c2 == 'S' {
        return true;
    }
    if c2 == 'M' && c1 == 'A' && c0 == 'S' {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_two() {
        let input = [
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ]
        .join("\n");
        assert_eq!(solve_part_two(&input), 9);
    }
}
