use std::iter::repeat;

fn numeric(code: &str) -> u64 {
    code.chars()
        .flat_map(|c| c.to_digit(10))
        .reduce(|n, d| n * 10 + d)
        .unwrap() as u64
}

fn numpad_pos(c: char) -> (usize, usize) {
    match c {
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '0' => (3, 1),
        'A' => (3, 2),
        _ => unimplemented!("{:?}", c),
    }
}

fn arrowpad_pos(c: char) -> (usize, usize) {
    match c {
        '<' => (1,0),
        '>' => (1,2),
        '^' => (0,1),
        'v' => (1,1),
        'A' => (0,2),
        _ => unimplemented!(),
    }
}

fn type_code(code: &str) -> String {
    let mut seq = String::new();
    let mut state = 'A';
    for c in code.chars() {
        let (state_x, state_y) = numpad_pos(state);
        let (c_x, c_y) = numpad_pos(c);
        let updown = match state_x.cmp(&c_x) {
            std::cmp::Ordering::Equal => String::new(),
            std::cmp::Ordering::Greater => String::from_iter(repeat('^').take(state_x - c_x)),
            std::cmp::Ordering::Less => String::from_iter(repeat('v').take(c_x - state_x)),
        };
        let leftright = match state_y.cmp(&c_y) {
            std::cmp::Ordering::Equal => String::new(),
            std::cmp::Ordering::Greater => String::from_iter(repeat('<').take(state_y - c_y)),
            std::cmp::Ordering::Less => String::from_iter(repeat('>').take(c_y - state_y)),
        };
        seq += &match (leftright.chars().next(), updown.chars().next()) {
            (Some('<'), Some('^')) if state_x == 3 => updown + &leftright,
            _ => leftright + &updown,
        };
        seq += "A";
        state = c;
    }
    seq
}

fn type_arrows(arrows: &String) -> String {
    let mut seq = String::new();
    let mut state = 'A';
    for c in arrows.chars() {
        let (state_x, state_y) = arrowpad_pos(state);
        let (c_x, c_y) = arrowpad_pos(c);
        let updown = match state_x.cmp(&c_x) {
            std::cmp::Ordering::Equal => String::new(),
            std::cmp::Ordering::Greater => String::from_iter(repeat('^').take(state_x - c_x)),
            std::cmp::Ordering::Less => String::from_iter(repeat('v').take(c_x - state_x)),
        };
        let leftright = match state_y.cmp(&c_y) {
            std::cmp::Ordering::Equal => String::new(),
            std::cmp::Ordering::Greater => String::from_iter(repeat('<').take(state_y - c_y)),
            std::cmp::Ordering::Less => String::from_iter(repeat('>').take(c_y - state_y)),
        };
        seq += &match (leftright.chars().next(), updown.chars().next()) {
            (Some('<'), Some('v')) if state_x == 0 => updown + &leftright,
            _ => leftright + &updown,
        };
        seq += "A";
        state = c;
    }
    seq
}

fn shortest(code: &str) -> u64 {
    type_arrows(&type_arrows(dbg!(&type_code(code)))).len() as u64
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| numeric(line) * shortest(line))
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input.lines().count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day21.txt")
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1603498);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
