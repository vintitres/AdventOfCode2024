pub fn part1(input: &str) -> u64 {
    let locs: Vec<Vec<u64>> = input
        .lines()
        .map(|l| {
            l.split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect()
        }).collect();
    let mut left: Vec<u64> = locs.iter().map(|l| l[0]).collect();
    let mut right: Vec<u64> = locs.iter().map(|l| l[1]).collect();
    left.sort();
    right.sort();
    left.iter().zip(right).map(|(l, r)| if *l > r { *l - r } else { r - *l }).sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            l.replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
        })
        .map(|l| {
            let mut digits = l.chars().flat_map(|c| c.to_digit(10));
            let f = digits.next().unwrap();
            let l = digits.last().unwrap_or(f);
            f * 10 + l
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day1.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1603498);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 55686);
    }
}
