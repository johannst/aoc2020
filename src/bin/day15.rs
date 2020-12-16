use std::collections::HashMap;

struct Game {
    round: u32,
    history: HashMap<u32, u32>,
    last: u32,
}

impl Game {
    fn new(start: &[u32]) -> Game {
        // Insert all except last, last will be inserted when stepping.
        let mut history = HashMap::new();
        for (r, &n) in start.iter().take(start.len() - 1).enumerate() {
            assert!(!history.contains_key(&n));
            history.insert(n, r as u32 + 1 /* start at round 1 */);
        }

        Game {
            round: start.len() as u32,
            history,
            last: *start.last().expect("Must have some input"),
        }
    }

    fn step(&mut self) -> u32 {
        // Check if `self.last` was already spoken and compute distance else speak `0`.
        let new_last = if let Some(last_occured) = self.history.get(&self.last) {
            self.round - last_occured
        } else {
            0
        };

        // Insert previous last into history.
        self.history.insert(self.last, self.round);

        // Advance last spoken.
        self.last = new_last;

        // Advance next round.
        self.round += 1;

        self.last
    }

    fn round(&self) -> u32 {
        self.round
    }
}

fn challenge1(input: &[u32]) -> u32 {
    let mut g = Game::new(input);

    // Step until one before round 2020.
    for _ in g.round()..2020 - 1 {
        g.step();
    }

    g.step()
}

fn challenge2(input: &[u32]) -> u32 {
    let mut g = Game::new(input);

    // Step until one before round 30000000.
    for _ in g.round()..30000000 - 1 {
        g.step();
    }

    g.step()
}

fn main() {
    println!("{}", challenge1(&[1, 0, 15, 2, 10, 13]));
    println!("{}", challenge2(&[1, 0, 15, 2, 10, 13]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_challenge1() {
        assert_eq!(challenge1(&[1, 0, 15, 2, 10, 13]), 211);
    }

    #[test]
    fn check_challenge2() {
        assert_eq!(challenge2(&[1, 0, 15, 2, 10, 13]), 2159626);
    }
}
