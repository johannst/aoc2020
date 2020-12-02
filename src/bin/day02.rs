use aoc20::input_bufreader;
use std::io::BufRead;

use regex::Regex;

struct DBEntry {
    num1: usize,
    num2: usize,
    c: char,
    pw: String,
}

fn parse_input() -> Vec<DBEntry> {
    let mut v = Vec::new();

    // Regex matching the input format.
    let input_re = Regex::new(r"^([0-9]+)-([0-9]+)\s([a-z]):\s([a-z]+)$").unwrap();

    // Extract DB entries based on the regex captures.
    for line in input_bufreader("day02").lines() {
        let line = line.expect("Failed to read input line!");

        match input_re.captures(&line) {
            Some(caps) => v.push(DBEntry {
                num1: caps[1].parse().unwrap(), // should be a number based on the re capture group
                num2: caps[2].parse().unwrap(), // should be a number based on the re capture group
                c: caps[3].chars().next().unwrap(), // should be a single char based on the re capture group
                pw: caps[4].into(),
            }),
            None => {
                // In case we encounter an invalid input line there is
                // not much more to do here besides exiting.
                println!("Input line '{}' has wrong format!", line);
                std::process::exit(1);
            }
        }
    }

    v
}

fn challenge1(entries: &Vec<DBEntry>) -> usize {
    entries
        .iter()
        .filter(|e| {
            let c_cnt = e.pw.chars().filter(|&c| e.c == c).count();
            // For a valid password e.c can not occurs more or less as
            // the specified boundaries in the input.
            c_cnt >= e.num1 && c_cnt <= e.num2
        })
        .count()
}

fn challenge2(entries: &Vec<DBEntry>) -> usize {
    entries
        .iter()
        .filter(|e| {
            assert!(e.num1 > 0 && e.num2 > 0); // As given by the pw policy.
            let char1 =
                e.pw.chars()
                    .nth(e.num1 - 1)
                    .expect("Input contains invalid offsets into password!");
            let char2 =
                e.pw.chars()
                    .nth(e.num2 - 1)
                    .expect("Input contains invalid offsets into password!");
            // For a valid password either char1 or char2 must match
            // but not both -> XOR.
            char1 != char2 && (char1 == e.c || char2 == e.c)
        })
        .count()
}

fn main() {
    let entries = parse_input();

    let valid_passwords = challenge1(&entries);
    println!(
        "Challenge1 input contains '{}' valid passwords.",
        valid_passwords
    );

    let valid_passwords = challenge2(&entries);
    println!(
        "Challenge2 input contains '{}' valid passwords.",
        valid_passwords
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_challenge1() {
        let entries = parse_input();
        assert_eq!(challenge1(&entries), 614);
    }

    #[test]
    fn check_challenge2() {
        let entries = parse_input();
        assert_eq!(challenge2(&entries), 354);
    }
}
