use itertools::Itertools;

fn read(input: &str) -> (u32, u32, u32, Vec<u32>) {
    let (a, b, c, i): (&str, &str, &str, &str) = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split(": ").nth(1).unwrap())
        .collect_tuple()
        .unwrap();
    (
        a.parse::<u32>().unwrap(),
        b.parse::<u32>().unwrap(),
        c.parse::<u32>().unwrap(),
        i.split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect_vec(),
    )
}

pub fn part1(input: &str) -> u32 {
    let (mut a, mut b, mut c, instr) = read(input);
    let combo = |operand| match operand {
        0 | 1 | 2 | 3 => operand,
        4 => a,
        5 => b,
        6 => c,
        7 => unreachable!("combo operand 7 unsupported"),
    };
    for mut chunk in &instr.iter().chunks(2) {
        // TODO movable loopp
        let opcode = chunk.next().unwrap();
        let operand = *chunk.next().unwrap();
        match opcode {
            0 => {
                a = a / 2_u32.pow(combo(operand));
            }
            1 => {
                b = b ^ operand;
            }
            2 => {
                b = combo(operand) % 8;
            }
            3 => {}
        }
    }
    0
}

pub fn part2(input: &str) -> u32 {
    input.lines().count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day17.txt")
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
