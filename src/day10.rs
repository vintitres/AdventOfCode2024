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

    fn trailheads(&self, pos: (usize, usize)) -> usize {
        let pos = (pos.0 as isize, pos.1 as isize);
        if self.get(pos) == Some(0) {
            let t = self.trails(pos);
            dbg!(t);
            t
        } else {
            0
        }
    }

    fn trails(&self, pos: Pos) -> usize {
        if let Some(h) = self.get(pos) {
            if h == 9 {
                1
            } else {
                [(0, 1), (1, 0), (0, -1), (-1, 0)]
                    .iter()
                    .map(|(shiftx, shifty)| {
                        let npos = (pos.0 + shiftx, pos.1 + shifty);
                        if self.get(npos) == Some(h + 1) {
                            self.trails(npos)
                        } else {
                            0
                        }
                    })
                    .sum()
            }
        } else {
            0
        }
    }
}

pub fn part1(input: &str) -> usize {
    let w = World::read(input);
    (0..w.height())
        .flat_map(|i| (0..w.width()).map(move |j| (i, j)))
        .map(|(i, j)| w.trailheads((i, j)))
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input.lines().count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day10.txt")
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
