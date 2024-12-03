use std::fmt::Display;
use std::time::Instant;

use aoc_2024::get_input_as_string;

fn is_valid(report: &Vec<u32>, use_dampener: bool, validator_fn: &dyn Fn(u32, u32) -> bool) -> bool {
    for i in 0..report.len() - 1 {
        let a = report[i];
        let b = report[i + 1];

        if !validator_fn(a, b) {
            if !use_dampener {
                return false;
            }
            if i == report.len() - 2 {
                return true;
            }

            let mut c = report.clone();
            c.remove(i);
            let mut d = report.clone();
            d.remove(i + 1);

            return is_valid(&c, false, validator_fn) || is_valid(&d, false, validator_fn);
        }
    }
    true
}

fn is_valid_ascending(a: u32, b: u32) -> bool {
    b > a && b <= a + 3
}

fn is_valid_descending(a: u32, b: u32) -> bool {
    a > b && a <= b + 3
}

fn count_safe_reports(input: &str, use_dampener: bool) -> u32 {
    input.split("\n")
        .filter(|l| !l.is_empty())
            .map(|l| l.split(" ")
                .filter(|v| !v.is_empty())
                .map(|v| v.parse::<u32>()
                    .unwrap()
                ).collect::<Vec<u32>>()
            ).map(|report| {
                if is_valid(&report, use_dampener, &is_valid_ascending) || is_valid(&report, use_dampener, &is_valid_descending) {
                    return 1;
                }
                0
            }).sum()
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let p1 = count_safe_reports(input, false);
    let p2 = count_safe_reports(input, true);

    (p1, p2)
}

fn main() {
    let input = get_input_as_string("day02.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

#[cfg(test)]
mod tests {
    use crate::count_safe_reports;

    #[test]
    fn test_p1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let res = count_safe_reports(&input, false);
        assert_eq!(2, res);
    }

    #[test]
    fn test_p2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let res = count_safe_reports(&input, true);

        assert_eq!(4, res);
    }

}

