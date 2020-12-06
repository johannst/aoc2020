use std::io;

#[cfg(target_arch = "x86_64")]
fn popcnt32(x: u32) -> u32 {
    unsafe { core::arch::x86_64::_popcnt32(x as i32) as u32 }
}

#[cfg(not(target_arch = "x86_64"))]
fn popcnt32(x: u32) -> u32 {
    (0..32).map(|shamt| (x >> shamt) & 0b1).sum()
}

fn challenge1() -> io::Result<u32> {
    let mut total = 0;
    for group_answers in aoc20::read_input_to_string("day06").split("\n\n") {
        let mut anyone_yes = 0;
        for answer in group_answers.chars() {
            match answer {
                'a'..='z' => {
                    let shamt = |c: char| c as u32 - 'a' as u32;
                    anyone_yes |= 1 << shamt(answer);
                }
                '\n' => {} // new person -> ignore
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid answer must be in range a-z",
                    ))
                }
            }
        }
        total += popcnt32(anyone_yes);
    }
    Ok(total)
}

fn challenge2() -> io::Result<u32> {
    let mut total = 0;
    for group_answers in aoc20::read_input_to_string("day06").split("\n\n") {
        let everyone_yes: io::Result<u32> = group_answers
            .lines()
            .map(|person_answers| {
                person_answers
                    .chars()
                    .try_fold(0u32, |person_yes, answer| match answer {
                        'a'..='z' => {
                            let shamt = |c: char| c as u32 - 'a' as u32;
                            Ok(person_yes | (1 << shamt(answer)))
                        }
                        _ => Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Invalid answer must be in range a-z",
                        )),
                    })
            })
            .try_fold(std::u32::MAX, |everyone_yes, person_yes| {
                Ok(everyone_yes & person_yes?)
            });
        total += popcnt32(everyone_yes?);
    }
    Ok(total)
}

fn main() -> io::Result<()> {
    println!("{:?}", challenge1()?);
    println!("{:?}", challenge2()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_challenge1() -> io::Result<()> {
        assert_eq!(challenge1()?, 6532);
        Ok(())
    }

    #[test]
    fn check_challenge2() -> io::Result<()> {
        assert_eq!(challenge2()?, 3427);
        Ok(())
    }
}
