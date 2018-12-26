use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<Error>> {
    let data = read_to_string("./data/day5.txt")?;
    let data = data.trim();
    part1(&data);
    part2(&data);
    Ok(())
}

fn part1(polymer: &str) {
    let reacted = react(polymer);
    println!("Part 1: {}", reacted.len());
}

fn part2(polymer: &str) {
    let unique_types = get_all_unit_types(polymer);
    let shortest = unique_types
        .iter()
        .map(|&t| {
            let reduced_polymer: String = polymer
                .chars()
                .filter(|c| c.to_ascii_lowercase() != t)
                .collect();
            react(&reduced_polymer).len()
        })
        .min();
    println!("Part 2: {:?}", shortest);
}

fn get_all_unit_types(polymer: &str) -> HashSet<char> {
    polymer.chars().map(|c| c.to_ascii_lowercase()).collect()
}

fn react(polymer: &str) -> String {
    let mut units: Vec<char> = polymer.chars().collect();
    let mut first = 0;
    let mut second = 1;

    while second < units.len() {
        if try_react(&units, first, second) == ReactionResult::NoReaction {
            first += 1;
            second += 1;
        } else {
            units.remove(second);
            units.remove(first);
            if first != 0 {
                first -= 1;
                second -= 1;
            }
        }
    }

    units.iter().collect()
}

#[derive(PartialEq)]
enum ReactionResult {
    NoReaction,
    Reacted,
}

fn try_react(units: &[char], first: usize, second: usize) -> ReactionResult {
    let unit1 = units[first];
    let unit2 = units[second];
    if unit1 != unit2 && unit1.to_ascii_lowercase() == unit2.to_ascii_lowercase() {
        ReactionResult::Reacted
    } else {
        ReactionResult::NoReaction
    }
}
