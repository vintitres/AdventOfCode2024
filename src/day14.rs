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

pub fn part1(input: &str) -> u64 {
    input.lines().count() as u64
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
