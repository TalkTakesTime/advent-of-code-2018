use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<Error>> {
    let data = read_to_string("./data/day2.txt")?;
    let data: Vec<&str> = data.lines().collect();
    part1(&data);
    part2(&data);
    Ok(())
}

fn part1(ids: &[&str]) {
    let counts = ids.iter().map(|&s| count_repeated_letters(s)).fold(
        (0, 0),
        // this is inelegant
        |acc, (has2, has3)| {
            (
                acc.0 + (if has2 { 1 } else { 0 }),
                acc.1 + (if has3 { 1 } else { 0 }),
            )
        },
    );
    println!("Part 1: {}", counts.0 * counts.1);
}

fn part2(ids: &[&str]) {
    let letters = ids.iter().find_map(|&id| find_matching_id_letters(id, ids));
    if let Some(s) = letters {
        println!("Part 2: {}", s);
    } else {
        println!("Part 2: Nothing found!");
    }
}

fn count_repeated_letters(id: &str) -> (bool, bool) {
    let mut counts: HashMap<char, isize> = HashMap::new();
    for c in id.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    (
        counts.values().any(|&v| v == 2),
        counts.values().any(|&v| v == 3),
    )
}

fn find_matching_id_letters(id: &str, ids: &[&str]) -> Option<String> {
    let match_len = id.len() - 1;
    ids.iter()
        .map(|&id2| {
            id.chars()
                .zip(id2.chars())
                .filter(|(c1, c2)| c1 == c2)
                .map(|(c1, _)| c1)
                .collect::<String>()
        })
        .find(|matched| matched.len() == match_len)
}
