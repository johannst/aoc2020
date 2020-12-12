use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
enum Seat {
    E,
    O,
}

const ADJACENT: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

type WaitingArea = HashMap<(i32, i32), Seat>;

fn challenge1() -> usize {
    // Count number of occupied seats directly around position `p` in
    // the waiting area `a`.
    let cnt_adj = |p: (i32, i32), a: &WaitingArea| {
        ADJACENT
            .iter()
            .filter(|&(x, y)| {
                if let Some(s) = a.get(&(p.0 + x, p.1 + y)) {
                    s == &Seat::O
                } else {
                    false
                }
            })
            .count()
    };

    let (mut curr_area, _) = parse_input();
    let mut next_area = WaitingArea::new();

    while curr_area != next_area {
        // Apply seat rules to current waiting area state.
        for (p, s) in &curr_area {
            let adj_occ = cnt_adj(*p, &curr_area);
            next_area.insert(
                *p,
                match s {
                    // If a seat is empty (L) and there are no
                    // occupied seats adjacent to it, the seat becomes
                    // occupied.
                    Seat::E if adj_occ == 0 => Seat::O,
                    // If a seat is occupied (#) and four or more
                    // seats adjacent to it are also occupied, the
                    // seat becomes empty.
                    Seat::O if adj_occ >= 4 => Seat::E,
                    // Otherwise, the seat's state does not change.
                    v => *v,
                },
            );
        }

        // Now move to newly computed area state.
        std::mem::swap(&mut curr_area, &mut next_area);
    }

    // Count number of occupied seats after reaching a fix-point.
    curr_area.iter().filter(|(_, &v)| v == Seat::O).count()
}

fn challenge2() -> usize {
    let (mut curr_area, (rows, cols)) = parse_input();

    // Count number of occupied seats around position `p` in the
    // waiting area `a`. Seats must not be directly adjacent, we look
    // for the first seat in the given direction.
    let cnt_adj = |p: (i32, i32), a: &WaitingArea| {
        ADJACENT
            .iter()
            .filter(|&(x, y)| {
                let mut pos = p;
                loop {
                    pos.0 += x;
                    pos.1 += y;

                    if let Some(s) = a.get(&pos) {
                        return s == &Seat::O;
                    } else if pos.0 < 0 || pos.0 > cols {
                        return false;
                    } else if pos.1 < 0 || pos.1 > rows {
                        return false;
                    }
                }
            })
            .count()
    };

    let mut next_area = WaitingArea::new();

    while curr_area != next_area {
        // Apply seat rules to current waiting area state.
        for (p, s) in &curr_area {
            let adj_occ = cnt_adj(*p, &curr_area);
            next_area.insert(
                *p,
                match s {
                    // If a seat is empty (L) and there are no
                    // occupied seats adjacent to it, the seat becomes
                    // occupied.
                    Seat::E if adj_occ == 0 => Seat::O,
                    // If a seat is occupied (#) and five or more
                    // seats adjacent to it are also occupied, the
                    // seat becomes empty.
                    Seat::O if adj_occ >= 5 => Seat::E,
                    // Otherwise, the seat's state does not change.
                    v => *v,
                },
            );
        }

        // Now move to newly computed area state.
        std::mem::swap(&mut curr_area, &mut next_area);
    }

    // Count number of occupied seats after reaching a fix-point.
    curr_area.iter().filter(|(_, &v)| v == Seat::O).count()
}

/// Parse waiting area seat map.
///
/// We only insert seat positions and skip floor positions because
/// they are not of any importantnce here.
fn parse_input() -> (WaitingArea, (i32, i32)) {
    let a: WaitingArea = aoc20::read_input_to_string("day11")
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == 'L')
                .map(move |(x, _)| ((x as i32, y as i32), Seat::E))
        })
        .collect();

    let rows = a.keys().map(|k| k.0).max().expect("Waiting area is not empty");
    let cols = a.keys().map(|k| k.1).max().expect("Waiting area is not empty");

    (a, (rows, cols))
}

fn main() {
    println!("{}", challenge1());
    println!("{}", challenge2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_challenge1() {
        assert_eq!(challenge1(), 2265);
    }

    #[test]
    fn check_challenge2() {
        assert_eq!(challenge2(), 2045);
    }
}
