use std::io::{self, Read};
use std::collections::VecDeque;

fn get_input() -> (i32, i32) {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let v: Vec<&str> = buffer.split(' ').collect();
    (v[0].parse::<i32>().unwrap(), v[6].parse::<i32>().unwrap())
}

// Using one function since only diff is input
fn part_one_two() {
    let (player_count, last_marble) = get_input();
    let mut circle: VecDeque<usize> = VecDeque::new();
    circle.push_back(0);
    let mut players = vec![0; player_count as usize];

    // Iterate until we've reached the last marble
    for i in 1.. last_marble {

        // Insert marble into correct location 
        if i % 23 != 0 {
            for _ in 0..2 {
                let marble = circle.pop_front().unwrap();
                circle.push_back(marble);
            }
            circle.push_front(i as usize);
        }
        else {
            for _ in 0..7{
                let marble = circle.pop_back().unwrap();
                circle.push_front(marble);
            }
            players[(i % player_count) as usize] += i as usize + circle.pop_front().unwrap();
        }
    }
    println!("{}", players.iter().max().unwrap());
}

fn main() {
    part_one_two();
}