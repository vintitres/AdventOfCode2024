pub fn part1(input: &str) -> u64 {
    let mut chars = input.chars();
    let mut last_index = 0;
    let indices: Vec<usize> = input.match_indices("mul(").map(|(index, _)| index).collect();
   indices.into_iter().map(|index|{
        //chars.skip(index + 4 - last_index);
        while last_index < index + 4 {
            last_index += 1;
            chars.next();
        }
        let mut n1: u64 = 0;
        while let Some(c) = chars.next() {
            last_index += 1;
            if c.is_ascii_digit() {
                n1 *= 10;
                n1 += c.to_digit(10).unwrap() as u64;
            } else if c == ',' {
                break;
            } else {
                return 0;
            }
        }
        let mut n2 = 0;
        while let Some(c) = chars.next() {
            last_index += 1;
            if c.is_ascii_digit() {
                n2 *= 10;
                n2 += c.to_digit(10).unwrap() as u64;
            } else if c == ')' {
                return n1 * n2;
            } else {
                return 0;
            }
        }
        0
    }).sum()
}

pub fn part2(input: &str) -> u64 {
    input.lines().count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day3.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 173517243);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
