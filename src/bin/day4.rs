use chrono::format;
use chrono::prelude::*;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<Error>> {
    let data = read_to_string("./data/day4.txt")?;
    // let data: Vec<Claim> = data.lines().map(Claim::new).collect();
    // part1(&data);
    // part2(&data);
    Ok(())
}

enum EventKind {
    GuardChange,
    Sleep,
    Wake,
}

struct Event {
    time: DateTime<Utc>,
    guard: usize,
    kind: EventKind,
}

impl Event {
    fn parse(eventStr: &str) -> Self {
        Self {
            time: Utc::now(),
            guard: 0,
            kind: EventKind::GuardChange,
        }
    }
}
