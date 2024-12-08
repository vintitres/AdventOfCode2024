use std::collections::{HashMap, HashSet};

use itertools::Itertools;

struct Antinodes {
    current: Pos,
    stepx: isize,
    stepy: isize,
}

impl Iterator for Antinodes {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current.0 += self.stepx;
        self.current.1 += self.stepy;

        Some(current)
    }
}

impl Antinodes {
    fn for_antennas(a1: &Pos, a2: &Pos) -> (Antinodes, Antinodes) {
        assert_ne!(a1, a2);
        let diffx = a1.0 - a2.0;
        let diffy = a1.1 - a2.1;
        (
            Antinodes {
                current: *a1,
                stepx: diffx,
                stepy: diffy,
            },
            Antinodes {
                current: *a2,
                stepx: -diffx,
                stepy: -diffy,
            },
        )
    }
}

struct World {
    map: Vec<Vec<char>>,
}

impl World {
    fn read(input: &str) -> World {
        let map = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        World { map }
    }

    fn find_antennas(&self) -> HashMap<char, HashSet<Pos>> {
        let mut antennas = HashMap::new();
        self.map.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, &c)| {
                if c != '.' {
                    antennas
                        .entry(c)
                        .or_insert(HashSet::new())
                        .insert((i as isize, j as isize));
                }
            })
        });
        antennas
    }

    fn find_antinodes(&self, limited: bool) -> HashSet<Pos> {
        let antennas = self.find_antennas();
        let mut antinodes = HashSet::<Pos>::new();
        for (_freq, antennas) in antennas.iter() {
            for antenna1 in antennas {
                for antenna2 in antennas {
                    if antenna1 != antenna2 {
                        antinodes.extend(self.antinodes(antenna1, antenna2, limited));
                    }
                }
            }
        }
        antinodes
    }

    fn is_in(&self, pos: &Pos) -> bool {
        pos.0 >= 0 && pos.0 < self.height() as isize && pos.1 >= 0 && pos.1 < self.width() as isize
    }

    fn antinodes(&self, a1: &Pos, a2: &Pos, limited: bool) -> Vec<Pos> {
        let (it1, it2) = Antinodes::for_antennas(a1, a2);
        if limited {
            Vec::from_iter(
                it1.skip(1)
                    .take(1)
                    .chain(it2.skip(1).take(1))
                    .filter(|pos| self.is_in(pos)),
            )
        } else {
            Vec::from_iter(
                it1.take_while(|pos| self.is_in(pos))
                    .chain(it2.take_while(|pos| self.is_in(pos))),
            )
        }
    }

    fn width(&self) -> usize {
        self.map.len()
    }

    fn height(&self) -> usize {
        self.map[0].len()
    }
}

type Pos = (isize, isize);

pub fn part1(input: &str) -> usize {
    World::read(input).find_antinodes(true).len()
}

pub fn part2(input: &str) -> usize {
    World::read(input).find_antinodes(false).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day8.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 394);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1277);
    }
}
