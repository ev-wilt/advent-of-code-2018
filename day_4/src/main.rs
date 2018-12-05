extern crate chrono;

use std::io::{self, BufRead};
use std::collections::HashMap;
use chrono::{NaiveDate, NaiveDateTime, Timelike};

fn part_one() {
    let stdin = io::stdin();
    let mut input = Vec::new();

    // Put input into vector
    for line in stdin.lock().lines() {
        input.push(line.unwrap());
    }

    input.sort_unstable();
    // Separate events and dates/time into vectors
    let mut dates = Vec::new();
    let mut events = Vec::new();

    for mut line in input {
        let split_index = line.find(']').unwrap() + 2;
        let event = line.split_off(split_index);
        let date = NaiveDateTime::parse_from_str(&line, "[%Y-%m-%d %H:%M] ").unwrap();
        dates.push(date);
        events.push(event);
    }

    // Iterate through dates/events
    let mut sleep_times: HashMap<String, Vec<(u32,u32)>> = HashMap::new();
    let mut sleep_start: u32 = 0;
    let mut current_id = "".to_string();
    let mut ids: Vec<String> = Vec::new();
    for set in dates.iter().zip(events.iter()) {
        let (date, event) = set;
        if event == "falls asleep" {
            sleep_start = date.minute();
        }
        else if event == "wakes up" {
            if let Some(current_times) = sleep_times.get_mut(&current_id) {
                current_times.push((sleep_start, date.minute() - 1));
            }
        }
        else {
            if !ids.contains(&event.split_whitespace().nth(1).unwrap().to_string()) {
                ids.push(event.split_whitespace().nth(1).unwrap().to_string());
            }
            current_id = event.split_whitespace().nth(1).unwrap().to_string();
            sleep_times.insert(event.split_whitespace().nth(1).unwrap().to_string(), Vec::new());
        }
    }

    let mut max_total_sleep = 0;
    let mut max_sleep_id = "".to_string();
    let mut max_minute_asleep = 0;

    for id in ids {
        let time_ranges = sleep_times.get(&id).unwrap();
        let mut sleep_sum = 0;
        for range in time_ranges {
            let (start, end) = range;
            sleep_sum += end - start;
        }
        if sleep_sum > max_total_sleep {
            max_total_sleep = sleep_sum;
            max_sleep_id = id;
            for minute in 0..60 {
                let mut times_asleep = 0;
                for range in time_ranges {
                    let (start, end) = range;
                    if *start <= minute && minute <= *end {
                        times_asleep += 1;
                    }
                }
                if times_asleep > max_minute_asleep {
                    max_minute_asleep = minute;
                }
            }
        }
    }
    println!("{}", max_sleep_id);
    println!("{}", max_minute_asleep);
}

fn main() {
    part_one();
}