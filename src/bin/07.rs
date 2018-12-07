use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Step {
    name: char,
    prerequisites: HashSet<char>,
}

// Well, what did you expect? :D
#[derive(Clone, Debug)]
struct SantaLittleHelper {
    id: i32,
    last_worked_on: char,
    work_left: i32,
}

impl Step {
    fn depend_on(&mut self, other: char) {
        self.prerequisites.insert(other);
    }

    fn new(name: char) -> Step {
        Step {
            name,
            prerequisites: HashSet::new(),
        }
    }
}

impl SantaLittleHelper {
    fn new(i: i32) -> SantaLittleHelper {
        // Born ready.
        println!("I'm not a number (#{}) - I'm a free man (or elf)!", i);
        SantaLittleHelper {
            id: i,
            last_worked_on: '_',
            work_left: 0,
        }
    }
}

impl Ord for Step {
    // Comparator for steps. Order is dictated by number of dependencies first, lexographic name
    // order second.
    fn cmp(&self, other: &Step) -> Ordering {
        let c = self.prerequisites.len().cmp(&other.prerequisites.len());
        match c {
            Ordering::Equal => self.name.cmp(&other.name),
            _ => c,
        }
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Step) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn read_file(path: &str) -> String {
    let mut input = String::new();
    // Read the input.
    let mut f = File::open(path).unwrap();
    f.read_to_string(&mut input).unwrap();
    input
}

fn parse_input(input: &str) -> Vec<Step> {
    // Record names of steps we've seen in the input; we'll use it to identify possible starting
    // point.
    let mut seen_targets = HashSet::new();
    let mut seen_prerequisites = HashSet::new();
    // steps will contain unprocessed instruction steps, mapping from a step's ID (a single
    // character) to set of other step IDs - its prerequisites.
    // We use a hash here during input reading for convenience, as it's faster to check if step is
    // already there.
    let mut steps = HashMap::new();
    for line in input.trim().lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let target_step_name = words[7].chars().nth(0).unwrap();
        let prerequisite_step_name = words[1].chars().nth(0).unwrap();
        let target_step = steps
            .entry(target_step_name)
            .or_insert_with(|| Step::new(target_step_name));
        target_step.depend_on(prerequisite_step_name);
        seen_targets.insert(target_step_name);
        seen_prerequisites.insert(prerequisite_step_name);
    }
    // Determine starting point. A possible starting point is a step that has been seen as
    // prerequisite, but not as a target in the input.
    let possible_origins = seen_prerequisites.difference(&seen_targets);
    for origin in possible_origins {
        steps.insert(*origin, Step::new(*origin));
    }
    // With all steps gathered, we can turn steps hash into a vector.
    let mut steps: Vec<Step> = steps.drain().map(|(_, v)| v).collect();
    steps.sort();
    steps
}

fn part1(steps: &[Step]) -> String {
    // Time to work out the sequence of steps.
    let mut steps = steps.to_owned();
    let mut step_sequence = String::with_capacity(steps.len());
    while !steps.is_empty() {
        // Take the first step, which should have no deps, and add it to the sequence.
        let current_step = steps.remove(0);
        assert_eq!(current_step.prerequisites.len(), 0);
        step_sequence.push(current_step.name);
        // Remove current_step from dependencies of all outstanding steps.
        for step in steps.iter_mut() {
            step.prerequisites.remove(&current_step.name);
        }
        steps.sort();
    }
    step_sequence
}

fn part2(steps: &[Step], total_workers: i32, fixed_cost: i32) -> i32 {
    // Elves, assemble!
    let mut steps = steps.to_owned();
    let mut workers: Vec<SantaLittleHelper> =
        (1..=total_workers).map(SantaLittleHelper::new).collect();
    let mut completed_steps = String::with_capacity(steps.len());
    let mut clock = 0;
    // Print a header for the timesheet.
    print!("clock\t");
    for worker in &workers {
        print!("{}\t", worker.id);
    }
    println!("Completed");
    println!("{:->60}", "");
    // Get to work people!
    loop {
        print!("{:>2}\t", clock);
        // Give every worker that is currently not working next outstanding unblocked step.
        if !steps.is_empty() {
            for worker in &mut workers {
                assert!(worker.work_left >= 0);
                // Is this worker busy? Is there still anything to do?
                if worker.work_left > 0 || steps.is_empty() {
                    continue;
                }
                // Is there a next step without prerequisites?
                if !steps[0].prerequisites.is_empty() {
                    // No unblocked steps, no more work for anyone in this second.
                    break;
                }
                let step = steps.remove(0);
                let step_work_time = step.name as i32 - 'A' as i32 + 1 + fixed_cost;
                // Actually give work to the worker.
                worker.work_left = step_work_time;
                worker.last_worked_on = step.name;
            }
        }
        //
        // Tick of time passes, workers do their bidding.
        //
        // Iterate through workers, decrease their outstanding work time.
        clock += 1;
        let mut at_least_one_worker_busy = false;
        let mut completed_this_cycle = String::new();
        for worker in &mut workers {
            if worker.work_left > 0 {
                at_least_one_worker_busy = true;
                worker.work_left -= 1;
                print!("{}\t", worker.last_worked_on);
                // Has this worker just completed a step?
                if worker.work_left == 0 {
                    completed_this_cycle.push(worker.last_worked_on);
                    // Remove finished step from all oustanding steps' prerequisites.
                    for step in &mut steps {
                        step.prerequisites.remove(&worker.last_worked_on);
                    }
                }
            } else {
                print!(".\t");
            }
        }
        println!("{}", completed_steps);

        // Maybe we're done?
        if !at_least_one_worker_busy && steps.is_empty() {
            break;
        }

        // If any step got completed, make sure the outstanding steps are sorted.
        if !completed_this_cycle.is_empty() {
            steps.sort();
            completed_steps.push_str(&completed_this_cycle);
        }
    }
    // We're done!
    println!("{:->60}", "");
    clock - 1
}

fn main() {
    let filename = "inputs/07";
    let number_of_workers = 5;
    let static_work_cost = 60;

    let input = read_file(filename);
    let steps = parse_input(&input);
    let step_sequence = part1(&steps);
    assert_eq!(step_sequence, "GRTAHKLQVYWXMUBCZPIJFEDNSO");
    println!(
        "Here's the sequence of steps for a single worker: {}",
        step_sequence
    );
    let time_elapsed = part2(&steps, number_of_workers, static_work_cost);
    assert_eq!(time_elapsed, 1115);
    println!(
        "We're done with construction; it only took us {} seconds.",
        time_elapsed
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
"#;

    #[test]
    fn test_part1() {
        let steps = parse_input(INPUT);
        assert_eq!(part1(&steps), "CABDFE");
    }

    #[test]
    fn test_part2() {
        let steps = parse_input(INPUT);
        assert_eq!(part2(&steps, 2, 0), 15);
    }
}
