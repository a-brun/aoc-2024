use std::fmt::Display;
use std::time::Instant;

use aoc_2024::{format_duration, get_input_as_string};

fn count_xmas(input: &str) -> u32 {
    let mut count: u32 = 0;
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let max_j = grid[0].len() - 4;
    let max_i = grid.len() - 4;
    let min_j = 3;
    let min_i = 3;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'X' {
                // E
                if j <= max_j
                    && grid[i][j + 1] == 'M'
                    && grid[i][j + 2] == 'A'
                    && grid[i][j + 3] == 'S'
                {
                    count += 1;
                }
                // SE
                if i <= max_i
                    && j <= max_j
                    && grid[i + 1][j + 1] == 'M'
                    && grid[i + 2][j + 2] == 'A'
                    && grid[i + 3][j + 3] == 'S'
                {
                    count += 1;
                }
                // S
                if i <= max_i
                    && grid[i + 1][j] == 'M'
                    && grid[i + 2][j] == 'A'
                    && grid[i + 3][j] == 'S'
                {
                    count += 1;
                }
                // SW
                if i <= max_i
                    && j >= min_j
                    && grid[i + 1][j - 1] == 'M'
                    && grid[i + 2][j - 2] == 'A'
                    && grid[i + 3][j - 3] == 'S'
                {
                    count += 1;
                }
                // W
                if j >= min_j
                    && grid[i][j - 1] == 'M'
                    && grid[i][j - 2] == 'A'
                    && grid[i][j - 3] == 'S'
                {
                    count += 1;
                }
                // NW
                if i >= min_i
                    && j >= min_j
                    && grid[i - 1][j - 1] == 'M'
                    && grid[i - 2][j - 2] == 'A'
                    && grid[i - 3][j - 3] == 'S'
                {
                    count += 1;
                }
                // N
                if i >= min_i
                    && grid[i - 1][j] == 'M'
                    && grid[i - 2][j] == 'A'
                    && grid[i - 3][j] == 'S'
                {
                    count += 1;
                }
                // NE
                if i >= min_i
                    && j <= max_j
                    && grid[i - 1][j + 1] == 'M'
                    && grid[i - 2][j + 2] == 'A'
                    && grid[i - 3][j + 3] == 'S'
                {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_x_mas(input: &str) -> u32 {
    let mut count: u32 = 0;
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            if grid[i][j] == 'A' {
                let mut sub_count: u32 = 0;

                if grid[i - 1][j - 1] == 'M' && grid[i + 1][j + 1] == 'S' {
                    sub_count += 1;
                }
                if grid[i - 1][j - 1] == 'S' && grid[i + 1][j + 1] == 'M' {
                    sub_count += 1;
                }
                if grid[i + 1][j - 1] == 'M' && grid[i - 1][j + 1] == 'S' {
                    sub_count += 1;
                }
                if grid[i + 1][j - 1] == 'S' && grid[i - 1][j + 1] == 'M' {
                    sub_count += 1;
                }

                if sub_count >= 2 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let p1 = count_xmas(input);
    let p2 = count_x_mas(input);

    (p1, p2)
}

fn main() {
    let input = get_input_as_string("day04.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[cfg(test)]
mod tests {
    use crate::count_x_mas;
    use crate::count_xmas;

    #[test]
    fn test_p1() {
        let input = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
        let res = count_xmas(&input);
        assert_eq!(18, res);
    }

    #[test]
    fn test_p2() {
        let input = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

        let res = count_x_mas(&input);

        assert_eq!(9, res);
    }
}
