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

pub fn part1(input: &str) -> String {
    let (mut a, mut b, mut c, instr) = read(input);
    let combo = |operand, a, b, c| match operand {
        0..=3 => operand,
        4 => a,
        5 => b,
        6 => c,
        _ => unreachable!("unsupported combo operand: {}", operand),
    };
    let mut instr_i = 0;
    let instr = instr
        .iter()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| (chunk.next().unwrap(), chunk.next().unwrap()))
        .collect_vec();
    let mut output = Vec::new();
    while instr_i < instr.len() {
        // TODO movable loopp
        let (opcode, &operand) = instr[instr_i];
        instr_i += 1;
        match opcode {
            0 => {
                a = a / 2_u32.pow(combo(operand, a, b, c));
            }
            1 => {
                b ^= operand;
            }
            2 => {
                b = combo(operand, a, b, c) % 8;
            }
            3 => {
                if a != 0 {
                    assert!(operand % 2 == 0);
                    instr_i = (operand / 2) as usize;
                }
            }
            4 => {
                b ^= c;
            }
            5 => {
                output.push(combo(operand, a, b, c) % 8);
            }
            6 => {
                b = a / 2_u32.pow(combo(operand, a, b, c));
            }
            7 => {
                c = a / 2_u32.pow(combo(operand, a, b, c));
            }
            _ => unreachable!("unsuppoerted opcode: {:?}", opcode),
        }
    }
    output.iter().map(|n| n.to_string()).join(",")
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

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), "7,1,3,7,5,1,0,3,4".to_string());
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
