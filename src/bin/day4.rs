use chrono::format;
use chrono::prelude::*;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<Error>> {
    let data = read_to_string("./data/day4.txt")?;
    let data: Vec<&str> = data.lines().collect();
    let data = sort_by_time(&data);
    let data = parse_events(data);
    for d in data.iter() {
        println!("{:?}", d);
    }
    // part1(&data);
    // part2(&data);
    Ok(())
}

fn sort_by_time(lines: &[&str]) -> Vec<(DateTime<Utc>, String)> {
    let mut lines_with_time: Vec<(DateTime<Utc>, String)> = lines
        .iter()
        .map(|line| {
            let mut line_iter = line.chars();
            line_iter.next(); // skip [
            let time: String = line_iter.by_ref().take(16).collect();
            line_iter.next(); // skip space
            (time.parse::<DateTime<Utc>>().unwrap(), line_iter.collect())
        })
        .collect();
    lines_with_time.sort_by_key(|pair| pair.0);
    lines_with_time
}

fn parse_events(mut lines: Vec<(DateTime<Utc>, String)>) -> Vec<Event> {
    let mut events: Vec<Event> = Vec::new();
    let mut curr_group: Vec<(DateTime<Utc>, String)> = Vec::new();

    for line in lines.drain(..) {
        if line.1.contains('#') && !curr_group.is_empty() {
            let mut parsed_group = Event::parse_group(&curr_group);
            events.append(&mut parsed_group);
            curr_group = Vec::new();
        }
        curr_group.push(line);
    }
    events
}

#[derive(Debug)]
enum EventKind {
    GuardChange,
    Sleep,
    Wake,
}

#[derive(Debug)]
struct Event {
    time: DateTime<Utc>,
    guard: usize,
    kind: EventKind,
}

impl Event {
    fn parse_group(group: &[(DateTime<Utc>, String)]) -> Vec<Self> {
        let first = &group[0];
        let guard = first
            .1
            .chars()
            .skip_while(|&c| c != '#')
            .skip(1)
            .take_while(|c| c.is_numeric())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        group
            .iter()
            .map(|pair| Self::parse(pair.0, guard, &pair.1))
            .collect()
    }

    fn parse(time: DateTime<Utc>, guard: usize, eventStr: &str) -> Self {
        let kind = if eventStr.contains('#') {
            EventKind::GuardChange
        } else if eventStr.contains("falls asleep") {
            EventKind::Sleep
        } else {
            EventKind::Wake
        };

        Self { time, guard, kind }
    }
}
