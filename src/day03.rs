use std::str::Chars;

pub fn read_num(chars: &mut Chars<'_>, last_index: &mut usize, expected_end: char) -> Option<u64> {
    let mut n: u64 = 0;
    for c in chars.by_ref() {
        *last_index += 1;
        if c.is_ascii_digit() {
            n *= 10;
            n += c.to_digit(10).unwrap() as u64;
        } else if c == expected_end {
            return Some(n);
        } else {
            return None;
        }
    }
    None
}

pub fn part1(input: &str) -> u64 {
    let mut chars = input.chars();
    let mut last_index = 0;
    let indices: Vec<usize> = input
        .match_indices("mul(")
        .map(|(index, _)| index)
        .collect();
    indices
        .into_iter()
        .map(|index| {
            while last_index < index + 4 {
                last_index += 1;
                chars.next();
            }
            if let Some(n1) = read_num(&mut chars, &mut last_index, ',') {
                if let Some(n2) = read_num(&mut chars, &mut last_index, ')') {
                    return n1 * n2;
                }
            }
            0
        })
        .sum()
}

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
enum Operation {
    Mul,
    Do,
    Dont,
}

pub fn part2(input: &str) -> u64 {
    let mut chars = input.chars();
    let mut last_index = 0;
    let mut indices: Vec<(usize, Operation)> = input
        .match_indices("mul(")
        .map(|(index, _)| (index, Operation::Mul))
        .chain(
            input
                .match_indices("do()")
                .map(|(index, _)| (index, Operation::Do)),
        )
        .chain(
            input
                .match_indices("don't()")
                .map(|(index, _)| (index, Operation::Dont)),
        )
        .collect();
    indices.sort();
    let mut doing = true;
    indices
        .into_iter()
        .map(|(index, op)| {
            match op {
                Operation::Do => {
                    doing = true;
                }
                Operation::Dont => {
                    doing = false;
                }
                Operation::Mul => {
                    if doing {
                        while last_index < index + 4 {
                            last_index += 1;
                            chars.next();
                        }
                        if let Some(n1) = read_num(&mut chars, &mut last_index, ',') {
                            if let Some(n2) = read_num(&mut chars, &mut last_index, ')') {
                                return n1 * n2;
                            }
                        }
                    }
                }
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day3.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 173517243);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 100450138);
    }
}
