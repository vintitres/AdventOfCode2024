use std::collections::HashSet;

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => unimplemented!("unknown direction"),
        }
    }
    fn turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn step(&self, pos: Pos) -> Pos {
        match self {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0, pos.1 + 1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1 - 1),
        }
    }
}

type Pos = (isize, isize);

struct World {
    map: Vec<Vec<char>>,
}

impl World {
    fn read(input: &str) -> (World, Pos) {
        let mut pos = (-1, -1);
        let map = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                if let Some(j) = line.find(['<', '^', '>', 'v']) {
                    pos = (i as isize, j as isize)
                }
                line.chars().collect_vec()
            })
            .collect_vec();
        (World { map }, pos)
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

    fn put_wall(&self, at: (usize, usize)) -> Option<World> {
        match self.get((at.0 as isize, at.1 as isize)) {
            Some('.') => {
                let mut map = self.map.clone();
                map[at.0][at.1] = '#';
                Some(World { map })
            }
            _ => None,
        }
    }
}

fn walk(world: &World, start_pos: Pos, start_dir: Direction) -> (usize, bool) {
    let mut seen = HashSet::new();
    let mut pos = start_pos;
    let mut dir = start_dir;
    let mut cycle = false;
    loop {
        let state = (pos, dir);
        if seen.contains(&state) {
            cycle = true;
            break;
        }
        seen.insert(state);
        let next_pos = dir.step(pos);
        match world.get(next_pos) {
            None => break,
            Some('#') => dir = dir.turn(),
            _ => {
                pos = next_pos;
            }
        }
    }
    (
        HashSet::<Pos>::from_iter(seen.iter().map(|(pos, _)| *pos)).len(),
        cycle,
    )
}

pub fn part1(input: &str) -> usize {
    let (world, pos) = World::read(input);
    let dir = Direction::from_char(world.get(pos).unwrap());
    walk(&world, pos, dir).0
}

pub fn part2(input: &str) -> usize {
    let (world, pos) = World::read(input);
    let dir = Direction::from_char(world.get(pos).unwrap());
    (0..world.height())
        .into_par_iter()
        .map(|x| {
            (0..world.width())
                .into_par_iter()
                .filter(|y| {
                    if let Some(new_world) = world.put_wall((x, *y)) {
                        walk(&new_world, pos, dir).1
                    } else {
                        false
                    }
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day6.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 5404);
    }

    #[ignore = "slow"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1984);
    }
}
