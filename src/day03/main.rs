use regex::Regex;
use std::fmt::Display;
use std::time::Instant;

use aoc_2024::{format_duration, get_input_as_string};

fn multiplicate(input: &str) -> (u32, u32) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))").unwrap();
    let mut sum: u32 = 0;
    let mut partial_sum: u32 = 0;
    let mut is_enabled: bool = true;

    for m in re.captures_iter(input) {
        match m.get(0).unwrap().as_str() {
            "do()" => is_enabled = true,
            "don't()" => is_enabled = false,
            _ => {
                let n1 = m.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let n2 = m.get(2).unwrap().as_str().parse::<u32>().unwrap();

                let r = n1 * n2;
                sum += r;

                if is_enabled {
                    partial_sum += r;
                }
            }
        }
    }

    (sum, partial_sum)
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let (p1, p2) = multiplicate(input);

    (p1, p2)
}

fn main() {
    let input = get_input_as_string("day03.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[cfg(test)]
mod tests {
    use crate::multiplicate;

    #[test]
    fn test_p1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let (res, _) = multiplicate(&input);
        assert_eq!(161, res);
    }

    #[test]
    fn test_p2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let (_, res) = multiplicate(&input);

        assert_eq!(48, res);
    }
}
