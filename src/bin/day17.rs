use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Cube {
    Active,
    Inactive,
}

type Vec3 = (i32, i32, i32);
type PocketDimension = HashMap<Vec3, Cube>;

/// Get iterator over all neighbor coordinates of position `p`.
fn neighbor_coords(p: Vec3) -> impl Iterator<Item = Vec3> {
    (-1..=1)
        .flat_map(|x| std::iter::repeat(x).zip(-1..=1))
        .flat_map(|xy| std::iter::repeat(xy).zip(-1..=1))
        .map(|((x, y), z)| (x, y, z))
        .filter(|&xyz| xyz != (0, 0, 0))
        .map(move |n| (p.0 + n.0, p.1 + n.1, p.2 + n.2))
}

/// Determine number of active cubes (neighbors) of position `p`.
fn active_neighbors(p: Vec3, d: &PocketDimension) -> usize {
    neighbor_coords(p)
        .filter(|p| {
            if let Some(v) = d.get(p) {
                v == &Cube::Active
            } else {
                false
            }
        })
        .count()
}

fn challenge1() -> usize {
    // Current state of pocket dimension.
    let mut dim: PocketDimension = aoc20::read_input_to_string("day17")
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some(((x as i32, y as i32, 0 as i32), Cube::Active)),
                _ => None,
            })
        })
        .collect();

    // Next state of pocket dimension.
    let mut next_dim = HashMap::new();

    // Compute expansion of dimension for 6 cycles.
    for _ in 0..6 {
        for (&p, cube) in &dim {
            // Expand dimension by computing states of cubes at the
            // boundary (not yet in map) of the current dimension state.
            // Cubes at the boundary are by definition `inactive` and
            // hence only become active if they have exactly 3 active
            // neighbor cubes.
            neighbor_coords(p)
                .filter(|p| !dim.contains_key(p))
                .for_each(|p| {
                    next_dim.insert(
                        p,
                        if active_neighbors(p, &dim) == 3 {
                            Cube::Active
                        } else {
                            Cube::Inactive
                        },
                    );
                });

            let active_neigh = active_neighbors(p, &dim);
            // Compute state of cube at position `p`.
            next_dim.insert(
                p,
                match cube {
                    // If a cube is active and exactly 2 or 3 of its
                    // neighbors are also active, the cube remains active ...
                    Cube::Active if [2, 3].contains(&active_neigh) => Cube::Active,
                    // If a cube is inactive but exactly 3 of its
                    // neighbors are active, the cube becomes active ...
                    Cube::Inactive if active_neigh == 3 => Cube::Active,
                    // ... otherwise, the cube becomes/remains inactive.
                    Cube::Active | Cube::Inactive => Cube::Inactive,
                },
            );
        }

        // Move to next cycle, taking next state as current one.
        std::mem::swap(&mut dim, &mut next_dim);
    }

    dim.iter()
        .filter(|&(_, cube)| cube == &Cube::Active)
        .count()
}

type Vec4 = (i32, i32, i32, i32);
type PocketDimension4 = HashMap<Vec4, Cube>;

/// Get iterator over all neighbor coordinates of position `p`.
fn neighbor_coords4(p: Vec4) -> impl Iterator<Item = Vec4> {
    (-1..=1)
        .flat_map(|x| std::iter::repeat(x).zip(-1..=1))
        .flat_map(|xy| std::iter::repeat(xy).zip(-1..=1))
        .flat_map(|xyz| std::iter::repeat(xyz).zip(-1..=1))
        .map(|(((x, y), z), w)| (x, y, z, w))
        .filter(|&xyz| xyz != (0, 0, 0, 0))
        .map(move |n| (p.0 + n.0, p.1 + n.1, p.2 + n.2, p.3 + n.3))
}

/// Determine number of active cubes (neighbors) of position `p`.
fn active_neighbors4(p: Vec4, d: &PocketDimension4) -> usize {
    neighbor_coords4(p)
        .filter(|p| {
            if let Some(v) = d.get(p) {
                v == &Cube::Active
            } else {
                false
            }
        })
        .count()
}

fn challenge2() -> usize {
    // Current state of pocket dimension.
    let mut dim: PocketDimension4 = aoc20::read_input_to_string("day17")
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some(((x as i32, y as i32, 0 as i32, 0 as i32), Cube::Active)),
                _ => None,
            })
        })
        .collect();

    // Next state of pocket dimension.
    let mut next_dim = HashMap::new();

    // Compute expansion of dimension for 6 cycles.
    for _ in 0..6 {
        for (&p, cube) in &dim {
            // Expand dimension by computing states of cubes at the
            // boundary (not yet in map) of the current dimension state.
            // Cubes at the boundary are by definition `inactive` and
            // hence only become active if they have exactly 3 active
            // neighbor cubes.
            neighbor_coords4(p)
                .filter(|p| !dim.contains_key(p))
                .for_each(|p| {
                    next_dim.insert(
                        p,
                        if active_neighbors4(p, &dim) == 3 {
                            Cube::Active
                        } else {
                            Cube::Inactive
                        },
                    );
                });

            let active_neigh = active_neighbors4(p, &dim);
            // Compute state of cube at position `p`.
            next_dim.insert(
                p,
                match cube {
                    // If a cube is active and exactly 2 or 3 of its
                    // neighbors are also active, the cube remains active ...
                    Cube::Active if [2, 3].contains(&active_neigh) => Cube::Active,
                    // If a cube is inactive but exactly 3 of its
                    // neighbors are active, the cube becomes active ...
                    Cube::Inactive if active_neigh == 3 => Cube::Active,
                    // ... otherwise, the cube becomes/remains inactive.
                    Cube::Active | Cube::Inactive => Cube::Inactive,
                },
            );
        }

        // Move to next cycle, taking next state as current one.
        std::mem::swap(&mut dim, &mut next_dim);
    }

    dim.iter()
        .filter(|&(_, cube)| cube == &Cube::Active)
        .count()
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
        assert_eq!(challenge1(), 336);
    }

    #[test]
    fn check_challenge2() {
        assert_eq!(challenge2(), 2620);
    }
}
