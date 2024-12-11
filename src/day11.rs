use std::collections::HashMap;

type Mem = HashMap<(u64, usize), usize>;

fn half(n: u64) -> Option<(u64, u64)> {
    let n = n.to_string();
    if n.len() % 2 == 0 {
        let (l, r) = n.split_at(n.len() / 2);
        Some((l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap()))
    } else {
        None
    }
}

fn stones(num: u64, steps: usize, mem: &mut Mem) -> usize {
    if steps == 0 {
        return 1;
    }
    let key = (num, steps);
    if let Some(s) = mem.get(&key) {
        *s
    } else {
        let s = if num == 0 {
            stones(1, steps - 1, mem)
        } else if let Some((l, r)) = half(num) {
            stones(l, steps - 1, mem) + stones(r, steps - 1, mem)
        } else {
            stones(num * 2024, steps - 1, mem)
        };
        mem.insert(key, s);
        s
    }
}

fn doit(input: &str, steps: usize) -> usize {
    let mut mem = Mem::new();
    input
        .trim()
        .split(' ')
        .map(|word| word.parse::<u64>().unwrap())
        .map(|num| stones(num, steps, &mut mem))
        .sum()
}

pub fn part1(input: &str) -> usize {
    doit(input, 25)
}

pub fn part2(input: &str) -> usize {
    doit(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day11.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 217443);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 257246536026785);
    }
}
