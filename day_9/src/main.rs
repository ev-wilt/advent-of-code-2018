use std::io::{self, Read};

fn get_input() -> (i32, i32) {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let v: Vec<&str> = buffer.split(' ').collect();
    (v[0].parse::<i32>().unwrap(), v[6].parse::<i32>().unwrap())
}

// Using one function since only diff is input
fn part_one_two() {
    let (player_count, last_marble) = get_input();
    let mut circle = vec![0, 2, 1];
    let mut players = vec![0; player_count as usize];
    let mut current_marble = 2;
    let mut current_index = 1;

    // Iterate until we've reached the last marble
    'game_over: loop {
        for score in &mut players {
            if current_marble == last_marble as usize {
                break 'game_over;
            }

            // Insert marble into correct location 
            if (current_marble + 1) % 23 != 0 {
                let new_index = if current_index == circle.len() - 1 {
                    (current_index + 3) % (circle.len() + 1)
                } else {
                    (current_index + 2) % (circle.len() + 1)
                };
                circle.insert(new_index, current_marble + 1);
                current_index = new_index;
            }
            else {
                let length = circle.len() as i32;

                // Finding modulus of length
                let new_index = ((((current_index as i32 - 7) % length) + length) % length) as usize;
                *score += (current_marble + 1) + circle[new_index];
                circle.remove(new_index);
                current_index = new_index;
            }
            current_marble += 1;
            println!("{}", current_marble);
        }
    }
    println!("{:?}", players.iter().max());
}

fn main() {
    part_one_two();
}