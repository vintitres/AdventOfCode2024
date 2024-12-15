use std::collections::{HashSet,HashMap};

use itertools::{concat, Itertools};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Dir {
    Up, Left, Right, Down
}

impl Dir {
    fn read(c: char) -> Option<Dir> {
        match c {
            '>' => Some(Dir::Right),
            '^' => Some(Dir::Up),
            '<' => Some(Dir::Left),
            'v' => Some(Dir::Down),
            '\n' => None,
            _ => unreachable!("unknown direction: {:?}", c)
        }
    }

    fn shift(&self) -> Pos {
        match self {
            Dir::Right => Pos { x: 0, y: 1 },
            Dir::Up => Pos { x: -1, y: 0 },
            Dir::Left => Pos { x: 0, y: -1 },
            Dir::Down => Pos { x: 1, y: 0 },
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn left(&self) -> Self {
        Pos {x: self.x, y : self.y - 1}
    }
    fn right(&self) -> Self {
        Pos {x: self.x, y : self.y + 1}
    }

    fn plus(&self, other: &Self) -> Self {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn double(&self) -> Self {
        Pos {
            x: self.x * 2,
            y: self.y,
        }
    }
    fn double_box(&self) -> (Self, Self) {
        (Pos {
            x: self.x * 2,
            y: self.y,
        },
        Pos {
            x: self.x * 2 + 1,
            y: self.y,
        })
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
    fn double_width(&self) -> Self {
        let map = self.map.iter().map(|row| row.iter().flat_map(|c| [*c, *c]).collect_vec()).collect_vec();
        World {map}
    }
}

pub fn part1(input: &str) -> u64 {
    let (world, moves) = input.split("\n\n").collect_tuple().unwrap();
    let (world, mut robot, mut boxes) = World::read(world);
    let mut last_mv = None;
    let mut last_mv_falied = false;
    for mv in moves.trim().chars() {
        let mv = Dir::read(mv);
        if mv.is_none() || last_mv_falied && mv == last_mv {
            continue;
        }
        last_mv = mv;
        let mv = mv.unwrap().shift();
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

fn try_move_box(boxes: &mut HashMap<Pos, usize>, at_pos: &Pos, box_id: usize, mv: &Dir) -> Option<usize> {
    None
    /*
    let at_pos_2 = if let Some(id) = boxes.get(&at_pos.left()) && box_id == id {
        at_pos.left()
    } else {
        assert!(boxes.get(&at_pos.right()).unwrap() == box_id);
        at_pos.right()
    };

    let mut moved = None;
    match mv {

    }
    for pos in [*at_pos, at_pos_2] {
        let next_pos = pos.plus(mv);
        if boxes
    }
    */


}


pub fn part2(input: &str) -> u64 {
    let (world, moves) = input.split("\n\n").collect_tuple().unwrap();
    let (world, robot, boxes) = World::read(world);
    let world = world.double_width();
    let mut robot = robot.double();
    let mut boxes = HashMap::from_iter(boxes.iter().enumerate().flat_map(|(i, bx)| {
        let (l, r) = bx.double_box();
        [(l, i), (r, i)]
    }));
    let mut last_mv = None;
    let mut last_mv_falied = false;
    for mv in moves.trim().chars() {
        let mv = Dir::read(mv);
        if mv.is_none() || last_mv_falied && mv == last_mv {
            continue;
        }
        last_mv = mv;
        let mv = mv.unwrap();
        let mvp = mv.shift();
        let next_robot_pos = robot.plus(&mvp);
        if let Some(i) = boxes.get(&next_robot_pos).cloned() {
            if try_move_box(&mut boxes, &next_robot_pos, i, &mv).is_some() {
                robot = next_robot_pos.clone();
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
    let mut boxes2 = HashMap::<usize, Vec<Pos>>::new();
    boxes.iter().for_each(|(bx, i)| boxes2.entry(*i).or_insert(Vec::new()).push(bx.clone()));
    boxes2.values().map(|bxs| (std::cmp::min(bxs[0].x, bxs[1].x) * 100 + bxs[0].y) as u64).sum()

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
