use std::collections::HashMap;
use std::io;

fn challenge1(adapters: &Vec<usize>) -> usize {
    let (cd1, cd3) = adapters
        .windows(2)
        .fold((0, 0), |(cd1, cd3), w| match w[1] - w[0] {
            1 => (cd1 + 1, cd3),
            2 => (cd1, cd3),
            3 => (cd1, cd3 + 1),
            _ => unreachable!(),
        });
    cd1 * cd3
}

fn challenge2(adapters: &Vec<usize>) -> usize {
    let mut variants: HashMap<usize, usize> = HashMap::new();

    // Start with single variant for adapter `0`.
    variants.insert(0, 1); // initial coin

    for &adapter in &adapters[1..] {
        // For each adatper sum up the variants of the previous adapters in range [-3, 0).
        let adapter_variants = variants.get(&adapter.wrapping_sub(1)).unwrap_or(&0)
            + variants.get(&adapter.wrapping_sub(2)).unwrap_or(&0)
            + variants.get(&adapter.wrapping_sub(3)).unwrap_or(&0);
        variants.insert(adapter, adapter_variants);
    }
    variants[adapters.last().expect("Definetly have some adapters")]
}

fn parse_adapters() -> io::Result<Vec<usize>> {
    let mut v = aoc20::read_input_to_string("day10")
        .lines()
        .map(|l| {
            l.parse()
                .map_err(|_| io::Error::new(io::ErrorKind::Other, ""))
        })
        .collect::<io::Result<Vec<usize>>>()?;

    // Add chargin outlet (0 jolts).
    v.push(0);

    // Build adapter chain.
    v.sort();

    // Add devices built-in adapter (last adapter + 3).
    v.push(v.last().unwrap() + 3);

    Ok(v)
}

fn main() -> io::Result<()> {
    let adapters = parse_adapters()?;
    println!("{}", challenge1(&adapters));
    println!("{}", challenge2(&adapters));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_challenge1() -> io::Result<()> {
        let a = parse_adapters()?;
        assert_eq!(challenge1(&a), 2176);
        Ok(())
    }

    #[test]
    fn check_challenge2() -> io::Result<()> {
        let a = parse_adapters()?;
        assert_eq!(challenge2(&a), 18512297918464);
        Ok(())
    }
}
