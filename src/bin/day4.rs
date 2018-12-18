use chrono::prelude::*;
use chrono::format;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<Error>> {
    let data = read_to_string("./data/day4.txt")?;
    // let data: Vec<Claim> = data.lines().map(Claim::new).collect();
    // part1(&data);
    // part2(&data);
    Ok(())
}

enum EventType {
    GuardChange,
    Sleep,
    Wake,
}

struct Event {
    time: DateTime<Utc>,
    guard: usize,
    type_: EventType,
}

impl Event {
    fn parse(eventStr: &str) -> Self {
        
        Self {
            time: Utc::now(),
            guard: 0,
            type_: EventType::GuardChange,
        }
    }
}
