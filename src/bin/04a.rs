extern crate regex;
use regex::Regex;
use std::collections::HashMap;

// Pretty printing for sleep schedules to make comparisons against example easier.
fn print_schedule(guard_id: &usize, date: &str, hour: &Vec<usize>) {
    print!("{} {:>4}  ", date, guard_id);
    for i in 0..hour.len() {
        print!("{}", if hour[i] == 1 { "#" } else { "." });
    }
    println!("{}", ".".repeat(60 - hour.len()));
}


fn main() {
    let input = include_str!("../../inputs/04");

    // Read in all data.
    let log_re =
        Regex::new(r"\[\d\d\d\d-(?P<date>\d\d-\d\d) (?P<hour>\d\d):(?P<minute>\d\d)] (?P<text>.+)")
            .unwrap();
    let mut logs: Vec<&str> = input.lines().collect();
    logs.sort();
    let mut current_guard: usize = 0;
    // guard_id -> day -> midnight_hour
    let mut sleeping_patterns: HashMap<usize, HashMap<&str, Vec<usize>>> = HashMap::new();
    // I'm assuming the data is well formed, that is:
    // - no unpaired wake/sleep
    // - no wake before sleep
    // - midnight hour begins and finish awake
    for log_line in logs {
        let caps = log_re.captures(log_line).unwrap();
        let date = caps.name("date").unwrap().as_str();
        let minute: usize = caps.name("minute").unwrap().as_str().parse().unwrap();
        let entry = caps.name("text").unwrap().as_str();
        if entry.ends_with("begins shift") {
            current_guard = entry
                .split_whitespace()
                .nth(1)
                .unwrap()
                .trim_start_matches('#')
                .parse()
                .unwrap();
            continue;
        }
        let dates = &mut sleeping_patterns.entry(current_guard).or_insert(HashMap::new());
        let day = dates.entry(date).or_insert(Vec::new());
        if entry.ends_with("falls asleep") {
            let mut awake_period = vec![0usize; minute - day.len()];
            day.append(&mut awake_period);
            day.push(1);
        }
        if entry.ends_with("wakes up") {
            let mut asleep_period = vec![1usize; minute - day.len()];
            day.append(&mut asleep_period);
            day.push(0);
        }
    }

    for (guard, days) in sleeping_patterns {
        for (date,hour) in days {
            print_schedule(&guard, date, &hour);
        }
    }
}
