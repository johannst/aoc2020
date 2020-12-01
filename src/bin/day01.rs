use aoc20::read_input;

fn parse_input() -> Vec<usize> {
    read_input("day01")
        .split_whitespace()
        .map(|input| input.parse::<usize>().expect("Input is not a number!"))
        .collect()
}

fn challenge1(input: &Vec<usize>) -> Option<usize> {
    for (i, n1) in input.iter().enumerate() {
        for n2 in input.iter().skip(i) {
            if n1 + n2 == 2020 {
                println!("Magic Number ({} * {}) = {}", n1, n2, n1 * n2);
                return Some(n1 * n2);
            }
        }
    }
    None
}

fn challenge2(input: &Vec<usize>) -> Option<usize> {
    for (i, n1) in input.iter().enumerate() {
        for (j, n2) in input.iter().skip(i).enumerate() {
            for n3 in input.iter().skip(j) {
                if n1 + n2 + n3 == 2020 {
                    println!("Magic Number ({} * {} * {}) = {}", n1, n2, n3, n1 * n2 * n3);
                    return Some(n1 * n2 * n3);
                }
            }
        }
    }
    None
}

fn main() {
    let input = parse_input();
    challenge1(&input);
    challenge2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_challange1() {
        let result = challenge1(&parse_input());
        assert_eq!(result, Some(876459));
    }

    #[test]
    fn check_challange2() {
        let result = challenge2(&parse_input());
        assert_eq!(result, Some(116168640));
    }
}
