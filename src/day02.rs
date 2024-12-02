pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let levels: Vec<u64> = line
                .split_whitespace()
                .map(|level| level.parse::<u64>().unwrap())
                .collect();
            check(&levels).is_none()
        })
        .count()
}

fn check(levels: &[u64]) -> Option<usize> {
    let mut last: Option<u64> = None;
    let mut dec = None;
    for (i, &level) in levels.iter().enumerate() {
        if last.is_some() {
            let last = last.unwrap();
            if dec.is_none() {
                dec = Some(last > level);
                if last == level {
                    return Some(i);
                }
            }
            let dec = dec.unwrap();
            if dec && last > level {
                if last - level > 3 {
                    return Some(i);
                }
            } else if !dec && last < level {
                if level - last > 3 {
                    return Some(i);
                }
            } else {
                return Some(i);
            }
        }
        last = Some(level);
    }
    None
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let levels: Vec<u64> = line
                .split_whitespace()
                .map(|level| level.parse::<u64>().unwrap())
                .collect();
            for i in 0..levels.len() {
                let mut levels2 = levels.clone();
                levels2.remove(i);
                if check(&levels2).is_none() {
                    return true;
                }
            }
            false
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 354);
    }
}
