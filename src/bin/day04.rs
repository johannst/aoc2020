use std::convert::TryFrom;
use std::io;

// Revert back to un-typed fields because in challenge1 input values
// contain invalid data, maybe challenge2 adds extra validation.
#[derive(Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    /// Validate passport for challenge1.
    ///
    /// The passport must contain data in all fields except `cid` can be empty.
    fn valid1(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    /// Validate passport for challenge2.
    ///
    /// Additional checks:
    ///  - byr (Birth Year): Four digits; at least 1920 and at most 2002.
    ///  - iyr (Issue Year): Four digits; at least 2010 and at most 2020.
    ///  - eyr (Expiration Year): four digits; at least 2020 and at most 2030.
    ///  - hgt (Height): A number followed by either cm or in:
    ///                  If cm, the number must be at least 150 and at most 193.
    ///                  If in, the number must be at least 59 and at most 76.
    ///  - hcl (Hair Color): A # followed by exactly six characters 0-9 or a-f.
    ///  - ecl (Eye Color): Exactly one of: amb blu brn gry grn hzl oth.
    ///  - pid (Passport ID): A nine-digit number, including leading zeroes.
    ///  - cid (Country ID): Ignored, missing or not.
    fn valid2(&self) -> bool {
        self._valid2().is_some()
    }

    fn _valid2(&self) -> Option<()> {
        // Validate Birth Year.
        let byr: usize = self.byr.as_ref()?.parse().ok()?;
        if byr < 1920 || byr > 2002 {
            return None;
        }

        // Validate Issue Year.
        let iyr: usize = self.iyr.as_ref()?.parse().ok()?;
        if iyr < 2010 || iyr > 2020 {
            return None;
        }

        // Validate Expiration Year.
        let eyr: usize = self.eyr.as_ref()?.parse().ok()?;
        if eyr < 2020 || eyr > 2030 {
            return None;
        }

        // Validate Height.
        if let Some(hgt_str) = self.hgt.as_ref()?.strip_suffix("cm") {
            let hgt: usize = hgt_str.parse().ok()?;
            if hgt < 150 || hgt > 193 {
                return None;
            }
        } else if let Some(hgt_str) = self.hgt.as_ref()?.strip_suffix("in") {
            let hgt: usize = hgt_str.parse().ok()?;
            if hgt < 59 || hgt > 76 {
                return None;
            }
        } else {
            return None;
        }

        // Validate Hair Color.
        if let Some(hcl_str) = self.hcl.as_ref()?.strip_prefix("#") {
            if !hcl_str.len() == 6 {
                return None;
            }
            u32::from_str_radix(hcl_str, 16).ok()?;
        } else {
            return None;
        }

        // Validate Eye Color.
        if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&self.ecl.as_ref()?) {
            return None;
        }

        // Validate Passport ID.
        if self.pid.as_ref()?.len() != 9 {
            return None;
        }

        Some(())
    }
}

impl TryFrom<&str> for Passport {
    type Error = io::Error;

    fn try_from(value: &str) -> io::Result<Self> {
        let mut p = Passport::default();
        for entry in value.split_whitespace() {
            let (k, v) = {
                let mut tokens = entry.split(':');
                (
                    tokens.next().ok_or(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Failed to extract key",
                    ))?,
                    tokens.next().ok_or(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Failed to extract value",
                    ))?,
                )
            };

            match k {
                "byr" => p.byr = Some(String::from(v)),
                "iyr" => p.iyr = Some(String::from(v)),
                "eyr" => p.eyr = Some(String::from(v)),
                "hgt" => p.hgt = Some(String::from(v)),
                "hcl" => p.hcl = Some(String::from(v)),
                "ecl" => p.ecl = Some(String::from(v)),
                "pid" => p.pid = Some(String::from(v)),
                "cid" => p.cid = Some(String::from(v)),
                k @ _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid key: {}", k),
                    ));
                }
            };
        }
        Ok(p)
    }
}

fn challenge1() -> io::Result<usize> {
    let passports: io::Result<Vec<_>> = aoc20::read_input_to_string("day04")
        .split("\n\n")
        .map(|data_raw| Passport::try_from(data_raw))
        .collect();
    Ok(passports?.iter().filter(|p| p.valid1()).count())
}

fn challenge2() -> io::Result<usize> {
    let passports: io::Result<Vec<_>> = aoc20::read_input_to_string("day04")
        .split("\n\n")
        .map(|data_raw| Passport::try_from(data_raw))
        .collect();
    Ok(passports?.iter().filter(|p| p.valid2()).count())
}

fn main() -> io::Result<()> {
    println!("Valid passports challenge1: {}", challenge1()?);
    println!("Valid passports challenge2: {}", challenge2()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_challenge1() -> io::Result<()> {
        assert_eq!(challenge1()?, 182);
        Ok(())
    }

    #[test]
    fn check_challenge2() -> io::Result<()> {
        assert_eq!(challenge2()?, 109);
        Ok(())
    }
}
