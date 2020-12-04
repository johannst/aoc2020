use std::io::{self, BufRead};

struct Slope {
    right: usize,
    down: usize,
}

fn try_cnt_tree_on_slope(slope: &Slope) -> io::Result<usize> {
    let mut cur_xpos: usize = 0;

    let elements_on_route: std::io::Result<Vec<_>> = aoc20::input_bufreader("day03")
        .lines()
        .step_by(slope.down)
        .map(|l| {
            let l = l?;
            let c = l.chars().nth(cur_xpos).ok_or(io::Error::new(
                io::ErrorKind::Other,
                "Invalid input line, too short",
            ))?;
            cur_xpos = (cur_xpos + slope.right) % l.len();
            Ok(c)
        })
        .collect();
    Ok(elements_on_route?.iter().filter(|&&c| c == '#').count())
}

fn challenge1() -> io::Result<usize> {
    let s = Slope { right: 3, down: 1 };
    try_cnt_tree_on_slope(&s)
}

fn challenge2() -> io::Result<usize> {
    let mut cnt_trees = 1;
    for s in &[
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ] {
        cnt_trees *= try_cnt_tree_on_slope(s)?;
    }
    Ok(cnt_trees)
}

fn main() -> io::Result<()> {
    println!("Number trees challenge1: {}", challenge1()?);
    println!("Number trees challenge2: {}", challenge2()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_challenge1() -> io::Result<()> {
        assert_eq!(challenge1()?, 151);
        Ok(())
    }

    #[test]
    fn check_challenge2() -> io::Result<()> {
        assert_eq!(challenge2()?, 7540141059);
        Ok(())
    }
}
