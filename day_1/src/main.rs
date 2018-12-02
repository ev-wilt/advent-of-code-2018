use std::io::{self, BufRead};

fn part_one() {
    let stdin = io::stdin();
    let mut sum = 0;

    // Read each line's number/operand and add to sum
    for line in stdin.lock().lines() {
        let mut operand = line.unwrap();
        let next_num: i32 = operand.split_off(1).parse().unwrap();
        match operand.as_ref() {
            "+" => sum += next_num,
            "-" => sum -= next_num,
            _ => panic!("Unexpected operand {}", operand)
        }
    }
    println!("{}", sum);
}

fn part_two() {
    let stdin = io::stdin();
    let mut sum = 0;
    let mut iter = 0;
    let mut input = Vec::new();
    let mut prev_sums = Vec::new();

    // Put input into vector as i32's
    for line in stdin.lock().lines() {
        let mut operand = line.unwrap();
        let next_num: i32 = operand.split_off(1).parse().unwrap();
        match operand.as_ref() {
            "+" => input.push(next_num),
            "-" => input.push(-next_num),
            _ => panic!("Unexpected operand {}", operand)
        }
    }

    let length = input.len();
    // Add to sum and save it until an equal is found
    while !prev_sums.contains(&sum) {
        prev_sums.push(sum);
        sum += input[iter % length];
        iter += 1;
    }
    println!("{}", sum);
}

fn main() {
    part_two();
}