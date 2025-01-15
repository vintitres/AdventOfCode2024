use itertools::{Either, Itertools};

fn parse(s: &str, cc: char) -> Vec<usize> {
    let mut ret = vec![0, 0, 0, 0, 0];
    for (i, line) in s.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == cc {
                ret[j] = i;
            }
        }
    }
    ret
}

#[derive(Debug)]
struct Key {
    pins: Vec<usize>,
}

impl Key {
    fn parse(s: &str) -> Key {
        Key {
            pins: parse(s, '.'),
        }
    }
}

#[derive(Debug)]
struct Lock {
    holes: Vec<usize>,
}
impl Lock {
    fn parse(s: &str) -> Lock {
        Lock {
            holes: parse(s, '#'),
        }
    }

    fn fit(&self, key: &Key) -> bool {
        for (i, hole) in self.holes.iter().enumerate() {
            if hole > key.pins.get(i).unwrap() {
                return false;
            }
        }
        true
    }
}

enum Schema {
    Key(Key),
    Lock(Lock),
}

impl Schema {
    fn parse(s: &str) -> Schema {
        if s.starts_with("#") {
            Schema::Lock(Lock::parse(s))
        } else {
            Schema::Key(Key::parse(s))
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let (keys, locks): (Vec<Key>, Vec<Lock>) = input
        .trim()
        .split("\n\n")
        .map(Schema::parse)
        .partition_map(|s| match s {
            Schema::Key(key) => Either::Left(key),
            Schema::Lock(lock) => Either::Right(lock),
        });
    let mut matches = 0;
    for lock in &locks {
        for key in &keys {
            if lock.fit(key) {
                matches += 1;
            }
        }
    }
    matches
}

pub fn part2(input: &str) -> u64 {
    input.lines().count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day25.txt")
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 3439); // ?
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
