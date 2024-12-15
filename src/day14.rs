use itertools::Itertools;

struct Limit {
    x: isize,
    y: isize,
}

struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn wrapone(x: isize, limitx: isize) -> isize {
        let mut x = x;
        if x < 0 {
            x = x + limitx * (1 + (limitx - x) / limitx);
        }
        x % limitx
    }
    fn wraped(&self, limit: &Limit) -> Pos {
        Pos {
            x: Self::wrapone(self.x, limit.x),
            y: Self::wrapone(self.y, limit.y),
        }
    }
    fn moved(&self, by: &Pos, limit: &Limit) -> Pos {
        Pos {
            x: self.x + by.x,
            y: self.y + by.y,
        }
        .wraped(limit)
    }
}

struct Robot {
    pos: Pos,
    vel: Pos,
}

impl Robot {
    fn read(line: &str) -> Robot {
        let (pos, vel) = line
            .split(" ")
            .map(|t| {
                let (x, y) = t[2..]
                    .split(",")
                    .map(|n| n.parse::<isize>().unwrap())
                    .collect_tuple()
                    .unwrap();
                Pos { x, y }
            })
            .collect_tuple()
            .unwrap();
        Robot { pos, vel }
    }

    fn step(&mut self, limit: &Limit) {
        self.pos = self.pos.moved(&self.vel, limit);
    }

    fn steps(&mut self, count: usize, limit: &Limit) {
        (0..count).for_each(|_| self.step(limit));
    }

    fn quardant(&self, limit: &Limit) -> usize {
        if self.pos.x < limit.x / 2 {
            if self.pos.y < limit.y / 2 {
                1
            } else if self.pos.y > limit.y / 2 {
                2
            } else {
                0
            }
        } else if self.pos.x > limit.x / 2 {
            if self.pos.y < limit.y / 2 {
                3
            } else if self.pos.y > limit.y / 2 {
                4
            } else {
                0
            }
        } else {
            0
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let mut robots = input.lines().map(Robot::read).collect_vec();
    let limit = Limit { x: 101, y: 103 };
    robots.iter_mut().for_each(|r| r.steps(100, &limit));
    let mut counts = [0; 5];
    robots.iter().for_each(|r| counts[r.quardant(&limit)] += 1);
    counts[1..].iter().product()
}

pub fn part2(input: &str) -> u64 {
    input.lines().count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day14.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 222901875);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
