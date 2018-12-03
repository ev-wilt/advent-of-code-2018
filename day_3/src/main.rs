use std::io::{self, BufRead};

fn part_one() {
    let stdin = io::stdin();
    let mut fabric = vec![vec![".".to_string(); 1000]; 1000];
    let mut sq_inches = 0;
    
    for line in stdin.lock().lines() {

        // Parse input
        let mut claim = line.unwrap();
        claim.retain(|c| c != ':' && c != '#');
        let claim: Vec<String> = claim.split(' ').map(String::from).collect();
        let id = &claim[0];
        let start_point: Vec<i32> = claim[2].split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        let size: Vec<i32> = claim[3].split('x').map(|x| x.parse::<i32>().unwrap()).collect();

        for y in 0..size[1]  {
            for x in 0..size[0] {

                // If point hasn't been visited, set point to ID
                if fabric[(y + start_point[1]) as usize][(x + start_point[0]) as usize] == "." {
                    fabric[(y + start_point[1]) as usize][(x + start_point[0]) as usize] = id.to_string();
                }

                // If point has been visted once, set point to X and 
                // increase the overall square inches
                else if fabric[(y + start_point[1]) as usize][(x + start_point[0]) as usize] != "X" {
                    sq_inches += 1;
                    fabric[(y + start_point[1]) as usize][(x + start_point[0]) as usize] = "X".to_string();
                }
            }
        }
    }

    println!("{}", sq_inches);
}

fn part_two() {
    let stdin = io::stdin();
    let mut fabric = vec![vec![".".to_string(); 1000]; 1000];
    let mut overlapping_ids: Vec<bool> = vec![false; 1233];

    for line in stdin.lock().lines() {

        // Parse input
        let mut claim = line.unwrap();
        claim.retain(|c| c != ':' && c != '#');
        let claim: Vec<String> = claim.split(' ').map(String::from).collect();
        let id = &claim[0];
        let start_point: Vec<i32> = claim[2].split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        let size: Vec<i32> = claim[3].split('x').map(|x| x.parse::<i32>().unwrap()).collect();

        for y in 0..size[1]  {
            for x in 0..size[0] {

                // If point hasn't been visited, add ID
                if fabric[(y + start_point[1]) as usize][(x + start_point[0]) as usize] == "." {
                    fabric[(y + start_point[1]) as usize][(x + start_point[0]) as usize] = id.to_string();
                }

                // If point has been visited > 2 times, just set ID to overlapping
                else if fabric[(y + start_point[1]) as usize][(x + start_point[0]) as usize] == "X" {
                    overlapping_ids[id.parse::<usize>().unwrap() - 1] = true;
                }

                // If point has been visited once, set both the current ID and the 
                // ID at that point to overlapping
                else {
                    overlapping_ids[fabric[(y + start_point[1]) as usize][(x + start_point[0]) as usize].parse::<usize>().unwrap() - 1] = true;
                    fabric[(y + start_point[1]) as usize][(x + start_point[0]) as usize] = "X".to_string();
                    overlapping_ids[id.parse::<usize>().unwrap() - 1] = true;
                }
            }
        }
    }

    for id in 0..overlapping_ids.len() {
        if !overlapping_ids[id] {
            println!("{}", id + 1);
        }
    }
}

fn main() {
    part_one();
}