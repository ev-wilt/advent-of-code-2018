use std::io::{self, BufRead};

fn part_one() {
    let stdin = io::stdin();
    let mut threes = 0;
    let mut twos = 0;

    for line in stdin.lock().lines() {
        let mut has_two = false;
        let mut has_three = false;
        let id = line.unwrap();

        // Iterate through each char and count the
        // number of matches
        for i in id.chars() {
            let count = id.matches(i).count();
            if count == 2 { has_two = true }
            else if count == 3 { has_three = true }
        }
        if has_two { twos += 1 };
        if has_three { threes += 1 };
    }
    println!("{}", threes * twos);
}

fn has_one_diff(id_1: & String, id_2: & String) -> bool {
    let mut diffs_found = 0;
    for i in 0..id_1.len() {
        if id_2.chars().nth(i).unwrap() != id_1.chars().nth(i).unwrap() {
            diffs_found += 1;
        }
    }
    if diffs_found != 1 {
        return false;
    }
    true
}

fn part_two() {
    let stdin = io::stdin();
    let mut ids = Vec::new();

    // Put input into vector
    for line in stdin.lock().lines() {
        ids.push(line.unwrap());
    }

    ids.sort_unstable();
    
    // Iterate through each id, once per id
    for i in 0..ids.len() {
        for j in i..ids.len() {
            if has_one_diff(& ids[i], & ids[j]) {
                for k in 0..ids[i].len() {
                    if ids[i].chars().nth(k) == ids[j].chars().nth(k) {
                        print!("{}", ids[i].chars().nth(k).unwrap());
                    }
                }
                print!("\n");
            }
        }
    }
}

fn main() {
    part_two();
}