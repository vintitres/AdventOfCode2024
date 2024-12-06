use std::collections::HashSet;

use itertools::Itertools;

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
}

pub fn part1(input: &str) -> usize {
    let (world, mut pos) = World::read(input);
    dbg!(pos);
    let mut dir = Direction::from_char(world.get(pos).unwrap());
    let mut seen = HashSet::new();
    seen.insert(pos);
    loop {
        let next_pos = dir.step(pos);
        match world.get(next_pos) {
            None => break,
            Some('#') => dir = dir.turn(),
            _ => {
                seen.insert(next_pos);
                pos = next_pos;
            }
        }
    }
    seen.len()
}

pub fn part2(input: &str) -> u64 {
    input.lines().count() as u64
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

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
