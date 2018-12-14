use std::fs::File;
use std::io::prelude::*;

pub fn read_file(path: &str) -> String {
    let mut input = String::new();
    // Read the input.
    let mut f = File::open(path).unwrap();
    f.read_to_string(&mut input).unwrap();
    input.to_string()
}
