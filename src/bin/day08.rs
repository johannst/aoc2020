use std::collections::HashSet;
use std::io;

#[derive(Default)]
struct HandHeld {
    pc: usize,
    acc: isize,
}

#[derive(PartialEq)]
enum ExitReason {
    Inf,
    End,
}

impl HandHeld {
    fn run(&mut self, code: &Vec<&str>) -> io::Result<ExitReason> {
        let mut prev_pcs = HashSet::new();

        loop {
            prev_pcs.insert(self.pc);

            let instr = code
                .get(self.pc)
                .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Illegal PC"))?;

            match HandHeld::decode(instr)? {
                ("acc", n) => {
                    self.acc += n;
                    self.pc += 1;
                }
                ("jmp", n) => {
                    if n.is_positive() {
                        self.pc += n as usize;
                    } else {
                        self.pc -= n.abs() as usize;
                    }
                }
                ("nop", _) => {
                    self.pc += 1;
                }
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid instruction encountered",
                    ));
                }
            }

            if self.pc == code.len() {
                return Ok(ExitReason::End);
            } else if prev_pcs.contains(&self.pc) {
                return Ok(ExitReason::Inf);
            }
        }
    }

    fn decode(instr: &str) -> io::Result<(&str, isize)> {
        let mut instr = instr.splitn(2, ' ');

        let op = instr.next().ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "No OPCODE in input",
        ))?;

        let arg = instr
            .next()
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "No ARGUMENT in input",
            ))?
            .parse::<isize>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "ARGUMENT is not a number"))?;

        Ok((op, arg))
    }
}

fn challenge1() -> io::Result<isize> {
    let input = aoc20::read_input_to_string("day08");
    let boot_code: Vec<&str> = input.lines().collect();

    let mut h = HandHeld::default();
    h.run(&boot_code)?;
    Ok(h.acc)
}

fn challenge2() -> io::Result<isize> {
    let input = aoc20::read_input_to_string("day08");
    let mut boot_code: Vec<&str> = input.lines().collect();

    struct Patch<'a> {
        old: &'a str,
        new: String,
        idx: usize,
    }

    // Collect all patch candidates.
    let patches: Vec<_> = boot_code
        .iter()
        .enumerate()
        .filter_map(|(idx, instr)| {
            let new = match &instr[0..3] {
                "jmp" => instr.replace("jmp", "nop"),
                "nop" => instr.replace("nop", "jmp"),
                _ => return None,
            };
            Some(Patch {
                old: instr,
                new,
                idx,
            })
        })
        .collect();

    for patch in &patches {
        // Replace original instruction with patched instruction.
        boot_code[patch.idx] = patch.new.as_str();

        // Evaluate patched boot code.
        let mut h = HandHeld::default();
        if h.run(&boot_code)? == ExitReason::End {
            return Ok(h.acc);
        }

        // Replace patched instruction with original instruction.
        boot_code[patch.idx] = patch.old;
    }

    Err(io::Error::new(
        io::ErrorKind::Other,
        "No proper patch found",
    ))
}

fn main() -> io::Result<()> {
    println!("{}", challenge1()?);
    println!("{}", challenge2()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_challenge1() -> io::Result<()> {
        assert_eq!(challenge1()?, 1810);
        Ok(())
    }

    #[test]
    fn check_challenge2() -> io::Result<()> {
        assert_eq!(challenge2()?, 969);
        Ok(())
    }
}
