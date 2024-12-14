use std::cmp::min;

use itertools::Itertools;

fn read_coords(line: &str, plus: u64) -> (u64, u64) {
    let (_, coords) = line.split_once(": ").unwrap();
    coords
        .split(", ")
        .map(|coord| coord[2..].parse::<u64>().unwrap() + plus)
        .collect_tuple()
        .unwrap()
}

fn min_tokens_1(button_a: (u64, u64), button_b: (u64, u64), prize: (u64, u64)) -> Option<u64> {
    let mut min_coins = None;
    for press_a in 0..=100 {
        let pos_after_a = (button_a.0 * press_a, button_a.1 * press_a);
        if pos_after_a.0 > prize.0 || pos_after_a.1 > prize.1 {
            break;
        }
        let press_b = (prize.0 - pos_after_a.0) / button_b.0;
        if pos_after_a.0 + press_b * button_b.0 == prize.0
            && pos_after_a.1 + press_b * button_b.1 == prize.1
        {
            dbg!(press_a);
            min_coins = Some(min(
                min_coins.unwrap_or(u64::MAX),
                press_a * 3 + press_b,
            ))
        }
    }
    min_coins
}

fn solve(a: i64, b: i64, c: i64) -> Option<((i64, i64), (i64, i64))> {

    // Helper function: Extended Euclidean Algorithm
    fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
        if b == 0 {
            return (a, 1, 0);
        }
        let (g, x1, y1) = extended_gcd(b, a % b);
        (g, y1, x1 - (a / b) * y1)
    }

    // Find GCD and coefficients x0, y0 for the equation ax + by = gcd(a, b)
    let (g, x0, y0) = extended_gcd(a, b);

    // If C is not divisible by GCD, no solutions exist
    if c % g != 0 {
        return None;
    }

    // Scale the particular solution to fit Ax + By = C
    let scale = c / g;
    let x0 = x0 * scale;
    let y0 = y0 * scale;

    // General solution: x = x0 + k(b/g), y = y0 - k(a/g)
    let b_g = b / g;
    let a_g = a / g;

    Some(((x0, b_g), (y0, a_g)))
}

fn find_positive_solutions(a: i64, b: i64, c: i64) -> Vec<(i64, i64)> {
    use std::cmp;

    // Helper function: Extended Euclidean Algorithm
    fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
        if b == 0 {
            return (a, 1, 0);
        }
        let (g, x1, y1) = extended_gcd(b, a % b);
        (g, y1, x1 - (a / b) * y1)
    }

    // Find GCD and coefficients x0, y0 for the equation ax + by = gcd(a, b)
    let (g, x0, y0) = extended_gcd(a, b);

    // If C is not divisible by GCD, no solutions exist
    if c % g != 0 {
        return vec![];
    }

    // Scale the particular solution to fit Ax + By = C
    let scale = c / g;
    let x0 = x0 * scale;
    let y0 = y0 * scale;

    // General solution: x = x0 + k(b/g), y = y0 - k(a/g)
    let b_g = b / g;
    let a_g = a / g;

    let mut solutions = Vec::new();

    // Find bounds for k such that x > 0 and y > 0
    // x = x0 + k(b/g) > 0  =>  k > -x0 / (b/g)
    // y = y0 - k(a/g) > 0  =>  k < y0 / (a/g)
    let k_min = if -x0 % b_g == 0 { -x0 / b_g } else { -x0 / b_g - 1 };
    let k_max = if y0 % a_g == 0 { y0 / a_g } else { y0 / a_g };

    // Generate solutions for k in the range [k_min, k_max]
    for k in k_min..=k_max {
        let x = x0 + k * b_g;
        let y = y0 - k * a_g;
        if x > 0 && y > 0 {
            solutions.push((x, y));
        }
    }

    solutions
}

fn min_tokens_2(button_a: (u64, u64), button_b: (u64, u64), prize: (u64, u64)) -> Option<u64> {
    let mut min_coins = None;
    for (press_a, press_b) in find_positive_solutions(button_a.0 as i64, button_b.0 as i64, prize.0 as i64) {
        if press_a as u64 * button_a.1 + press_b as u64 * button_b.1 == prize.1
        {
            dbg!(press_a);
            min_coins = Some(min(
                min_coins.unwrap_or(u64::MAX),
                press_a as u64 * 3 + press_b as u64,
            ))
        }
    }
    min_coins
}

fn min_tokens(button_a: (u64, u64), button_b: (u64, u64), prize: (u64, u64)) -> Option<u64> {
    let mut min_coins = None;
    if let Some(((x0, x_step), _)) = solve(button_a.0 as i64, button_b.0 as i64, prize.0 as i64) {
        // press_a = x0 + k * x_step

        for k in -100000..100000 {
            let press_a = x0 + k * x_step;
            if press_a <= 0 {
                continue;
            }
            let remaining_c = prize.0 as i64 - button_a.0 as i64 * press_a;
            if remaining_c % button_b.0 as i64 != 0 {
                continue;
            }
            let y = remaining_c / button_b.0 as i64;
            if y > 0 {
                if press_a as u64 * button_a.1 + y as u64 * button_b.1 == prize.1
                {
                    dbg!(press_a);
                    min_coins = Some(min(
                        min_coins.unwrap_or(u64::MAX),
                        press_a as u64 * 3 + y as u64,
                    ))
                }
            }
        }
    }
    min_coins
}

fn doit(input: &str, plus: u64) -> u64 {
    input
        .lines()
        .chunks(4)
        .into_iter()
        .map(|mut lines| {
            let button_a = read_coords(lines.next().unwrap(), 0);
            let button_b = read_coords(lines.next().unwrap(), 0);
            let prize = read_coords(lines.next().unwrap(), plus);
            // dbg!(min_tokens_1(button_a, button_b, prize));
            // dbg!(min_tokens(button_a, button_b, prize));
            min_tokens(button_a, button_b, prize).unwrap_or(0)
        })
        .sum()
}

pub fn part1(input: &str) -> u64 {
    doit(input, 0)
}

pub fn part2(input: &str) -> u64 {
    doit(input, 10000000000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day13.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 31761);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
