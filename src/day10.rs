use std::collections::HashSet;

use itertools::Itertools;

type Pos = (isize, isize);

struct World {
    map: Vec<Vec<u32>>,
}

impl World {
    fn read(input: &str) -> World {
        let map = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
            .collect_vec();
        World { map }
    }
    fn get(&self, pos: Pos) -> Option<u32> {
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

    fn trailhead(&self, pos: (usize, usize)) -> Vec<Pos> {
        let pos = (pos.0 as isize, pos.1 as isize);
        if self.get(pos) == Some(0) {
            self.trails(pos)
        } else {
            vec![]
        }
    }

    fn trailhead_score(&self, pos: (usize, usize)) -> usize {
        HashSet::<Pos>::from_iter(self.trailhead(pos)).len()
    }

    fn trails(&self, pos: Pos) -> Vec<Pos> {
        if let Some(h) = self.get(pos) {
            if h == 9 {
                vec![pos]
            } else {
                [(0, 1), (1, 0), (0, -1), (-1, 0)]
                    .iter()
                    .flat_map(|(shiftx, shifty)| {
                        let npos = (pos.0 + shiftx, pos.1 + shifty);
                        if self.get(npos) == Some(h + 1) {
                            self.trails(npos)
                        } else {
                            vec![]
                        }
                    })
                    .collect()
            }
        } else {
            vec![]
        }
    }
}

pub fn part1(input: &str) -> usize {
    let w = World::read(input);
    (0..w.height())
        .flat_map(|i| (0..w.width()).map(move |j| (i, j)))
        .map(|(i, j)| w.trailhead_score((i, j)))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let w = World::read(input);
    (0..w.height())
        .flat_map(|i| (0..w.width()).map(move |j| (i, j)))
        .map(|(i, j)| w.trailhead((i, j)).len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day10.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 719);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1530);
    }
}
