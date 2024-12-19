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
    const ALL: [Dir; 4] = [Dir::Up, Dir::Right, Dir::Down, Dir::Left];
}

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Copy, Debug, Default)]
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
    fn as_tuple(&self) -> (isize, isize) {
        (self.x, self.y)
    }
}

fn doit(corrupted: &[Pos], take: usize) -> Option<u64> {
    let corrupted = HashSet::<Pos>::from_iter(corrupted.iter().take(take).cloned());
    let mut pq = BTreeSet::new();
    let end = Pos { x: SIZE, y: SIZE };
    pq.insert((0_u64, Pos { x: 0, y: 0 }));
    let mut best_score = HashMap::<Pos, u64>::new();
    let mut best_end_score = None;
    while !pq.is_empty() {
        let (score, pos) = pq.pop_first().unwrap();
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
        best_score.insert(pos, score);
        for d in Dir::ALL {
            pq.insert((score + 1, pos.next(&d)));
        }
    }
    best_end_score
}

fn read_corrupted(input: &str) -> Vec<Pos> {
    input
        .lines()
        .map(|l| {
            let (x, y) = l
                .split(',')
                .map(|n| n.parse::<isize>().unwrap())
                .collect_tuple()
                .unwrap();
            Pos { x, y }
        })
        .collect()
}

const SIZE: isize = 70;
const TAKE: usize = 1024;

pub fn part1(input: &str) -> u64 {
    doit(&read_corrupted(input), TAKE).unwrap()
}

pub fn part2(input: &str) -> (isize, isize) {
    let corrupted = read_corrupted(input);
    let mut left = 0;
    let mut right = corrupted.len();

    while left < right {
        let mid = (left + right + 1) / 2;
        if doit(&corrupted, mid).is_none() {
            right = mid - 1;
        } else {
            left = mid;
        }
    }
    corrupted.get(left).unwrap().as_tuple()
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), (60, 46));
    }
}
