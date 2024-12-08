use std::collections::{HashMap, HashSet};

use itertools::Itertools;

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
                    antinodes.extend(self.antinodes(antenna1, antenna2, limited).iter());
                }
            }
        }
        antinodes
    }

    fn antinodes(&self, a1: &Pos, a2: &Pos, limited: bool) -> Vec<Pos> {
        let mut v = vec![];
        if *a1 != *a2 {
            let diffx = a1.0 - a2.0;
            let diffy = a1.1 - a2.1;
            let mut x = a2.0 - diffx;
            let mut y = a2.1 - diffy;
            while x >= 0 && x < self.height() as isize && y >= 0 && y < self.width() as isize {
                v.push((x, y));
                x -= diffx;
                y -= diffy;
                if limited {
                    break;
                }
            }
            let mut x = a1.0 + diffx;
            let mut y = a1.1 + diffy;
            while x >= 0 && x < self.height() as isize && y >= 0 && y < self.width() as isize {
                v.push((x, y));
                x += diffx;
                y += diffy;
                if limited {
                    break;
                }
            }
            if !limited {
                v.push(*a1);
                v.push(*a2);
            }
        }
        v
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
