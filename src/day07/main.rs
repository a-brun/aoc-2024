use std::fmt::Display;
use std::time::Instant;

use aoc_2024::{format_duration, get_input_as_string};

fn equation_fixer(input: &str) -> (u64, u64) {
    let mut p1: u64 = 0;
    let mut p2: u64 = 0;
    for l in input.split("\n").filter(|l| !l.is_empty()) {
        let b = l.split(": ").collect::<Vec<&str>>();
        let [left, right] = b.as_slice() else {
            panic!("Invalid input")
        };

        let target: u64 = left.parse::<u64>().unwrap();
        let mut remaining_numbers: Vec<u64> = right
            .split(" ")
            .filter(|v| !v.is_empty())
            .map(|v| v.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let initial: u64 = remaining_numbers.remove(0);
        if compute(target, initial, remaining_numbers.clone(), false) {
            p1 += target;
        }
        if compute(target, initial, remaining_numbers.clone(), true) {
            p2 += target;
        }
    }

    (p1, p2)
}

fn compute(target: u64, sum: u64, mut remaining_numbers: Vec<u64>, use_concat: bool) -> bool {
    let n = remaining_numbers.remove(0);

    if remaining_numbers.len() == 0 {
        return sum * n == target
            || sum + n == target
            || use_concat && format!("{}{}", sum, n).parse::<u64>().unwrap() == target;
    }

    let add_sum = sum + n;
    let mut add_c: bool = false;
    if add_sum <= target {
        let r = compute(target, add_sum, remaining_numbers.clone(), use_concat);
        add_c = r;
    }

    let mult_sum = sum * n;
    let mut mult_c: bool = false;
    if mult_sum <= target {
        let r = compute(target, mult_sum, remaining_numbers.clone(), use_concat);
        mult_c = r;
    }

    let mut concat_c: bool = false;
    if use_concat {
        let concat_sum = format!("{}{}", sum, n).parse::<u64>().unwrap();

        let r = compute(target, concat_sum, remaining_numbers.clone(), use_concat);
        concat_c = r;
    }

    return add_c || mult_c || concat_c;
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let (p1, p2) = equation_fixer(input);

    (p1, p2)
}

fn main() {
    let input = get_input_as_string("day07.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[cfg(test)]
mod tests {
    use crate::equation_fixer;

    #[test]
    fn test_p1() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
        let (res, _) = equation_fixer(&input);
        assert_eq!(3749, res);
    }

    #[test]
    fn test_p2() {
        let input = "192: 17 8 14
";

        let (_, res) = equation_fixer(&input);

        assert_eq!(192, res);
    }
}
