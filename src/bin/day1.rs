use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<Error>> {
    let data = read_to_string("./data/day1.txt")?;
    let data: Vec<i64> = data
        .trim()
        .split('\n')
        .map(|line| line.trim_start_matches('+').parse::<i64>().unwrap_or(0i64))
        .collect();
    part1(&data);
    part2(&data);
    Ok(())
}

fn part1(data: &[i64]) {
    let result: i64 = data.iter().sum();
    println!("Part 1: {}", result);
}

fn part2(data: &[i64]) {
    let mut freqs: HashSet<i64> = HashSet::new();
    let nums = data.iter().cycle();

    let mut total = 0i64;
    for n in nums {
        total += n;
        if !freqs.insert(total) {
            break;
        }
    }

    println!("Part 2: {}", total);
}
