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

fn doit(input: &str, limited: bool) -> usize {
    let mut antennas = HashMap::new();
    let height = input.lines().count() as isize;
    let width = input.lines().next().unwrap().chars().count() as isize;
    input.lines().enumerate().for_each(|(i, row)| {
        row.chars().enumerate().for_each(|(j, c)| {
            if c != '.' {
                antennas
                    .entry(c)
                    .or_insert(HashSet::new())
                    .insert((i as isize, j as isize));
            }
        })
    });
    let mut antinodes_set = HashSet::<Pos>::new();
    for (_freq, antennas) in antennas.iter() {
        for antenna1 in antennas {
            for antenna2 in antennas {
                if antenna1 != antenna2 {
                    antinodes_set.extend(antinodes(antenna1, antenna2, limited, height, width));
                }
            }
        }
    }
    antinodes_set.len()
}

fn antinodes(a1: &Pos, a2: &Pos, limited: bool, height: isize, width: isize) -> Vec<Pos> {
    let is_in =
        |pos: &Pos| pos.0 >= 0 && pos.0 < height as isize && pos.1 >= 0 && pos.1 < width as isize;
    let (it1, it2) = Antinodes::for_antennas(a1, a2);
    if limited {
        Vec::from_iter(
            it1.skip(1)
                .take(1)
                .chain(it2.skip(1).take(1))
                .filter(|pos| is_in(pos)),
        )
    } else {
        Vec::from_iter(
            it1.take_while(|pos| is_in(pos))
                .chain(it2.take_while(|pos| is_in(pos))),
        )
    }
}

type Pos = (isize, isize);

pub fn part1(input: &str) -> usize {
    doit(input, true)
}

pub fn part2(input: &str) -> usize {
    doit(input, false)
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
