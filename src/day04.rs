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

fn count_xmas(puzzle: &[Vec<char>], i: usize, j: usize) -> usize {
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

pub fn part1(input: &str) -> usize {
    count(input, &count_xmas)
}

fn is_m_and_s(c1: char, c2: char) -> bool {
    (c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M')
}

fn is_x_mas_counter(puzzle: &[Vec<char>], i: usize, j: usize) -> usize {
    if is_x_mas(puzzle, i, j) {
        1
    } else {
        0
    }
}

fn is_x_mas(puzzle: &[Vec<char>], i: usize, j: usize) -> bool {
    match get(puzzle, i as isize, j as isize) {
        Some('A') => {
            (match get(puzzle, i as isize + 1, j as isize + 1) {
                Some(c1) => match get(puzzle, i as isize - 1, j as isize - 1) {
                    Some(c2) => is_m_and_s(c1, c2),
                    _ => false,
                },
                _ => false,
            }) && (match get(puzzle, i as isize + 1, j as isize - 1) {
                Some(c1) => match get(puzzle, i as isize - 1, j as isize + 1) {
                    Some(c2) => is_m_and_s(c1, c2),
                    _ => false,
                },
                _ => false,
            })
        }
        _ => false,
    }
}

fn count(input: &str, counter_fn: &dyn Fn(&[Vec<char>], usize, usize) -> usize) -> usize {
    let puzzle: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    (0..puzzle.len())
        .map(|i| {
            (0..puzzle[i].len())
                .map(|j| counter_fn(&puzzle, i, j))
                .sum::<usize>()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    count(input, &is_x_mas_counter)
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1905);
    }
}
