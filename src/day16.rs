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
                line.chars().enumerate().for_each(|(j, c)| match c {
                    'S' => {
                        start = Some(Pos {
                            x: i as isize,
                            y: j as isize,
                        });
                    }
                    'E' => {
                        end = Some(Pos {
                            x: i as isize,
                            y: j as isize,
                        });
                    }
                    _ => (),
                });
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
}

fn doit(input: &str) -> (u64, HashSet<Pos>) {
    let (world, start, end) = World::read(input);
    let mut pq = BTreeSet::new();
    pq.insert((0_u64, Dir::Right, start, vec![]));
    let mut best_score = HashMap::<(Pos, Dir), u64>::new();
    let mut best_end_paths_parts = HashSet::<Pos>::new();
    let mut best_end_score = None;
    while !pq.is_empty() {
        let (score, dir, pos, mut path) = pq.pop_first().unwrap();
        if let Some(best_end_score) = best_end_score {
            if score > best_end_score {
                break;
            }
        }
        if pos == end {
            best_end_score = Some(score);
            dbg!(&path);
            best_end_paths_parts.extend(path.iter());
        }
        if !world.open(&pos).unwrap() {
            continue;
        }
        let posdir = (pos, dir);
        if score >= *best_score.get(&posdir).unwrap_or(&u64::MAX) {
            continue;
        }
        best_score.insert(posdir, score);
        for d in Dir::all() {
            if d != dir {
                pq.insert((score + 1000, d, pos, path.clone()));
            }
        }
        path.push(pos);
        pq.insert((score + 1, dir, pos.next(&dir), path));
    }
    (best_end_score.unwrap(), best_end_paths_parts)
}

pub fn part1(input: &str) -> u64 {
    doit(input).0
}

pub fn part2(input: &str) -> usize {
    doit(input).1.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day16.txt")
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1603498);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
