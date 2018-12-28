use aoc::*;
use regex::Regex;
use std::collections::HashMap;

struct Schedule<'a> {
    // A structure mapping from a guard to his list of sleep schedules per day.
    //                         guard_id->     day->    sleep_schedule
    sleeping_patterns: HashMap<usize, HashMap<&'a str, Vec<usize>>>,
}

fn parse_input(input: &str) -> Schedule {
    // Read in all data.
    let log_re =
        Regex::new(r"\[\d\d\d\d-(?P<date>\d\d-\d\d) (?P<hour>\d\d):(?P<minute>\d\d)] (?P<text>.+)")
            .unwrap();
    let mut logs: Vec<&str> = input.trim().lines().collect();
    logs.sort();
    let mut current_guard: usize = 0;
    let mut sleeping_patterns = HashMap::new();
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
        let dates = &mut sleeping_patterns
            .entry(current_guard)
            .or_insert_with(HashMap::new);
        let day = dates.entry(date).or_insert_with(Vec::new);
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
    Schedule { sleeping_patterns }
}

fn part1(schedule: &Schedule) -> usize {
    // Find the biggest sleeper
    let mut total_sleep: HashMap<usize, usize> = HashMap::new();
    // This can be converted to a chain of functional calls for sure - but I'm tired.
    for (guard, days) in schedule.sleeping_patterns.iter() {
        for (_, minutes) in days.iter() {
            let minutes_of_sleep = minutes.iter().filter(|&&x| x > 0).count();
            let s = total_sleep.entry(*guard).or_insert(0);
            *s += minutes_of_sleep;
        }
    }
    let sleepiest_guard = total_sleep.iter().max_by_key(|x| x.1).unwrap().0;

    let mut days_asleep: HashMap<usize, usize> = HashMap::new();
    for (_, minutes) in schedule.sleeping_patterns[sleepiest_guard].iter() {
        for (i, minute) in minutes.iter().enumerate() {
            if *minute == 1 {
                let d = days_asleep.entry(i).or_insert(0);
                *d += 1
            }
        }
    }
    let favourite_minute = days_asleep.iter().max_by_key(|x| x.1).unwrap().0;
    sleepiest_guard * favourite_minute
}

fn part2(schedule: &Schedule) -> usize {
    // Find the biggest sleeper
    // guard_id -> number_of_times_slept_in_particular_minute
    let mut minutes_frequency: HashMap<usize, Vec<usize>> = HashMap::new();
    for (guard, days) in schedule.sleeping_patterns.iter() {
        let minutes = &mut minutes_frequency
            .entry(*guard)
            .or_insert_with(|| vec![0; 60]);
        for hour in days.values() {
            for i in 0..hour.len() {
                minutes[i] += hour[i];
            }
        }
    }

    //  (guard_id, minute, sleep_count)
    let mut chosen_one = (0, 0, 0);
    for (guard, frequency) in minutes_frequency.iter() {
        let (best_minute, days_slept) = frequency
            .iter()
            .enumerate()
            .max_by_key(|(_, &count)| count)
            .unwrap();
        if *days_slept > chosen_one.2 {
            chosen_one.0 = *guard;
            chosen_one.1 = best_minute;
            chosen_one.2 = *days_slept;
        }
    }
    chosen_one.0 * chosen_one.1
}

fn main() {
    let input = read_file("inputs/04");
    let schedule = parse_input(&input);
    let answer1 = part1(&schedule);
    assert_eq!(answer1, 84636);
    println!("Part 1: {}", answer1);

    let answer2 = part2(&schedule);
    assert_eq!(answer2, 91679);
    println!("Part 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
"#;

    #[test]
    fn test_part1() {
        let steps = parse_input(INPUT);
        assert_eq!(part1(&steps), 240);
    }

    #[test]
    fn test_part2() {
        let steps = parse_input(INPUT);
        assert_eq!(part2(&steps), 4455);
    }
}
