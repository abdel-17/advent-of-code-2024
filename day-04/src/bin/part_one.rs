use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("{}", solve_part_one(&input));
}

fn solve_part_one(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let columns = grid[0].len();

    let row_matches: usize = grid
        .iter()
        .map(|row| row.windows(4).filter(|slice| is_xmas(slice)).count())
        .sum();

    let column_matches: usize = (0..columns)
        .map(|j| {
            (3..rows)
                .map(|i| [grid[i - 3][j], grid[i - 2][j], grid[i - 1][j], grid[i][j]])
                .filter(|slice| is_xmas(slice))
                .count()
        })
        .sum();

    let diagonal_matches: usize = (3..rows)
        .map(|i| {
            (3..columns)
                .map(|j| {
                    [
                        grid[i - 3][j - 3],
                        grid[i - 2][j - 2],
                        grid[i - 1][j - 1],
                        grid[i][j],
                    ]
                })
                .filter(|slice| is_xmas(slice))
                .count()
        })
        .sum();

    let reverse_diagonal_matches: usize = (3..rows)
        .map(|i| {
            (3..columns)
                .rev()
                .map(|j| {
                    [
                        grid[i - 3][j],
                        grid[i - 2][j - 1],
                        grid[i - 1][j - 2],
                        grid[i][j - 3],
                    ]
                })
                .filter(|slice| is_xmas(slice))
                .count()
        })
        .sum();

    return row_matches + column_matches + diagonal_matches + reverse_diagonal_matches;
}

fn is_xmas(slice: &[char]) -> bool {
    if slice[0] == 'X' && slice[1] == 'M' && slice[2] == 'A' && slice[3] == 'S' {
        return true;
    }
    if slice[3] == 'X' && slice[2] == 'M' && slice[1] == 'A' && slice[0] == 'S' {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one() {
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
        assert_eq!(solve_part_one(&input), 18);
    }
}
