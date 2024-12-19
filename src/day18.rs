use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Left,
    Right,
    Down,
}

impl Dir {
    fn all() -> Vec<Dir> {
        vec![Dir::Up, Dir::Right, Dir::Down, Dir::Left]
    }
}

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Copy, Debug)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn next(&self, dir: &Dir) -> Pos {
        let (x, y) = match dir {
            Dir::Down => (self.x + 1, self.y),
            Dir::Up => (self.x - 1, self.y),
            Dir::Left => (self.x, self.y - 1),
            Dir::Right => (self.x, self.y + 1),
        };
        Pos { x, y }
    }
}

const SIZE: isize = 70;

fn doit(input: &str) -> u64 {
    let corrupted = HashSet::<Pos>::from_iter(input.lines().take(1024).map(|l| {
        let (x, y) = l
            .split(',')
            .map(|n| n.parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();
        Pos { x, y }
    }));
    let mut pq = BTreeSet::new();
    let end = Pos { x: SIZE, y: SIZE };
    pq.insert((0_u64, Pos { x: 0, y: 0 }));
    let mut best_score = HashMap::<Pos, u64>::new();
    let mut best_end_score = None;
    while !pq.is_empty() {
        let (score, pos) = pq.pop_first().unwrap();
        dbg!(score, pos);
        if pos == end {
            best_end_score = Some(score);
            break;
        }
        if pos.x < 0 || pos.y < 0 || pos.x > SIZE || pos.y > SIZE {
            continue;
        }
        if corrupted.contains(&pos) {
            continue;
        }
        match score.cmp(best_score.get(&pos).unwrap_or(&u64::MAX)) {
            std::cmp::Ordering::Less => (),
            std::cmp::Ordering::Equal => {
                continue;
            }
            std::cmp::Ordering::Greater => {
                continue;
            }
        }
        dbg!(score, pos);
        best_score.insert(pos, score);
        for d in Dir::all() {
            pq.insert((score + 1, pos.next(&d)));
        }
    }
    best_end_score.unwrap()
}
pub fn part1(input: &str) -> u64 {
    doit(input)
}

pub fn part2(input: &str) -> u64 {
    input.lines().count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day18.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 226);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
