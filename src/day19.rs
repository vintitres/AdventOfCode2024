use std::collections::HashMap;

fn possiblilities(design: &str, towels: &[&str], start: usize, seen: &mut HashMap<usize, u64>) -> u64 {
    // dbg!(design, towels, start);
    if start == design.len() {
        return 1;
    }
    if seen.contains_key(&start) {
        return *seen.get(&start).unwrap();
    }
    let mut count = 0;
    for towel in towels {
        if design.split_at(start).1.starts_with(towel) {
            count += possiblilities(design, towels, start + towel.len(), seen);
        }
    }
    seen.insert(start, count);
    return count;
}

fn read_towels(input: &str) -> Vec<&str> {
    input.lines().take(1).next().unwrap().split(", ").collect()
}

pub fn part1(input: &str) -> usize {
    let towels = read_towels(input);
    input.lines().skip(2).filter(|design| possiblilities(design, &towels, 0, &mut HashMap::new()) > 0).count()
}

pub fn part2(input: &str) -> u64 {
    let towels = read_towels(input);
    input.lines().skip(2).map(|design| possiblilities(design, &towels, 0, &mut HashMap::new())).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day19.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 308);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 662726441391898);
    }
}
