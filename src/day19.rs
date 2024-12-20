use std::collections::HashMap;

fn is_possibe(design: &str, towels: &[&str], start: usize, seen: &mut HashMap<usize, bool>) -> bool {
    // dbg!(design, towels, start);
    if start == design.len() {
        return true;
    }
    if seen.contains_key(&start) {
        return *seen.get(&start).unwrap();
    }
    for towel in towels {
        if design.split_at(start).1.starts_with(towel) {
            if is_possibe(design, towels, start + towel.len(), seen) {
                seen.insert(start, true);
                return true;
            }
        }
    }
    seen.insert(start, false);
    false
}

fn read_towels(input: &str) -> Vec<&str> {
    input.lines().take(1).next().unwrap().split(", ").collect()
}

pub fn part1(input: &str) -> usize {
    let towels = read_towels(input);
    input.lines().skip(2).filter(|design| is_possibe(design, &towels, 0, &mut HashMap::<usize, bool>::new())).count()
}

pub fn part2(input: &str) -> u64 {
    input.lines().count() as u64
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

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
