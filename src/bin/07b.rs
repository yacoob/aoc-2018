extern crate regex;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug)]
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
            name: name,
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

fn main() {
    // Read the input.
    let input = include_str!("../../inputs/07").trim();
    let re = Regex::new(
        r"Step (?P<prerequisite>\D) must be finished before step (?P<target>\D) can begin.",
    )
    .unwrap();
    // Record names of steps we've seen in the input; we'll use it to identify possible starting
    // point.
    let mut seen_targets = HashSet::new();
    let mut seen_prerequisites = HashSet::new();
    // steps will contain unprocessed instruction steps, mapping from a step's ID (a single
    // character) to set of other step IDs - its prerequisites.
    // We use a hash here during input reading for convenience, as it's faster to check if step is
    // already there.
    let mut steps = HashMap::new();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let target_step_name = caps
            .name("target")
            .unwrap()
            .as_str()
            .chars()
            .nth(0)
            .unwrap();
        let prerequisite_step_name = caps
            .name("prerequisite")
            .unwrap()
            .as_str()
            .chars()
            .nth(0)
            .unwrap();
        let target_step = steps
            .entry(target_step_name)
            .or_insert(Step::new(target_step_name));
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

    // Elves, assemble!
    let total_workers = 5;
    let fixed_cost = 60;
    let mut workers: Vec<SantaLittleHelper> = (1..=total_workers)
        .map(|i| SantaLittleHelper::new(i))
        .collect();
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
        if steps.len() > 0 {
            for worker in &mut workers {
                assert!(worker.work_left >= 0);
                // Is this worker busy? Is there still anything to do?
                if worker.work_left > 0 || steps.len() == 0 {
                    continue;
                }
                // Is there a next step without prerequisites?
                if steps[0].prerequisites.len() > 0 {
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
        if steps.len() == 0 && !at_least_one_worker_busy {
            break;
        }

        // If any step got completed, make sure the outstanding steps are sorted.
        if completed_this_cycle.len() > 0 {
            steps.sort();
            completed_steps.push_str(&completed_this_cycle);
        }
    }
    // We're done!
    println!("{:->60}", "");
    println!(
        "We're done with construction; it only took us {} seconds.",
        clock - 1
    );
}
