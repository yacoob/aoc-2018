use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read puzzle input. Aren't we optimistic that everything will work just fine? :D
    let f = File::open("inputs/01").unwrap();
    let buffered = BufReader::new(&f);
    
    // You can substantially simplify all of this with iterator adapters:
    let freq = buffered.lines().map(|line| line.unwrap().parse::<i32>().unwrap()).sum();
    println!("Final frequency: {}", freq);
    
    // Also, generally BufRead::lines is a bit of an antipattern, because it does way
    // too much allocation (because it creates a new string for every line). This is a bit
    // more idiomatic:
    let mut input = String::new();
    let f = File::open("inputs/01").unwrap();
    f.read_to_string(&mut input).unwrap();
    let freq = input.lines().map(|line| line.parse::<i32>().unwrap()).sum();
    println!("Final frequency: {}", freq);
    
}
