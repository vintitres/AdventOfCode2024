const XMAS: &[char] = &['X', 'M', 'A', 'S'];

fn get(puzzle: &[Vec<char>], i: isize, j: isize) -> Option<char> {
    if i < 0 || j < 0 {
        return None;
    }
    if let Ok(ii) = usize::try_from(i) {
        if let Ok(jj) = usize::try_from(j) {
            if let Some(row) = puzzle.get(ii) {
                return row.get(jj).copied();
            }
        }
    }
    None
}

fn count(puzzle: &[Vec<char>], i: usize, j: usize) -> u64 {
    let moves = &[
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    moves
        .iter()
        .map(|(mx, my)| {
            let mut x = i as isize;
            let mut y = j as isize;
            for step in 0..XMAS.len() {
                if let Some(c) = get(puzzle, x, y) {
                    if c != *XMAS.get(step).unwrap() {
                        return 0;
                    }
                } else {
                    return 0;
                }
                x += mx;
                y += my;
            }
            //dbg!("{:?}, {}, {}", (mx, my), i, j);
            1
        })
        .sum()
}

pub fn part1(input: &str) -> u64 {
    let puzzle: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    (0..puzzle.len())
        .map(|i| {
            (0..puzzle[i].len())
                .map(|j| count(&puzzle, i, j))
                .sum::<u64>()
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input.lines().count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day4.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 2613);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
