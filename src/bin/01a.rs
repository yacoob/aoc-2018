use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut input = String::new();
    // Read puzzle input. Aren't we optimistic that everything will work just fine? :D
    let mut file = File::open("inputs/01").unwrap();
    file.read_to_string(&mut input).unwrap();
    // Look, ma! Fully functional!
    let freq: i32 = input.lines().map(|line| line.parse::<i32>().unwrap()).sum();
    println!("Final frequency: {}", freq);
}
