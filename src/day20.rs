use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Left,
    Right,
    Down,
}

impl Dir {
    const ALL: [Dir; 4] = [Dir::Up, Dir::Right, Dir::Down, Dir::Left];
}

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Copy, Debug)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new_from_usize(x: usize, y: usize) -> Pos {
        Pos {
            x: x as isize,
            y: y as isize,
        }
    }
    fn next(&self, dir: &Dir) -> Pos {
        let (x, y) = match dir {
            Dir::Down => (self.x + 1, self.y),
            Dir::Up => (self.x - 1, self.y),
            Dir::Left => (self.x, self.y - 1),
            Dir::Right => (self.x, self.y + 1),
        };
        Pos { x, y }
    }
}

struct World {
    map: Vec<Vec<char>>,
}

impl World {
    fn read(input: &str) -> (World, Pos, Pos) {
        let mut start = None;
        let mut end = None;
        let map = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                if let Some(j) = line.find('S') {
                    start = Some(Pos::new_from_usize(i, j));
                }
                if let Some(j) = line.find('E') {
                    end = Some(Pos::new_from_usize(i, j));
                }
                line.chars().collect_vec()
            })
            .collect_vec();
        (World { map }, start.unwrap(), end.unwrap())
    }
    /*
    fn draw(&self, path: &[Pos]) {
        let path: HashSet<Pos> = HashSet::from_iter(path.iter().cloned());
        self.map.iter().enumerate().for_each(|(i, row)| {
            println!(
                "{}",
                row.iter()
                    .enumerate()
                    .map(|(j, c)| if path.contains(&Pos::new_from_usize(i, j)) {
                        if *c == '#' {
                            'x'
                        } else {
                            'o'
                        }
                    } else {
                        *c
                    })
                    .collect::<String>()
            );
        });
    }
    */
    fn open(&self, pos: &Pos) -> Option<bool> {
        if let Ok(i) = usize::try_from(pos.x) {
            if let Ok(j) = usize::try_from(pos.y) {
                if let Some(map_row) = self.map.get(i) {
                    if let Some(&c) = map_row.get(j) {
                        return Some(c != '#');
                    }
                }
            }
        }
        None
    }
}

fn doit(
    world: &World,
    start: Pos,
    end: Pos,
    limit: usize,
    cheat: Option<Pos>,
) -> Option<(usize, HashMap<Pos, usize>)> {
    let mut pq = BTreeSet::new();
    pq.insert((0, start));
    let mut best_score = HashMap::new();
    while !pq.is_empty() {
        let (score, pos) = pq.pop_first().unwrap();
        if score > limit {
            return None;
        }
        if pos == end {
            best_score.insert(pos, score);
            return Some((score, best_score));
        }
        if Some(pos) != cheat && !world.open(&pos).unwrap() {
            continue;
        }
        match score.cmp(best_score.get(&pos).unwrap_or(&usize::MAX)) {
            std::cmp::Ordering::Less => (),
            std::cmp::Ordering::Equal => {
                continue;
            }
            std::cmp::Ordering::Greater => {
                continue;
            }
        }
        best_score.insert(pos, score);
        for dir in Dir::ALL {
            pq.insert((score + 1, pos.next(&dir)));
        }
    }
    None
}

pub fn part1(input: &str) -> usize {
    let (world, start, end) = World::read(input);
    let nocheat = doit(&world, start, end, usize::MAX, None).unwrap();
    doit2(&world, nocheat.0 - SAVE, &nocheat.1, 1)
}

const SAVE: usize = 100;

fn cheat(
    score: usize,
    pos: Pos,
    limit: usize,
    best_score: &HashMap<Pos, usize>,
    world: &World,
    cheat_size: usize,
) -> usize {
    let mut paths = 0;
    let mut q = VecDeque::new();
    for dir in Dir::ALL {
        q.push_back((pos.next(&dir), score + 1, cheat_size));
    }
    let mut seen = HashSet::new();
    while !q.is_empty() {
        let (pos, score, steps_left) = q.pop_front().unwrap();
        if seen.contains(&pos) {
            continue;
        }
        seen.insert(pos);
        if score > limit {
            break;
        }
        match world.open(&pos) {
            None => {
                continue;
            }
            Some(true) => {
                let bscore = *best_score.get(&pos).unwrap();
                if score + SAVE <= bscore {
                    paths += 1;
                }
            }
            Some(false) => {}
        }
        if steps_left == 0 {
            continue;
        }
        for dir in Dir::ALL {
            q.push_back((pos.next(&dir), score + 1, steps_left - 1));
        }
    }
    paths
}

fn doit2(
    world: &World,
    limit: usize,
    best_score: &HashMap<Pos, usize>,
    cheat_size: usize,
) -> usize {
    best_score
        .iter()
        .map(|(&pos, &score)| cheat(score, pos, limit, best_score, world, cheat_size))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let (world, start, end) = World::read(input);
    let nocheat = doit(&world, start, end, usize::MAX, None).unwrap();
    doit2(&world, nocheat.0 - SAVE, &nocheat.1, 19)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day20.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1450);
    }

    #[ignore = "slow"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1015247);
    }
}
