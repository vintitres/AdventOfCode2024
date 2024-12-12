use std::collections::{BTreeSet, HashSet};

use itertools::{Either, Itertools};

type Pos = (isize, isize);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn shift(&self) -> Pos {
        match self {
            Dir::Down => (1, 0),
            Dir::Up => (-1, 0),
            Dir::Right => (0, 1),
            Dir::Left => (0, -1),
        }
    }

    fn all() -> Vec<Dir> {
        vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right]
    }
}

struct World {
    map: Vec<Vec<char>>,
    seen: HashSet<Pos>,
}

impl World {
    fn read(input: &str) -> World {
        let map = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        World {
            map,
            seen: HashSet::new(),
        }
    }
    fn get(&self, pos: Pos) -> Option<char> {
        if let Ok(i) = usize::try_from(pos.0) {
            if let Ok(j) = usize::try_from(pos.1) {
                if let Some(map_row) = self.map.get(i) {
                    return map_row.get(j).copied();
                }
            }
        }
        None
    }

    fn width(&self) -> usize {
        self.map.len()
    }

    fn height(&self) -> usize {
        self.map[0].len()
    }

    fn fence_detail(&mut self, i: usize, j: usize) -> (u64, BTreeSet<(Pos, Dir)>) {
        let pos = (i as isize, j as isize);
        if self.seen.contains(&pos) {
            return (0, BTreeSet::new());
        }
        self.visit(pos, self.get(pos).unwrap())
    }

    fn fence_price(&mut self, i: usize, j: usize) -> u64 {
        let (a, p) = self.fence_detail(i, j);
        a * p.len() as u64
    }

    fn fence_price_discounted(&mut self, i: usize, j: usize) -> u64 {
        let (a, p) = self.fence_detail(i, j);
        let mut sides = 0;
        let (up, p): (Vec<Pos>, Vec<(Pos, Dir)>) = p.iter().partition_map(|&(pos, dir)| {
            if dir == Dir::Up {
                Either::Left(pos)
            } else {
                Either::Right((pos, dir))
            }
        });
        let (down, p): (Vec<Pos>, Vec<(Pos, Dir)>) = p.iter().partition_map(|&(pos, dir)| {
            if dir == Dir::Down {
                Either::Left(pos)
            } else {
                Either::Right((pos, dir))
            }
        });
        let (right, left): (Vec<Pos>, Vec<Pos>) = p.iter().partition_map(|&(pos, dir)| {
            if dir == Dir::Right {
                Either::Left(pos)
            } else {
                Either::Right(pos)
            }
        });
        let mut last_pos = (-2, -2);
        let mut count_connected = |pos: &Pos, shift: &Pos| {
            let (x, y) = last_pos;
            last_pos = *pos;
            if x + shift.0 != pos.0 || y + shift.1 != pos.1 {
                1
            } else {
                0
            }
        };
        let mut updown = |pos: &Pos| count_connected(pos, &Dir::Right.shift());
        sides += up.iter().map(&mut updown).sum::<u64>();
        sides += down.iter().map(&mut updown).sum::<u64>();
        let mut leftright = |pos: &Pos| count_connected(pos, &Dir::Down.shift());
        let flipped_cmp = |l: &&Pos, r: &&Pos| (l.1, l.0).cmp(&(r.1, r.0));
        sides += right
            .iter()
            .sorted_by(flipped_cmp)
            .map(&mut leftright)
            .sum::<u64>();
        sides += left
            .iter()
            .sorted_by(flipped_cmp)
            .map(&mut leftright)
            .sum::<u64>();
        a * sides
    }

    fn visit(&mut self, pos: Pos, c: char) -> (u64, BTreeSet<(Pos, Dir)>) {
        if self.seen.contains(&pos) {
            return (0, BTreeSet::new());
        }
        self.seen.insert(pos);
        Dir::all()
            .iter()
            .map(|dir| {
                let (x, y) = dir.shift();
                let next_pos = (pos.0 + x, pos.1 + y);
                if self.get(next_pos) == Some(c) {
                    self.visit(next_pos, c)
                } else {
                    (0, BTreeSet::from([(pos, *dir)]))
                }
            })
            .fold((1, BTreeSet::new()), |(l1, mut r1), (l2, r2)| {
                r1.extend(r2);
                (l1 + l2, r1)
            })
    }
}

pub fn part1(input: &str) -> u64 {
    let mut w = World::read(input);
    let h = w.height();
    let width = w.width();
    (0..h)
        .flat_map(|i| (0..width).map(move |j| (i, j)))
        .map(|(i, j)| w.fence_price(i, j))
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let mut w = World::read(input);
    let h = w.height();
    let width = w.width();
    (0..h)
        .flat_map(|i| (0..width).map(move |j| (i, j)))
        .map(|(i, j)| w.fence_price_discounted(i, j))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day12.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1359028);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 839780);
    }
}
