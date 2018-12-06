use std::io::{self, Read};

fn read_stdin() -> Result<String, io::Error> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn shortest_length(mut polymer: Vec<char>) -> usize {
    loop {
        let mut i = 1;
        let mut length = polymer.len() - 1;
        let prev_length = length;
        while i < length {
            let letter = polymer[i];
            let left = polymer[i - 1];
            let right = polymer[i + 1];
            if letter == letter.to_ascii_uppercase() {
                if left == letter.to_ascii_lowercase() {
                    polymer.remove(i);
                    polymer.remove(i - 1);
                    length -= 2;
                }
                else if right == letter.to_ascii_lowercase() {
                    polymer.remove(i + 1);
                    polymer.remove(i);
                    length -= 2;
                }
            }
            else if letter == letter.to_ascii_lowercase() {
                if left == letter.to_ascii_uppercase() {
                    polymer.remove(i);
                    polymer.remove(i - 1);
                    length -= 2;
                }
                else if right == letter.to_ascii_uppercase() {
                    polymer.remove(i + 1);
                    polymer.remove(i);
                    length -= 2;
                }
            }
            i += 1;
        }
        if length == prev_length {
            return length;
        }
    }
}

fn part_one() {
    let input = read_stdin().unwrap();
    let mut polymer: Vec<char> = Vec::new();

    for character in input.chars() {
        polymer.push(character);
    }

    println!("{}", shortest_length(polymer));
}


fn part_two() {
    let input = read_stdin().unwrap();
    let mut polymer: Vec<char> = Vec::new();
    
    for character in input.chars() {
        polymer.push(character);
    }
    let alphabet = (b'A' .. b'Z' + 1)
            .map(|c| c as char)
            .filter(|c| c.is_alphabetic())
            .collect::<Vec<_>>();

    let mut min_length = std::usize::MAX;
    for upper in alphabet {
        let lower = upper.to_ascii_lowercase();
        let polymer: Vec<char> = input
            .chars()
            .filter(|c| *c != lower && *c != upper)
            .collect();
        let length = shortest_length(polymer);
        if length < min_length {
            min_length = length;
        }
    }
    println!("{}", min_length);
}

fn main() {
    part_two();
}