use std::io;

/// Given `curr_val` in cypher and window of previous values `prev_vals`,
/// check if `curr_val` full fills XMAS cypher requirements.
/// - `curr_val` must be sum of any 2 values in `prev_vals`
/// - summands must have different values
fn is_valid(curr_val: usize, prev_vals: &[usize]) -> bool {
    for (i, n1) in prev_vals.iter().enumerate() {
        for n2 in prev_vals.iter().skip(i + 1) {
            if n1 == n2 {
                continue;
            }
            if n1 + n2 == curr_val {
                return true;
            }
        }
    }
    false
}

fn challenge1() -> io::Result<usize> {
    let input = aoc20::read_input_to_string("day09")
        .lines()
        .map(|n| {
            n.parse::<usize>().map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "Input line is not a number")
            })
        })
        .collect::<io::Result<Vec<_>>>()?;

    const WLEN: usize = 25 /* previous window */ + 1 /* current value */;

    for window in input.windows(WLEN) {
        let (curr_val, prev_vals) = window.split_last().expect("WLEN > 2");
        if !is_valid(*curr_val, prev_vals) {
            return Ok(*curr_val);
        }
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "No invalid value found in input cypher",
    ))
}

fn challenge2() -> io::Result<usize> {
    let input = aoc20::read_input_to_string("day09")
        .lines()
        .map(|n| {
            n.parse::<usize>().map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "Input line is not a number")
            })
        })
        .collect::<io::Result<Vec<_>>>()?;

    // Invalid number in input cypher as determined in challenge1.
    const INVALID_VAL: usize = 257342611;

    for (i, n1) in input.iter().enumerate() {
        let mut min = n1;
        let mut max = n1;

        let mut sum = *n1;
        for n2 in input.iter().skip(i + 1) {
            sum += n2;

            // Overshoot, try again.
            if sum > INVALID_VAL {
                break;
            }

            // Track min/max values in current sequence.
            if n2 > max {
                max = n2;
            } else if n2 < min {
                min = n2;
            }

            // Check if current sequence matches the invalid number.
            if sum == INVALID_VAL {
                return Ok(min + max);
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "No sequence found that sums up to the invalid number",
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
        assert_eq!(challenge1()?, 257342611);
        Ok(())
    }

    #[test]
    fn check_challenge2() -> io::Result<()> {
        assert_eq!(challenge2()?, 35602097);
        Ok(())
    }
}
