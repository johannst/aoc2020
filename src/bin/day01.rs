use aoc20::read_input;

fn main() {
    let input = read_input("day01");

    let input_num: Vec<_> = input
        .split_whitespace()
        .map(|input| input.parse::<usize>().expect("Input is not a number!"))
        .collect();

    'outer: for (i, n1) in input_num.iter().enumerate() {
        for n2 in input_num.iter().skip(i) {
            if n1 + n2 == 2020 {
                println!("Magic Numbers ({} * {}) = {}", n1, n2, n1 * n2);
                break 'outer;
            }
        }
    }

    'outer2: for (i, n1) in input_num.iter().enumerate() {
        for (j, n2) in input_num.iter().skip(i).enumerate() {
            for n3 in input_num.iter().skip(j) {
                if n1 + n2 + n3 == 2020 {
                    println!("Magic Number ({} * {} * {}) = {}", n1, n2, n3, n1 * n2 * n3);
                    break 'outer2;
                }
            }
        }
    }
}
