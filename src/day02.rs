pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let levels: Vec<u64> = line
                .split_whitespace()
                .map(|level| level.parse::<u64>().unwrap())
                .collect();
            check(&levels)
        })
        .count()
}

fn check(levels: &Vec<u64>) -> bool {
    let mut last: Option<u64> = None;
    let mut dec = None;
    for &level in levels {
        if last.is_some() {
            let last = last.unwrap();
            if dec.is_none() {
                dec = Some(last > level);
                if last == level {
                    return false;
                }
            }
            let dec = dec.unwrap();
            if dec && last > level {
                if last - level > 3 {
                    return false;
                }
            } else if !dec && last < level {
                if level - last > 3 {
                    return false;
                }
            } else {
                return false;
            }
        }

        last = Some(level);
    }
    true
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let levels: Vec<u64> = line
                .split_whitespace()
                .map(|level| level.parse::<u64>().unwrap())
                .collect();
            check(&levels)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day2.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 287);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
