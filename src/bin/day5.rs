use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<Error>> {
    let data = read_to_string("./data/day5.txt")?;
    // part1(&data);
    // part2(&data);
    Ok(())
}

fn reduce(polymer: &str) -> String {
    let mut units: Vec<char> = Vec::new();
    let mut unit_iter = polymer.chars().peekable();
    let mut last_unit = unit_iter.next().unwrap();

    while let Some(curr_unit) = unit_iter.next() {
        if last_unit == curr_unit
            || last_unit.to_ascii_lowercase() != curr_unit.to_ascii_lowercase()
        {
            units.push(last_unit);
        } else {
            last_unit = curr_unit;
            unit_iter.next();
        }
    }

    units.iter().collect()
}
