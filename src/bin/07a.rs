extern crate regex;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug)]
struct Step {
    name: char,
    deps: HashSet<char>,
}

impl Step {
    fn depend_on(&mut self, other: char) {
        self.deps.insert(other);
    }

    fn new(name: char) -> Step {
        Step {
            name,
            deps: HashSet::new(),
        }
    }
}

impl Ord for Step {
    // Comparator for steps. Order is dictated by number of dependencies first, lexographic name
    // order second.
    fn cmp(&self, other: &Step) -> Ordering {
        let c = self.deps.len().cmp(&other.deps.len());
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

    // Time to work out the sequence of steps.
    let mut step_sequence = String::with_capacity(steps.len());
    while !steps.is_empty() {
        // Take the first step, which should have no deps, and add it to the sequence.
        let current_step = steps.remove(0);
        assert_eq!(current_step.deps.len(), 0);
        step_sequence.push(current_step.name);
        // Remove current_step from dependencies of all outstanding steps.
        for step in steps.iter_mut() {
            step.deps.remove(&current_step.name);
        }
        steps.sort();
    }
    println!("Here's the sequence of steps: {}", step_sequence);
}
