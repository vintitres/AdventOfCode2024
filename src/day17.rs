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

fn run(a: u64, b: u32, c: u32, instr: &[u32]) -> Vec<u32> {
    let mut a = a;
    let mut b = b;
    let mut c = c;
    let combo = |operand, a, b, c| match operand {
        0..=3 => operand,
        4 => (a % 8) as u32,
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
        let (opcode, &operand) = instr[instr_i];
        instr_i += 1;
        match opcode {
            0 => {
                a = a / 2_u64.pow(combo(operand, a, b, c));
                // dbg!("adv", operand, combo(operand, a, b, c));
            }
            1 => {
                b ^= operand;
                // dbg!("bxl", operand);
            }
            2 => {
                b = combo(operand, a, b, c) % 8;
                // dbg!("bst", operand, combo(operand, a, b, c));
            }
            3 => {
                if a != 0 {
                    assert!(operand % 2 == 0);
                    instr_i = (operand / 2) as usize;
                }
                // dbg!("jnz", operand / 2);
            }
            4 => {
                b ^= c;
                // dbg!("bxc");
            }
            5 => {
                let o = combo(operand, a, b, c) % 8;
                output.push(o);
                // dbg!("out", operand, combo(operand, a, b, c) % 8);
            }
            6 => {
                b = (a / 2_u64.pow(combo(operand, a, b, c)) % 8) as u32;
                // dbg!("bdv", operand, combo(operand, a, b, c));
            }
            7 => {
                c = (a / 2_u64.pow(combo(operand, a, b, c)) % 8) as u32;
                // dbg!("cdv", operand, combo(operand, a, b, c));
            }
            _ => unreachable!("unsuppoerted opcode: {:?}", opcode),
        }
    }
    output
}

pub fn part1(input: &str) -> String {
    let (a, b, c, instr) = read(input);
    run(a as u64, b, c, &instr)
        .iter()
        .map(|n| n.to_string())
        .join(",")
}

pub fn part2(input: &str) -> u64 {
    let (_a, _b, _c, instr) = read(input);
    let mut a = 0_u64;
    for i in instr.iter().rev() {
        a *= 8;
        for sub_a in 0..8 {
            if run(a + sub_a, 0, 0, &instr).first() == Some(i) {
                a += sub_a;
                break;
            }
        }
    }
    a
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 190384113204239);
    }
}
