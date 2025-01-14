use std::{collections::HashSet, iter::repeat, u64};

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
        '<' => (1, 0),
        '>' => (1, 2),
        '^' => (0, 1),
        'v' => (1, 1),
        'A' => (0, 2),
        _ => unimplemented!(),
    }
}

fn type_code(code: &str) -> HashSet<String> {
    let mut res = HashSet::new();
    res.insert(String::new());
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
        let leftup_ok = match (leftright.chars().next(), updown.chars().next()) {
            (Some('<'), Some('^')) if state_x == 3 => false,
            _ => true,
        };
        let mut newres = HashSet::new();
        for s in &res {
            if leftup_ok {
                newres.insert(s.clone() + &leftright + &updown + "A");
            }
            newres.insert(s.clone() + &updown + &leftright + "A");
        }
        res = newres;
        state = c;
    }
    res
}

fn is_ok(pos: &Pos, blocked: &Pos, movements: &String) -> bool {
    p
}

fn type_arrows(arrows: &String) -> HashSet<String> {
    // let mut seq = String::new();
    let mut ret = HashSet::new();
    ret.insert(String::new());
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
        // seq += &match (leftright.chars().next(), updown.chars().next()) {
        //     (Some('<'), Some('v')) if state_x == 0 => updown + &leftright,
        //     _ => leftright + &updown,
        // };
        // seq += "A";
        // seq += &match (leftright.chars().next(), updown.chars().next()) {
        let leftup_ok = !(leftright.starts_with("<") && updown.starts_with("v") && state_x == 0);
        let mut newret = HashSet::new();
        for s in ret {
            if leftup_ok {
                newret.insert(s.clone() + &leftright + &updown + "A");
            }
            newret.insert(s.clone() + &updown + &leftright + "A");
        }
        ret = newret;
        state = c;
    }
    // seq
    ret
}

fn shortest(code: &str) -> u64 {
    let mut min_len = usize::MAX;
    for arrows1 in type_code(code) {
        for arrows2 in type_arrows(&arrows1) {
            dbg!(&arrows2, arrows2.len());
            for arrows3 in type_arrows(&arrows2) {
                min_len = std::cmp::min(min_len, arrows3.len());
            }
        }
    }
    min_len as u64
    //type_arrows(&type_arrows(dbg!(&type_code(code)))).len() as u64
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .skip(4) // del
        .map(|line| dbg!(numeric(line)) * shortest(line))
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
        assert_eq!(part1(input()), 171460); // too high
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
