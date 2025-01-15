use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

#[derive(Debug)]
enum Oper {
    Or,
    Xor,
    And,
}

impl Oper {
    fn parse(s: &str) -> Oper {
        match s {
            "XOR" => Oper::Xor,
            "OR" => Oper::Or,
            "AND" => Oper::And,
            _ => unimplemented!(),
        }
    }

    fn eval(&self, v1: bool, v2: bool) -> bool {
        match self {
            Oper::And => v1 && v2,
            Oper::Or => v1 || v2,
            Oper::Xor => v1 ^ v2,
        }
    }
}

#[derive(Debug)]
struct Gate {
    inputs: (String, String),
    op: Oper,
    output: String,
}

impl Gate {
    fn parse(line: &str) -> Gate {
        let (i1, op, i2, _, out) = line.split(' ').collect_tuple().unwrap();
        Gate {
            inputs: (i1.to_string(), i2.to_string()),
            op: Oper::parse(op),
            output: out.to_string(),
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let values: HashMap<&str, bool> = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (name, val) = l.split(": ").collect_tuple().unwrap();
            (name, val == "1")
        })
        .collect();
    let gates = input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(Gate::parse)
        .collect_vec();
    let mut inputs = HashMap::new();
    for (i, gate) in gates.iter().enumerate() {
        inputs
            .entry(gate.inputs.0.clone())
            .or_insert(HashSet::new())
            .insert(i);
        inputs
            .entry(gate.inputs.1.clone())
            .or_insert(HashSet::new())
            .insert(i);
    }
    let mut q = VecDeque::from_iter(values);
    let mut values = HashMap::new();
    while !q.is_empty() {
        let (wire, val) = q.pop_front().unwrap();
        values.insert(wire, val);
        for gate_i in inputs.entry(wire.to_string()).or_default().iter() {
            let gate = gates.get(*gate_i).unwrap();
            if let (Some(v1), Some(v2)) = (
                values.get(gate.inputs.0.as_str()),
                values.get(gate.inputs.1.as_str()),
            ) {
                let val = gate.op.eval(*v1, *v2);
                values.insert(&gate.output, val);
                q.push_back((&gate.output, val));
            }
        }
    }
    let mut n = 0;
    (0..63)
        .map(|n| "z".to_owned() + (if n < 10 { "0" } else { "" }) + &n.to_string())
        .flat_map(|name| values.get(name.as_str()))
        .rev()
        .for_each(|bit| {
            n *= 2;
            n += if *bit { 1 } else { 0 };
        });

    n
}

pub fn part2(input: &str) -> u64 {
    input.lines().count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day24.txt")
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 55730288838374); // ?
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
