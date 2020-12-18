use std::collections::HashMap;
use std::str::FromStr;

/// Check if any rule matches `val` and hence it is a valid value.
fn valid(val: u32, rules: &Vec<Rule>) -> bool {
    rules
        .iter()
        .flat_map(|rule| rule.1.iter())
        .any(|&Range(start, end)| start <= val && val <= end)
}

fn challenge1(input: &Input) -> u32 {
    input
        .nearby_tickets
        .iter()
        .flatten()
        .filter(|&&v| !valid(v, &input.rules))
        .sum()
}

/// Collect the index of all rules that are valid for a given value `val`.
fn valid_rules(val: u32, rules: &Vec<Rule>) -> Vec<usize> {
    rules
        .iter()
        .enumerate()
        .filter(|(_, rule)| {
            rule.1
                .iter()
                .any(|&Range(start, end)| start <= val && val <= end)
        })
        .map(|(idx, _)| idx)
        .collect()
}

fn challenge2(input: &Input) -> usize {
    // Filter out invalid tickets.
    let valid_tickets: Vec<&Ticket> = input
        .nearby_tickets
        .iter()
        .filter(|t| t.iter().all(|&v| valid(v, &input.rules)))
        .collect();

    // Collect all rules that are valid for each field in each ticket.
    //
    // v[0]    <- valid rules for all fields of `ticket 0`
    // v[0][0] <- valid rules for for `field 0` of `ticket 0`
    let mut valid_rules_per_ticket_field: Vec<Vec<Vec<usize>>> = valid_tickets
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .map(|&val| valid_rules(val, &input.rules))
                .collect()
        })
        .collect();

    // Compute the intersection of all valid rules for ticket fields with the
    // same index.
    //
    // Eg: For ticket 0 `field 0` compute the intersection of valid rules with
    //     all other tickets N `field 0` where N is [1..].
    //
    // Store results for each field in `intersections` in the form `(field idx,
    // valid rule indices)`.
    let mut intersections = {
        let (first, other) = valid_rules_per_ticket_field.split_first_mut().unwrap();
        // Walk over all fields and compute the intersections.
        for i in 0..first.len() {
            for other in &*other {
                assert_eq!(first.len(), other.len(), "All tickets same number fields");
                first[i].retain(|v| other[i].contains(v));
            }
        }
        first
            .iter()
            .cloned()
            .enumerate()
            .collect::<Vec<(usize, Vec<usize>)>>()
    };

    // Sort by number of valid rules (ascending).
    //
    // We assume the result is not ambiguous and hence there must be at least
    // one intersection with len == 1.
    intersections.sort_by_key(|(_, rules)| rules.len());

    // Store calculated mapping of `rule` index to ticket `field` index.
    let mut rule_to_field = HashMap::new();

    // Iterate over sorted intersections and save `rule idx` -> `field idx`.
    //
    // When reducing an intersection to a single rule remove already mapped
    // rule indices.
    for (field_id, rules) in &mut intersections {
        // Remove already mapped rules.
        rules.retain(|v| !rule_to_field.contains_key(v));

        assert_eq!(rules.len(), 1, "Rules ambiguous!");
        rule_to_field.insert(&rules[0], *field_id);
    }

    // Multiply fields of my ticket that start with `departure`.
    input
        .rules
        .iter()
        .enumerate()
        .filter(|(_, rule)| rule.0.starts_with("departure"))
        .map(|(rule_idx, _)| {
            let field_idx = rule_to_field[&rule_idx];
            input.my_ticket[field_idx] as usize
        })
        .product()
}

#[derive(Debug)]
struct Range(u32, u32);

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = |i| -> Result<_, Self::Err> {
            Ok(s.trim()
                .split('-')
                .nth(i)
                .ok_or("Parse Range: Invalid input")?
                .parse()
                .map_err(|_| "Parse Range: Expected number")?)
        };

        Ok(Range(num(0)?, num(1)?))
    }
}

#[derive(Debug)]
struct Rule(String, Vec<Range>);

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(':');
        match (iter.next(), iter.next()) {
            (Some(name), Some(ranges)) => Ok(Rule(
                name.chars().collect::<String>(),
                ranges
                    .split("or")
                    .map(|r| r.parse::<Range>())
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            _ => Err(String::from("Parse Rule: Invalid input")),
        }
    }
}

type Ticket = Vec<u32>;

struct Input {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn parse_input() -> Result<Input, String> {
    let input = aoc20::read_input_to_string("day16");

    let rules = input
        .lines()
        .take_while(|&l| !l.is_empty())
        .map(|l| l.parse::<Rule>())
        .collect::<Result<Vec<_>, _>>()?;

    let my_ticket = input
        .lines()
        .skip_while(|&l| !l.starts_with("your ticket:"))
        .skip(1)
        .take(1)
        .flat_map(|l| {
            l.split(',').map(|n| {
                n.parse::<u32>()
                    .map_err(|_| "My ticket: encountered entry which is not a number")
            })
        })
        .collect::<Result<Ticket, _>>()?;

    let nearby_tickets = input
        .lines()
        .skip_while(|&l| !l.starts_with("nearby tickets:"))
        .skip(1)
        .map(|l| {
            l.split(',')
                .map(|n| {
                    n.parse::<u32>()
                        .map_err(|_| "My ticket: encountered entry which is not a number")
                })
                .collect::<Result<Ticket, _>>()
        })
        .collect::<Result<Vec<Ticket>, _>>()?;

    Ok(Input {
        rules,
        my_ticket,
        nearby_tickets,
    })
}

fn main() -> Result<(), String> {
    let input = parse_input()?;
    println!("{}", challenge1(&input));
    println!("{}", challenge2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_challenge1() -> Result<(), String> {
        assert_eq!(challenge1(&parse_input()?), 27911);
        Ok(())
    }

    #[test]
    fn check_challenge2() -> Result<(), String> {
        assert_eq!(challenge2(&parse_input()?), 737176602479);
        Ok(())
    }
}
