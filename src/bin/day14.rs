use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
enum Operation {
    /// Bitmask
    /// Various information for the value and address decoder.
    Mask {
        /// To clear requested bits, bitwise `and` the `clear` mask.
        clear: u64,
        /// To set requested bits, bitwise `and` the `set` mask.
        set: u64,
        /// `float` contains the bit indicies of the floating bits for challenge 2.
        float: Vec<usize>,
    },
    /// Memory operation to write `val` at address `addr`.
    Mem { addr: u64, val: u64 },
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (key, value) = {
            let mut iter = s.split('=');
            (
                iter.next().ok_or("Invalid input line")?.trim(),
                iter.next().ok_or("Invalid input line")?.trim(),
            )
        };

        if key.starts_with("mask") {
            let (clear, set, float) = value.chars().rev().enumerate().fold(
                (0u64, 0u64, Vec::new()),
                |(mut clear, mut set, mut float), (idx, c)| {
                    match c {
                        '1' => {
                            set |= 1 << idx;
                            clear |= 1 << idx;
                        }
                        'X' => {
                            set |= 0 << idx;
                            clear |= 1 << idx;
                            float.push(idx);
                        }
                        '0' => {}
                        _ => {
                            unimplemented!();
                        }
                    }
                    (clear, set, float)
                },
            );

            Ok(Operation::Mask { clear, set, float })
        } else if let Some(key) = key.strip_prefix("mem[") {
            let addr = key
                .chars()
                .filter_map(|c| c.to_digit(10))
                .fold(0u64, |acc, d| acc * 10 + u64::from(d));
            let val = value
                .parse()
                .map_err(|_| String::from("Invalid memory value"))?;

            Ok(Operation::Mem { addr, val })
        } else {
            Err("Invalid input key found".into())
        }
    }
}

fn challenge1(ops: &Vec<Operation>) -> u64 {
    let mut memory = HashMap::new();

    // Initial bit mask values
    let mut clear_mask = u64::MAX;
    let mut set_mask = 0;

    for op in ops {
        match op {
            &Operation::Mask { clear, set, .. } => {
                clear_mask = clear;
                set_mask = set;
            }
            Operation::Mem { addr, val } => {
                memory.insert(addr, (val | set_mask) & clear_mask);
            }
        }
    }

    memory.values().sum()
}

/// From `val` and `float` generate all possible values by recursively
/// mutating all floating bits and collect candidates in `candidates`.
fn permute(val: u64, float: &[usize], candidates: &mut Vec<u64>) {
    if float.is_empty() {
        candidates.push(val);
        return;
    }

    let val0 = val & !(1 << float[0]);
    let val1 = val | (1 << float[0]);
    permute(val0, &float[1..], candidates);
    permute(val1, &float[1..], candidates);
}

fn challenge2(ops: &Vec<Operation>) -> u64 {
    let mut memory = HashMap::new();

    let mut set_mask = 0;
    let mut floats = &Vec::new();

    for op in ops {
        match op {
            Operation::Mask { set, float, .. } => {
                set_mask = *set;
                floats = float;
            }
            Operation::Mem { addr, val } => {
                let mut addrs = Vec::new();
                permute(addr | set_mask, &floats, &mut addrs);

                for addr in addrs {
                    memory.insert(addr, *val);
                }
            }
        }
    }

    memory.values().sum()
}

fn parse_input() -> Result<Vec<Operation>, String> {
    aoc20::read_input_to_string("day14")
        .lines()
        .map(|l| l.parse::<Operation>())
        .collect()
}

fn main() -> Result<(), String> {
    let ops = parse_input()?;
    println!("{}", challenge1(&ops));
    println!("{}", challenge2(&ops));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_challenge1() -> Result<(), String> {
        assert_eq!(challenge1(&parse_input()?), 9879607673316);
        Ok(())
    }

    #[test]
    fn check_challenge2() -> Result<(), String> {
        assert_eq!(challenge2(&parse_input()?), 3435342392262);
        Ok(())
    }
}
