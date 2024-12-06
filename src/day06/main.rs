use std::fmt::Display;
use std::time::Instant;

use aoc_2024::{format_duration, get_input_as_string};

fn guard_walk(input: &str) -> (usize, usize) {
    let grid = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut initial_position: (usize, usize) = (0, 0);

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '^' {
                initial_position = (i, j);
            }
        }
    }

    let r1 = march(grid.clone(), initial_position.clone()).unwrap();

    let mut infinite_loop: usize = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '.' {
                let mut new_grid = grid.clone();
                new_grid[i][j] = '#';
                match march(new_grid, initial_position.clone()) {
                    Ok(_) => continue,
                    Err(_) => infinite_loop += 1,
                }
            }
        }
    }

    (r1, infinite_loop)
}

fn march(mut grid: Vec<Vec<char>>, mut curr_position: (usize, usize)) -> Result<usize, String> {
    let mut steps_count: u32 = 0;
    loop {
        let (i, j) = curr_position;
        match grid[i][j] {
            '^' => {
                if i == 0 {
                    grid[i][j] = 'x';
                    return Ok(count_distinct_steps(&grid));
                }
                if grid[i - 1][j] == '#' {
                    grid[i][j] = '>';
                } else {
                    grid[i][j] = 'x';
                    grid[i - 1][j] = '^';
                    curr_position = (i - 1, j);
                }
            }
            '>' => {
                if j == grid[i].len() - 1 {
                    grid[i][j] = 'x';
                    return Ok(count_distinct_steps(&grid));
                }
                if grid[i][j + 1] == '#' {
                    grid[i][j] = 'v';
                } else {
                    grid[i][j] = 'x';
                    grid[i][j + 1] = '>';
                    curr_position = (i, j + 1);
                }
            }
            'v' => {
                if i == grid.len() - 1 {
                    grid[i][j] = 'x';
                    return Ok(count_distinct_steps(&grid));
                }
                if grid[i + 1][j] == '#' {
                    grid[i][j] = '<';
                } else {
                    grid[i][j] = 'x';
                    grid[i + 1][j] = 'v';
                    curr_position = (i + 1, j);
                }
            }
            '<' => {
                if j == 0 {
                    grid[i][j] = 'x';
                    return Ok(count_distinct_steps(&grid));
                }
                if grid[i][j - 1] == '#' {
                    grid[i][j] = '^';
                } else {
                    grid[i][j] = 'x';
                    grid[i][j - 1] = '<';
                    curr_position = (i, j - 1);
                }
            }
            _ => panic!("We lost track of the guard."),
        }

        steps_count += 1;

        if steps_count > 10000 {
            return Err(String::from("Too many steps"));
        }
    }
}

fn count_distinct_steps(grid: &Vec<Vec<char>>) -> usize {
    grid.into_iter()
        .map(|row| row.into_iter().filter(|v| **v == 'x').count())
        .sum()
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let (p1, p2) = guard_walk(input);

    (p1, p2)
}

fn main() {
    let input = get_input_as_string("day06.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[cfg(test)]
mod tests {
    use crate::guard_walk;

    #[test]
    fn test_p1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...

";
        let (res, _) = guard_walk(&input);
        assert_eq!(41, res);
    }

    #[test]
    fn test_p2() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...

";

        let (_, res) = guard_walk(&input);

        assert_eq!(6, res);
    }
}
