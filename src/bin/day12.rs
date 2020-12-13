#[derive(Default, PartialEq, Debug)]
struct Vec2(i32, i32);

impl Vec2 {
    fn rot_right90(&mut self) {
        // Apply right rotation matrix.
        // |  0 1 |
        // | -1 0 |
        *self = Vec2(self.1, -self.0);
    }

    fn rot_left90(&mut self) {
        // Apply left rotation matrix.
        // | 0 -1 |
        // | 1  0 |
        *self = Vec2(-self.1, self.0);
    }
}

struct Ferry {
    face: Vec2,
    pos: Vec2,
}

impl Default for Ferry {
    fn default() -> Ferry {
        Ferry {
            // Start off facing east.
            face: Vec2(1, 0),
            pos: Vec2(0, 0),
        }
    }
}

fn challenge1() -> i32 {
    let ferry = aoc20::read_input_to_string("day12")
        .lines()
        .map(|l| {
            let (a, n) = l.split_at(1);
            (a, n.parse::<i32>().unwrap())
        })
        .fold(Ferry::default(), |mut ferry, (action, val)| {
            match action {
                "N" => ferry.pos.1 += val,
                "S" => ferry.pos.1 -= val,
                "E" => ferry.pos.0 += val,
                "W" => ferry.pos.0 -= val,
                "L" => {
                    assert!(val % 90 == 0);
                    for _ in 0..val / 90 {
                        ferry.face.rot_left90();
                    }
                }
                "R" => {
                    assert!(val % 90 == 0);
                    for _ in 0..val / 90 {
                        ferry.face.rot_right90();
                    }
                }
                "F" => {
                    ferry.pos.0 += ferry.face.0 * val;
                    ferry.pos.1 += ferry.face.1 * val;
                }
                _ => unreachable!(),
            }

            ferry
        });

    ferry.pos.0.abs() + ferry.pos.1.abs()
}

fn challenge2() -> i32 {
    let (ferry, _) = aoc20::read_input_to_string("day12")
        .lines()
        .map(|l| {
            let (a, n) = l.split_at(1);
            (a, n.parse::<i32>().unwrap())
        })
        .fold(
            (
                Ferry::default(),
                Vec2(10, 1), /*waypoint starts 10 units east and 1 unit north relative to the ship*/
            ),
            |(mut ferry, mut waypoint), (action, val)| {
                match action {
                    "N" => waypoint.1 += val,
                    "S" => waypoint.1 -= val,
                    "E" => waypoint.0 += val,
                    "W" => waypoint.0 -= val,
                    "L" => {
                        assert!(val % 90 == 0);
                        for _ in 0..val / 90 {
                            waypoint.rot_left90();
                        }
                    }
                    "R" => {
                        assert!(val % 90 == 0);
                        for _ in 0..val / 90 {
                            waypoint.rot_right90();
                        }
                    }
                    "F" => {
                        ferry.pos.0 += waypoint.0 * val;
                        ferry.pos.1 += waypoint.1 * val;
                    }
                    _ => unreachable!(),
                }

                (ferry, waypoint)
            },
        );

    ferry.pos.0.abs() + ferry.pos.1.abs()
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
        assert_eq!(challenge1(), 636);
    }

    #[test]
    fn check_challenge2() {
        assert_eq!(challenge2(), 26841);
    }

    #[test]
    fn vec2_rot_left90() {
        let mut v = Vec2(2, 1);
        v.rot_left90();
        assert_eq!(v, Vec2(-1, 2));
        v.rot_left90();
        assert_eq!(v, Vec2(-2, -1));
        v.rot_left90();
        assert_eq!(v, Vec2(1, -2));
        v.rot_left90();
        assert_eq!(v, Vec2(2, 1));
    }

    #[test]
    fn vec2_rot_right90() {
        let mut v = Vec2(2, 1);
        v.rot_right90();
        assert_eq!(v, Vec2(1, -2));
        v.rot_right90();
        assert_eq!(v, Vec2(-2, -1));
        v.rot_right90();
        assert_eq!(v, Vec2(-1, 2));
        v.rot_right90();
        assert_eq!(v, Vec2(2, 1));
    }
}
