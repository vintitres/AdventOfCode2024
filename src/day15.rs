use std::collections::HashSet;

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn plus(&self, other: &Self) -> Self {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct World {
    map: Vec<Vec<char>>,
}

impl World {
    fn read(input: &str) -> (World, Pos, HashSet<Pos>) {
        let mut robot_pos = Pos { x: -1, y: -1 };
        let mut boxes = HashSet::<Pos>::new();
        let map = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars().enumerate().for_each(|(j, c)| match c {
                    '@' => {
                        robot_pos = Pos {
                            x: i as isize,
                            y: j as isize,
                        }
                    }
                    'O' => {
                        boxes.insert(Pos {
                            x: i as isize,
                            y: j as isize,
                        });
                    }
                    _ => (),
                });
                line.chars().collect_vec()
            })
            .collect_vec();
        (World { map }, robot_pos, boxes)
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
pub fn part1(input: &str) -> u64 {
    let (world, moves) = input.split("\n\n").collect_tuple().unwrap();
    let (world, mut robot, mut boxes) = World::read(world);
    let mut last_mv = '*';
    let mut last_mv_falied = false;
    dbg!(moves.trim());
    for mv in moves.trim().chars() {
        if mv == last_mv && last_mv_falied {
            continue;
        }
        last_mv = mv;
        let mv = match mv {
            '>' => Pos { x: 0, y: 1 },
            '^' => Pos { x: -1, y: 0 },
            '<' => Pos { x: 0, y: -1 },
            'v' => Pos { x: 1, y: 0 },
            '\n' => {
                continue;
            }
            _ => unimplemented!("unknown move: {:?}", mv),
        };
        let next_robot_pos = robot.plus(&mv);
        if boxes.contains(&next_robot_pos) {
            let mut next_box = next_robot_pos.plus(&mv);
            while boxes.contains(&next_box) {
                next_box = next_box.plus(&mv);
            }
            if world.open(&next_box).unwrap() {
                robot = next_robot_pos.clone();
                boxes.remove(&next_robot_pos);
                boxes.insert(next_box);
                last_mv_falied = false;
            } else {
                last_mv_falied = true;
            }
        } else if world.open(&next_robot_pos).unwrap() {
            robot = next_robot_pos.clone();
            last_mv_falied = false;
        } else {
            last_mv_falied = true;
        };
    }
    boxes.iter().map(|bx| (bx.x * 100 + bx.y) as u64).sum()
}

pub fn part2(input: &str) -> u64 {
    input.lines().count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day15.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1511865);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
