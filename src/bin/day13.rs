fn challenge1() -> usize {
    // Parse input into our own arrival time at the bus station and all
    // the IDs of the buses currently in service.
    let (arrival_time, bus_ids) = {
        let input = aoc20::read_input_to_string("day13");

        let arrival_time: usize = input
            .lines()
            .nth(0)
            .expect("Input must have 2 lines")
            .parse()
            .expect("First line must contain a number");

        // Parse out ids of buses currently in service.
        let ids = input
            .lines()
            .nth(1)
            .expect("Input must have 2 lines")
            .split(',')
            .filter_map(|id| id.parse().ok())
            .collect::<Vec<usize>>();

        (arrival_time, ids)
    };

    let (time_to_depart, bus_id) = bus_ids
        .iter()
        // Compute distance of next depature time of bus `id` from our arrival time.
        .map(|id| match arrival_time % id {
            rem if rem == 0 => (0, id),
            rem => (id - rem, id),
        })
        .min_by_key(|&(time_to_depart, _)| time_to_depart)
        .expect("There is at least one bus");

    time_to_depart * bus_id
}

fn challenge2() -> usize {
    aoc20::read_input_to_string("day13")
        .lines()
        .nth(1)
        .expect("Input must have 2 lines")
        .split(',')
        .enumerate()
        .filter_map(|(offset, id)| match id.parse::<usize>() {
            Ok(id) => Some((offset, id)),
            Err(_) => None,
        })
        .fold(
            (0 /* initial time */, 1 /* initial lcm */),
            |(mut time, lcm), (offset, id)| {
                // For the current bus `id` advance time by the LCM of
                // the already computed busses until we find a time
                // where the offset requirement holds for the current
                // bus.
                // Stepping by the LCM ensures that the requirement
                // still holds for the already computed busses.
                while (time + offset) % id != 0 {
                    time += lcm;
                }
                // Compute new LCM based on the buses that already
                // full-fill the offset requirement.
                (time, lcm * id)
            },
        )
        .0
}

fn main() {
    println!("{}", challenge1());
    println!("{}", challenge2());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_challenge1() {
        assert_eq!(challenge1(), 370);
    }

    #[test]
    fn check_challenge2() {
        assert_eq!(challenge2(), 894954360381385);
    }
}
