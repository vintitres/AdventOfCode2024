use std::collections::HashSet;

use itertools::Itertools;

type Pos = (isize, isize);

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

    fn fence_price(&mut self, i: usize, j: usize) -> u64 {
        let pos = (i as isize, j as isize);
        if self.seen.contains(&pos) {
            return 0;
        }
        let (a, p) = self.visit(pos, self.get(pos).unwrap());
        a * p
    }

    fn visit(&mut self, pos: Pos, c: char) -> (u64, u64) {
        if self.seen.contains(&pos) {
            return (0, 0);
        }
        self.seen.insert(pos);
        [(0, 1), (1, 0), (-1, 0), (0, -1)]
            .iter()
            .map(|(x, y)| {
                let next_pos = (pos.0 + x, pos.1 + y);
                if self.get(next_pos) == Some(c) {
                    self.visit(next_pos, c)
                } else {
                    (0, 1)
                }
            })
            .fold((1, 0), |(l1, r1), (l2, r2)| (l1 + l2, r1 + r2))
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
    input.lines().count() as u64
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

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
