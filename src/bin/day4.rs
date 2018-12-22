use chrono::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<Error>> {
    let data = read_to_string("./data/day4.txt")?;
    let data: Vec<&str> = data.lines().collect();
    let data = sort_lines_by_time(&data);
    let events = parse_events(data);
    let hours_by_guard = count_sleeps(&events);

    part1(&hours_by_guard);
    part2(&hours_by_guard);
    Ok(())
}

fn part1(hours_by_guard: &HashMap<usize, HashMap<u32, usize>>) {
    let (guard, hour) = hours_by_guard
        .iter()
        .max_by_key(|&(_, hour)| hour.values().sum::<usize>())
        .unwrap();
    println!(
        "Part 1: {}",
        guard * (*hour.iter().max_by_key(|&(_, count)| count).unwrap().0 as usize)
    );
}

fn part2(hours_by_guard: &HashMap<usize, HashMap<u32, usize>>) {
    let (guard, hour) = hours_by_guard
        .iter()
        .max_by_key(|&(_, hour)| hour.values().max())
        .unwrap();
    println!(
        "Part 2: {}",
        guard * (*hour.iter().max_by_key(|&(_, count)| count).unwrap().0 as usize)
    );
}

fn count_sleeps(events: &[Event]) -> HashMap<usize, HashMap<u32, usize>> {
    let mut hours_by_guard: HashMap<usize, HashMap<u32, usize>> = HashMap::new();
    let (sleeps, wakes): (Vec<Event>, Vec<Event>) = events
        .iter()
        .filter(|e| e.kind != EventKind::GuardChange)
        .partition(|e| e.kind == EventKind::Sleep);

    for (sleep, wake) in sleeps.iter().zip(wakes.iter()) {
        let guard = hours_by_guard.entry(sleep.guard).or_insert(HashMap::new());
        for min in sleep.time.minute()..wake.time.minute() {
            *guard.entry(min).or_insert(0) += 1;
        }
    }
    hours_by_guard
}

fn sort_lines_by_time(lines: &[&str]) -> Vec<(DateTime<Utc>, String)> {
    let mut lines_with_time: Vec<(DateTime<Utc>, String)> = lines
        .iter()
        .map(|line| {
            let mut line_iter = line.chars();
            line_iter.next(); // skip [
            let time: String = line_iter.by_ref().take(16).collect();
            line_iter.next(); // skip ]
            (
                Utc.datetime_from_str(&time, "%Y-%m-%d %H:%M").unwrap(),
                line_iter.collect(),
            )
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

#[derive(Debug, PartialEq, Copy, Clone)]
enum EventKind {
    GuardChange,
    Sleep,
    Wake,
}

#[derive(Debug, Copy, Clone)]
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

    fn parse(time: DateTime<Utc>, guard: usize, event_str: &str) -> Self {
        let kind = if event_str.contains('#') {
            EventKind::GuardChange
        } else if event_str.contains("falls asleep") {
            EventKind::Sleep
        } else {
            EventKind::Wake
        };

        Self { time, guard, kind }
    }
}
