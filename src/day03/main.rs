use regex::Regex;
use std::fmt::Display;
use std::time::Instant;

use aoc_2024::get_input_as_string;

fn multiplicate_some(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum: u32 = 0;

    for (_, [n1, n2]) in re.captures_iter(input).map(|r| r.extract()) {
        sum += n1.parse::<u32>().unwrap() * n2.parse::<u32>().unwrap();
    }

    sum
}

fn multiplicate_restrictive(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))").unwrap();
    let mut sum: u32 = 0;
    let mut is_enabled: bool = true;

    for (_, m) in re.captures_iter(input).enumerate() {
        match m.get(0).unwrap().as_str() {
            "do()" => is_enabled = true,
            "don't()" => is_enabled = false,
            _ => {
                if is_enabled {
                    let n1 = m.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    let n2 = m.get(2).unwrap().as_str().parse::<u32>().unwrap();

                    sum += n1 * n2;
                }
            }
        }
    }

    sum
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let p1 = multiplicate_some(input);
    let p2 = multiplicate_restrictive(input);

    (p1, p2)
}

fn main() {
    let input = get_input_as_string("day03.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

#[cfg(test)]
mod tests {
    use crate::multiplicate_restrictive;
    use crate::multiplicate_some;

    #[test]
    fn test_p1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let res = multiplicate_some(&input);
        assert_eq!(161, res);
    }

    #[test]
    fn test_p2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let res = multiplicate_restrictive(&input);

        assert_eq!(48, res);
    }
}
