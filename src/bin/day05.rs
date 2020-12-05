use std::io::{self, BufRead};
use std::str::FromStr;

struct BoardingPass {
    row: u32,
    col: u32,
}

impl BoardingPass {
    fn uid(&self) -> u32 {
        // Just panic if overflow for now.
        self.row * 8 + self.col
    }
}

impl FromStr for BoardingPass {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Wrong input data length",
            ));
        }

        let row = s.chars().take(7).try_fold(0u32, |sum, c| match c {
            'B' => Ok((sum << 1) | 1),
            'F' => Ok((sum << 1) | 0),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "Need B/F in row specifier",
            )),
        })?;

        let col = s.chars().skip(7).take(3).try_fold(0u32, |sum, c| match c {
            'R' => Ok((sum << 1) | 1),
            'L' => Ok((sum << 1) | 0),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "Need B/F in col specifier",
            )),
        })?;

        Ok(BoardingPass { row, col })
    }
}

fn challenge1() -> io::Result<u32> {
    let mut max = 0u32;
    for l in aoc20::input_bufreader("day05").lines() {
        let uid = l?.parse::<BoardingPass>()?.uid();
        if uid > max {
            max = uid;
        }
    }
    Ok(max)
}

fn challenge2() -> io::Result<u32> {
    let bp_uids = {
        let mut v = aoc20::input_bufreader("day05")
            .lines()
            .map(|l| Ok(l?.parse::<BoardingPass>()?.uid()))
            .collect::<io::Result<Vec<_>>>()?;
        v.sort();
        v
    };

    // By challenge definition we must have more boarding passes.
    assert!(bp_uids.len() > 2);

    let mut prev_uid = bp_uids[0];
    for &uid in &bp_uids[1..] {
        if uid - prev_uid == 2 {
            return Ok(uid - 1);
        }
        prev_uid = uid;
    }
    Err(io::Error::new(
        io::ErrorKind::Other,
        "No free boarding pass uid found",
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
        assert_eq!(challenge1()?, 871);
        Ok(())
    }

    #[test]
    fn check_challenge2() -> io::Result<()> {
        assert_eq!(challenge2()?, 640);
        Ok(())
    }
}
