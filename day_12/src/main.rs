use std::io::{self, BufRead};
use std::collections::HashMap;

fn build_map() -> HashMap<String, char> {
    let stdin = io::stdin();
    let mut states = HashMap::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.contains("=>") {
            let line = line.split_whitespace().collect::<Vec<_>>();
            states.insert(line[0].to_string(), line[2].chars().nth(0).unwrap());
        }
    }
    states
}

fn part_one() {
    let mut cur_state = String::from("##.#..########..##..#..##.....##..###.####.###.##.###...###.##..#.##...#.#.#...###..###.###.#.#");
    let states = build_map();
    let mut left_shift: i32 = 0;
    for _ in 0..20 as i64 {
        while &cur_state[0..4] != "....".to_string() {
            cur_state.insert(0, '.');
            left_shift += 1;
        }
        while &cur_state[cur_state.len() - 4..cur_state.len()] != "....".to_string() {
            cur_state.push('.');
        }

        let mut new_state = String::new();
        for i in 0..cur_state.len() - 4 {
            let slice = &cur_state[i..i + 5].to_string();
            new_state.push(*states.get(slice).unwrap());
        }
        left_shift -= 2;
        cur_state = new_state;
        
    }
    let mut sum: i32 = 0;
    for pot in cur_state.chars().enumerate() {
        let (index, state) = pot;
        if state == '#' {
            sum += index as i32 - left_shift;
        }
    }
    println!("{}", sum);
}


fn main() {
    part_one();
}
