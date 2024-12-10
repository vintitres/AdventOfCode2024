use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::ops::Bound::{Included, Unbounded};
use std::usize;

use itertools::{DedupWithCount, Itertools};

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

#[derive(Debug)]
enum Block2 {
    Empty(usize, usize),     // pos, len
    File(usize, usize, u64), // pos, len, id
}

impl Block2 {
    fn checksum(&self) -> u64 {
        match self {
            Block2::File(pos, len, id) => (*pos..(pos + len)).sum::<usize>() as u64 * id,
            Block2::Empty(_, _) => 0,
        }
    }
}

pub fn part2(input: &str) -> u64 {
    let blocks: Vec<usize> = input
        .trim_end()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let mut pos = 0;
    let blocks: Vec<Block2> = blocks
        .iter()
        .enumerate()
        .map(|(i, length)| {
            let last_pos = pos;
            pos += length;
            match i % 2 {
                0 => Block2::File(last_pos, *length, (i / 2) as u64),
                1 => Block2::Empty(last_pos, *length),
                _ => unreachable!("% 2"),
            }
        })
        .collect();

    let mut empties = BTreeMap::new();
    let mut added_empties = BTreeSet::new();
    for block in blocks.iter() {
        match block {
            Block2::Empty(pos, length) => {
                empties
                    .entry(*length)
                    .or_insert(BTreeSet::new())
                    .insert(*pos);
                added_empties.insert((*pos, *length));
            }
            Block2::File(_, _, _) => (),
        };
    }

    let mut final_blocks = Vec::new();
    let mut checksum = 0;
    for block in blocks.iter().rev() {
        // dbg!(&empties);
        // dbg!(&added_empties);
        let last_file_pos;
        let mut add_empty = None;
        let mut drop_length = None;
        match block {
            Block2::File(pos, length, id) => {
                if *length == 0 {
                    continue;
                }
                last_file_pos = *pos;
                let mut longer = empties.range_mut((Included(length), Unbounded));
                if let Some((empty_length, length_empties)) = longer.next() {
                    let empty_pos = length_empties.pop_first().unwrap();
                    added_empties.remove(&(empty_pos, *empty_length));
                    assert!(*pos > empty_pos);
                    // dbg!(&block);
                    checksum += Block2::File(empty_pos, *length, *id).checksum();
                    final_blocks.push((empty_pos, *length));
                    // dbg!(checksum);
                    if length < empty_length {
                        add_empty = Some((empty_length - length, empty_pos + length));
                    }
                    if length_empties.is_empty() {
                        drop_length = Some(*empty_length);
                    }
                } else {
                    final_blocks.push((*pos, *length));
                    checksum += block.checksum();
                    // dbg!(&block);
                    // dbg!(checksum);
                }
            }
            Block2::Empty(pos, _) => {
                last_file_pos = *pos;
            }
        }
        // dbg!(add_empty);
        if let Some((length, pos)) = add_empty {
            empties.entry(length).or_insert(BTreeSet::new()).insert(pos);
            added_empties.insert((pos, length));
        }
        if let Some(length) = drop_length {
            assert!(empties[&length].is_empty());
            empties.remove(&length);
        }

        let mut to_del = Vec::new();
        for (empty_pos, empty_len) in
            added_empties.range((Included((last_file_pos + 1, 0)), Unbounded))
        {
            empties.get_mut(empty_len).unwrap().remove(empty_pos);
            if empties[empty_len].is_empty() {
                empties.remove(empty_len);
            }
            to_del.push((*empty_pos, *empty_len));
        }

        to_del.iter().for_each(|e| {
            added_empties.remove(e);
        });
    }

    /*
    2911213141
    00.........1.22.333.4444.
    004444.....1.22.333
    004444333..1.22
    004444333221
     */
    final_blocks.sort();
    let mut last_end = 0;
    for (s, e) in &final_blocks {
        assert!(*s >= last_end);
        last_end = *e;
    }

    final_blocks
        .iter()
        .map(|(_, len)| len)
        .sorted()
        .zip(
            blocks
                .iter()
                .flat_map(|b| match b {
                    Block2::Empty(_, _) => None,
                    Block2::File(_, l, _) => Some(l),
                })
                .sorted(),
        )
        .for_each(|(a, b)| assert_eq!(a, b));
    dbg!(&final_blocks);

    checksum
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
