use std::io::{self, BufRead};

fn build_map() {
    let stdin = io::stdin();
    let mut input = Vec::new();

    // Put input into vector
    for line in stdin.lock().lines() {
        input.push(line.unwrap());
    }
}

fn part_one() {
}

fn part_two() {
    
}

fn main() {
    part_one();
}