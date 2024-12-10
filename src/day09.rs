use std::collections::VecDeque;

enum Block {
    Empty(usize),
    File(usize, u64),
}

impl Block {
    fn length(&self) -> usize {
        match self {
            Block::Empty(length) => *length,
            Block::File(length, _) => *length,
        }
    }
    fn is_empty_block(&self) -> bool {
        match self {
            Block::Empty(_) => true,
            Block::File(_, _) => false,
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let blocks: Vec<usize> = input
        .trim_end()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let size = blocks.iter().sum();
    let mut blocks: VecDeque<Block> = blocks
        .iter()
        .enumerate()
        .map(|(i, length)| match i % 2 {
            0 => Block::File(*length, (i / 2) as u64),
            1 => Block::Empty(*length),
            _ => unreachable!("% 2"),
        })
        .collect();
    let mut left_block = blocks.pop_front().unwrap();
    let mut right_block = blocks.pop_back().unwrap();
    let mut checksum: u64 = 0;
    for i in 0..size {
        match left_block {
            Block::Empty(length) => {
                assert_ne!(length, 0);
                match right_block {
                    Block::File(right_length, file_id) => {
                        if right_length == 0 {
                            unreachable!("empty right block");
                        }
                        checksum += i as u64 * file_id;
                        left_block = Block::Empty(length - 1);
                        right_block = Block::File(right_length - 1, file_id);
                    }
                    _ => unreachable!("empty right block"),
                }
            }
            Block::File(length, file_id) => {
                assert_ne!(length, 0);
                checksum += i as u64 * file_id;
                left_block = Block::File(length - 1, file_id);
            }
        }
        loop {
            if right_block.length() == 0 || right_block.is_empty_block() {
                if blocks.is_empty() {
                    if let Block::File(length, file_id) = left_block {
                        checksum += (i..(i + length)).sum::<usize>() as u64 * file_id;
                    }
                    return checksum;
                }
                right_block = blocks.pop_back().unwrap();
            } else {
                break;
            }
        }
        loop {
            if left_block.length() == 0 {
                left_block = if blocks.is_empty() {
                    match right_block {
                        Block::Empty(length) => {
                            right_block = Block::Empty(length - 1);
                            Block::Empty(1)
                        }
                        Block::File(length, file_id) => {
                            right_block = Block::File(length - 1, file_id);
                            Block::File(1, file_id)
                        }
                    }
                } else {
                    blocks.pop_front().unwrap()
                };
            } else {
                break;
            }
        }
    }
    unreachable!("!");
}

pub fn part2(input: &str) -> u64 {
    input.lines().count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day9.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 6211348208140);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
