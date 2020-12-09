use std::collections::{HashMap, HashSet, VecDeque};
use std::io;

struct Node<'a> {
    col: &'a str,
    cnt: usize,
}

/// Challenge1
///
/// Allow to map inner bags to outer bags which allows for easy
/// upwards travesal of the dependency graph and hence counting how
/// many bags can contain a "shiny gold" bag.
type Inner2Outer<'a> = HashMap<&'a str, HashSet<&'a str>>;

/// Challenge2
///
/// Map outer bags to number of inner bags needed which allows for
/// easy downwards travesal to compute the total number of inner bags.
type Outer2Inner<'a> = HashMap<&'a str, Vec<Node<'a>>>;

fn parse_line<'a>(line: &'a str) -> io::Result<(&'a str, Vec<Node<'a>>)> {
    // Potential improvement, don't compile the regex every time.
    let outer_parser = regex::Regex::new(r"([a-z ]+) bags contain").unwrap();
    let inner_parser = regex::Regex::new(r"(\d+) ([a-z ]+) bags?[,.]?").unwrap();

    let outer = match outer_parser.captures(line) {
        Some(c) => c.get(1).expect("match outer_parser").as_str(),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Malformed input line encountered",
            ))
        }
    };

    let mut inner = Vec::new();
    for cap in inner_parser.captures_iter(line) {
        let col = cap.get(2).expect("match inner_parser").as_str();
        let cnt = cap[1]
            .parse::<usize>()
            .expect("must be a number as specified in inner_parse");
        inner.push(Node { col, cnt });
    }

    Ok((outer, inner))
}

fn challenge1() -> io::Result<usize> {
    let input = aoc20::read_input_to_string("day07");

    // Compute graph of inner bags to outer bags.
    let mut graph = Inner2Outer::new();
    for line in input.lines() {
        let (outer, inner) = parse_line(line)?;
        for Node { col, .. } in inner {
            graph.entry(col).or_insert(HashSet::new()).insert(outer);
        }
    }

    // Current working set of bags to inspect and detect if they can
    // be contained in other bags. Fill the queue initially with our
    // "shiny gold" bag and then start looking which bags can contain
    // our bag. We do this until there are no more bags left to
    // inspect.
    let mut queue = VecDeque::new();
    queue.push_back("shiny gold");

    let mut visited = HashSet::new();
    while let Some(bag) = queue.pop_front() {
        if visited.contains(bag) {
            continue;
        }

        // We only count for bags that can contain the "shiny gold"
        // bag, not our bag itself.
        if bag != "shiny gold" {
            visited.insert(bag);
        }

        if let Some(bags) = graph.get(bag) {
            for bag in bags {
                queue.push_back(bag);
            }
        }
    }

    Ok(visited.len())
}

fn challenge2() -> io::Result<usize> {
    let input = aoc20::read_input_to_string("day07");

    // Compute graph of outer bags to inner bags.
    let mut graph = Outer2Inner::new();
    for line in input.lines() {
        let (outer, inner) = parse_line(line)?;
        graph.insert(outer, inner);
    }

    // Working set of inner bags that need to be packed.
    // We start of with our "shiny gold" one.
    let mut queue = VecDeque::new();
    queue.push_back("shiny gold");

    let mut inner_bags = 0;
    while let Some(bag) = queue.pop_front() {
        for node in &graph[bag] {
            inner_bags += node.cnt;
            for _ in 0..node.cnt {
                queue.push_back(node.col);
            }
        }
    }

    Ok(inner_bags)
}

fn main() -> io::Result<()> {
    println!("{}", challenge1()?);
    println!("{}", challenge2()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_challenge1() -> io::Result<()> {
        assert_eq!(challenge1()?, 268);
        Ok(())
    }

    #[test]
    fn check_challenge2() -> io::Result<()> {
        assert_eq!(challenge2()?, 7867);
        Ok(())
    }
}
