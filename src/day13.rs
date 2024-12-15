use itertools::Itertools;

fn read_coords(line: &str, plus: u128) -> (u128, u128) {
    let (_, coords) = line.split_once(": ").unwrap();
    coords
        .split(", ")
        .map(|coord| coord[2..].parse::<u128>().unwrap() + plus)
        .collect_tuple()
        .unwrap()
}

fn min_tokens_1(
    button_a: (u128, u128),
    button_b: (u128, u128),
    prize: (u128, u128),
) -> Option<u128> {
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
            min_coins = Some(std::cmp::min(
                min_coins.unwrap_or(u128::MAX),
                press_a * 3 + press_b,
            ))
        }
    }
    min_coins
}

fn solve(a: i128, b: i128, c: i128) -> Option<((i128, i128), (i128, i128))> {
    // Helper function: Extended Euclidean Algorithm
    fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
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

    Some(((x0, b_g), (y0, -a_g)))
}

fn bounds(x0: i128, x_step: i128, y0: i128, y_step: i128) -> (i128, i128) {
    // Find bounds for k such that x > 0 and y > 0
    // x = x0 + k * x_step > 0  =>  k > -x0 / x_step
    // y = y0 - k * y_step > 0  =>  k < y0 / y_step
    let k_min = if -x0 % x_step == 0 {
        -x0 / x_step
    } else {
        -x0 / x_step - 1
    };
    let k_max = if y0 % -y_step == 0 {
        y0 / -y_step
    } else {
        y0 / -y_step
    };
    if k_min > k_max {
        (k_max - 1, k_min + 1)
    } else {
        (k_min - 1, k_max + 1)
    }
}

fn all_bounds(equations_left: &[(i128, i128)], equations_right: &[(i128, i128)]) -> (i128, i128) {
    let lelen = equations_left.len();
    let relen = equations_right.len();
    (0..lelen)
        .flat_map(|i| (0..relen).map(move |j| (i, j)))
        .map(|(i, j)| {
            let eq1 = *equations_left.get(i).unwrap();
            let eq2 = *equations_right.get(j).unwrap();
            bounds(eq1.0, eq1.1, eq2.0, eq2.1)
        })
        .reduce(|(mx, mi), (l, r)| (std::cmp::max(mx, l), std::cmp::min(mi, r)))
        .unwrap()
}

fn find(
    min_k: i128,
    max_k: i128,
    x0: i128,
    x_step: i128,
    button_a: (u128, u128),
    button_b: (u128, u128),
    prize: (u128, u128),
) -> Option<u128> {
    let mut min_coins = None;
    dbg!(max_k - min_k);
    for k in min_k..=max_k {
        // dbg!(k);
        let press_a = x0 + k * x_step;
        // dbg!(press_a);
        if press_a < 0 {
            continue;
        }
        if press_a > prize.0 as i128 {
            continue;
        }
        let remaining_c = prize.0 as i128 - button_a.0 as i128 * press_a;
        if remaining_c % button_b.0 as i128 != 0 {
            continue;
        }
        let press_b = remaining_c / button_b.0 as i128;
        if press_b >= 0 && press_a as u128 * button_a.1 + press_b as u128 * button_b.1 == prize.1 {
            min_coins = Some(std::cmp::min(
                min_coins.unwrap_or(u128::MAX),
                press_a as u128 * 3 + press_b as u128,
            ));
            dbg!("found", press_a);
        }
    }
    min_coins
}

fn min_tokens(button_a: (u128, u128), button_b: (u128, u128), prize: (u128, u128)) -> Option<u128> {
    if let Some(((a0, a_step), (b0, b_step))) =
        solve(button_a.0 as i128, button_b.0 as i128, prize.0 as i128)
    {
        // press_a = a0 + k * a_step
        // press_b = b0 + k * b_step
        if let Some(((aa0, aa_step), (bb0, bb_step))) =
            solve(button_a.1 as i128, button_b.1 as i128, prize.1 as i128)
        {
            // press_a = aa0 + l * aa_step
            // press_b = bb0 + l * bb_step
            // press_a = a0 + k * a_step = aa0 + l * aa_step
            // k * a_step + l * -aa_step = aa0 - a0
            if let Some(((k0, k_step), (l0, l_step))) = solve(a_step, -aa_step, aa0 - a0) {
                // k = k0 + m * k_step
                // l = l0 + m * l_step
                // press_a = aa0 + (l0 + m * l_step) * aa_step
                // press_a = (aa0 + l0 * aa_step) + m * (l_step * aa_step)
                // press_b = bb0 + (l0 + m * l_step) * bb_step
                // press_b = (bb0 + l0 * bb_step) + m * (l_step * bb_step)
                let (min_m, max_m) = all_bounds(
                    &[
                        (a0 + k0 * a_step, k_step * a_step),
                        (aa0 + l0 * aa_step, l_step * aa_step),
                    ],
                    &[
                        (b0 + k0 * b_step, k_step * b_step),
                        (bb0 + l0 * bb_step, l_step * bb_step),
                    ],
                );

                // press_b = b0 + k * b_step = bb0 + l * bb_step
                // k * b_step + l * -bb_step = bb0 - b0
                if let Some(((kk0, kk_step), (ll0, ll_step))) = solve(b_step, -bb_step, bb0 - b0) {
                    // kk = kk0 + n * kk_step
                    // ll = ll0 + n * ll_step

                    // press_b = bb0 + (ll0 + n * ll_step) * bb_step
                    // press_b = b0 + (kk0 + n * kk_step) * b_step
                    // press_a = a0 + (kk0 + n * kk_step) * a_step
                    // press_a = aa0 + (ll0 + n * kk_step) * a_step
                    let (min_n, max_n) = all_bounds(
                        &[
                            (a0 + kk0 * a_step, kk_step * a_step),
                            (aa0 + ll0 * aa_step, ll_step * aa_step),
                        ],
                        &[
                            (b0 + kk0 * b_step, kk_step * b_step),
                            (bb0 + ll0 * bb_step, ll_step * bb_step),
                        ],
                    );

                    // press_a = a0 + (kk0 + n * kk_step) * a_step = aa0 + (l0 + m * l_step) * aa_step
                    // (a0 + kk0 * a_step) + n * (kk_step * a_step) = (aa0 + l0 * aa_step) + m * (l_step * aa_step)
                    // n * (kk_step * a_step) + m * -(l_step * aa_step) = aa0 + l0 * aa_step - (a0 + kk0 * a_step)
                    if let Some(((nn0, nn_step), (mm0, mm_step))) = solve(
                        kk_step * a_step,
                        -l_step * aa_step,
                        aa0 + l0 * aa_step - (a0 + kk0 * a_step),
                    ) {
                        // n = nn0 + o * nn_step
                        // m = mm0 + o * mm_step
                        let (min_o, max_o) = all_bounds(
                            &[
                                // press_a = a0 + (kk0 + (nn0 + o * nn_step) * kk_step) * a_step
                                (
                                    a0 + kk0 * a_step + nn0 * kk_step * a_step,
                                    nn_step * kk_step * a_step,
                                ),
                                // press_a = aa0 + (l0 + (nn0 + o * nn_step) * l_step) * aa_step
                                // press_a = aa0 + l * aa_step
                                // l = l0 + m * l_step
                                // press_a = aa0 + (l0 + m * l_step) * aa_step

                                /*(
                                    aa0 + l0 * aa_step + nn0 * l_step * aa_step,
                                    nn_step * l_step * aa_step,
                                ),*/
                                // press_a = aa0 + (l0 + m * l_step) * aa_step
                                // m = mm0 + o * mm_step
                                (
                                    aa0 + l0 * aa_step + mm0 * l_step * aa_step,
                                    mm_step * l_step * aa_step,
                                ),
                            ],
                            &[
                                // press_b = bb0 + (ll0 + (nn0 + o * nn_step) * ll_step) * bb_step
                                (
                                    bb0 + ll0 * bb_step + nn0 * ll_step * bb_step,
                                    nn_step * ll_step * bb_step,
                                ),
                                // press_b = b0 + (kk0 + (nn0 + o * nn_step) * kk_step) * b_step
                                (
                                    b0 + kk0 * b_step + nn0 * kk_step * b_step,
                                    nn_step * kk_step * b_step,
                                ),
                                // press_b = (bb0 + l0 * bb_step) + m * (l_step * bb_step)
                                // m = mm0 + o * mm_step
                                (
                                    bb0 + l0 * bb_step + mm0 * l_step * bb_step,
                                    mm_step * l_step * bb_step,
                                ),
                            ],
                        );
                        dbg!(max_m - min_m, max_n - min_n, max_o - min_o);
                        //let find_m = find(min_m, max_m, aa0 + l0 * aa_step, l_step * aa_step, button_a, button_b, prize);
                        let find_o = find(
                            min_o,
                            max_o,
                            aa0 + l0 * aa_step + mm0 * l_step * aa_step,
                            mm_step * l_step * aa_step,
                            button_a,
                            button_b,
                            prize,
                        );
                        // if find_m != find_o {
                        //     dbg!(find_m, find_o);
                        //     dbg!(min_o, max_o);
                        // }
                        return find_o;
                    }
                }
            }
        }
    }
    None
}

fn doit(input: &str, plus: u128) -> u128 {
    input
        .lines()
        .chunks(4)
        .into_iter()
        .enumerate()
        // .skip(289)
        // .take(1)
        .map(|(i, mut lines)| {
            dbg!(i);
            let button_a = read_coords(lines.next().unwrap(), 0);
            let button_b = read_coords(lines.next().unwrap(), 0);
            let prize = read_coords(lines.next().unwrap(), plus);
            // dbg!(min_tokens_1(button_a, button_b, prize));
            // dbg!(min_tokens(button_a, button_b, prize));
            dbg!(min_tokens(button_a, button_b, prize).unwrap_or(0))
        })
        .sum()
}

pub fn part1(input: &str) -> u128 {
    doit(input, 0)
}

pub fn part2(input: &str) -> u128 {
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

    #[ignore = "very slow"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 90798500745591);
    }
}
