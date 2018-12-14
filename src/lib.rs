use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

pub fn read_file(path: &str) -> String {
    let mut input = String::new();
    // Read the input.
    let mut f = File::open(path).unwrap();
    f.read_to_string(&mut input).unwrap();
    input.to_string()
}

pub struct Stopwatch {
    clock: Instant,
}

impl Stopwatch {
    pub fn start() -> Stopwatch {
        Stopwatch {
            clock: Instant::now(),
        }
    }

    pub fn split(self) {
        let d = self.clock.elapsed();
        eprintln!("Done in {}.{:06} seconds", d.as_secs(), d.subsec_micros());
    }
}
