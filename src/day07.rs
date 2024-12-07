use std::collections::HashSet;

use itertools::Itertools;

fn plusmul(sum: i64, nums: &[i64], concat: bool) -> bool {
    let mut nums = nums.iter();
    let mut posibilities = HashSet::new();
    posibilities.insert(*nums.next().unwrap());
    for num in nums {
        let mut new_posibilities = HashSet::new();
        for posibility in &posibilities {
            let posi1 = *posibility * num;
            if posi1 <= sum {
                new_posibilities.insert(posi1);
            }
            let posi2 = *posibility + num;
            if posi2 <= sum {
                new_posibilities.insert(posi2);
            }
            if concat {
                let ten = 10_i64.pow((*num as f64).log10() as u32 + 1); // ugh!
                let posi3 = *posibility * ten + num;
                //dbg!(posibility, num, posi3, ten);
                if posi3 <= sum {
                    new_posibilities.insert(posi3);
                }
            }
        }
        posibilities = new_posibilities;
    }
    return posibilities.contains(&sum);
}

fn read(line: &str) -> (i64, Vec<i64>) {
    let (sum, nums) = line.split(": ").collect_tuple().unwrap();
    (
        sum.parse().unwrap(),
        nums.split(' ').map(|num| num.parse().unwrap()).collect(),
    )
}

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(read)
        .filter(|(sum, nums)| plusmul(*sum, nums, false))
        .map(|(sum, _)| sum)
        .sum()
}

pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(read)
        .filter(|(sum, nums)| plusmul(*sum, nums, true))
        .map(|(sum, _)| sum)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day7.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1260333054159);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 162042343638683);
    }
}
