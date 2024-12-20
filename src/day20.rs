use std::{
    collections::{BTreeSet, HashMap},
    u64,
};

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
    fn new_from_usize(x: usize, y: usize) -> Pos {
        Pos {
            x: x as isize,
            y: y as isize,
        }
    }
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

struct World {
    map: Vec<Vec<char>>,
}

impl World {
    fn read(input: &str) -> (World, Pos, Pos) {
        let mut start = None;
        let mut end = None;
        let map = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                if let Some(j) = line.find('S') {
                    start = Some(Pos::new_from_usize(i, j));
                }
                if let Some(j) = line.find('E') {
                    end = Some(Pos::new_from_usize(i, j));
                }
                line.chars().collect_vec()
            })
            .collect_vec();
        (World { map }, start.unwrap(), end.unwrap())
    }
    fn open(&self, pos: &Pos) -> Option<bool> {
        if let Ok(i) = usize::try_from(pos.x) {
            if let Ok(j) = usize::try_from(pos.y) {
                if let Some(map_row) = self.map.get(i) {
                    return Some(*map_row.get(j).unwrap() != '#');
                }
            }
        }
        None
    }
    fn all_cheats(&self) -> Vec<Pos> {
        let height = self.map.len() - 2;
        let width = self.map[0].len() - 2;
        self.map
            .iter()
            .enumerate()
            .skip(1)
            .take(height)
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .skip(1)
                    .take(width)
                    .flat_map(move |(j, c)| {
                        if *c == '#' {
                            Some(Pos::new_from_usize(i, j))
                        } else {
                            None
                        }
                    })
            })
            .collect()
    }
}

fn doit(world: &World, start: Pos, end: Pos, limit: u64, cheat: Option<Pos>) -> Option<u64> {
    let mut pq = BTreeSet::new();
    pq.insert((0_u64, start));
    let mut best_score = HashMap::<Pos, u64>::new();
    while !pq.is_empty() {
        let (score, pos) = pq.pop_first().unwrap();
        if score > limit {
            return None;
        }
        if pos == end {
            return Some(score);
        }
        if Some(pos) != cheat && !world.open(&pos).unwrap() {
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
        for dir in Dir::all() {
            pq.insert((score + 1, pos.next(&dir)));
        }
    }
    None
}

pub fn part1(input: &str) -> usize {
    let (world, start, end) = World::read(input);
    let nocheat = doit(&world, start, end, u64::MAX, None).unwrap();
    let cheats = world.all_cheats();
    cheats
        .into_iter()
        .rev()
        .filter(|cheat| doit(&world, start, end, nocheat - 100, Some(*dbg!(cheat))).is_some())
        .count()
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day20.txt")
    }

    #[ignore = "slow"]
    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1450);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
