use std::io::{self, Read};

fn get_input_tree() -> Vec<u32> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let input_tree: Vec<u32> = buffer
            .split(' ').map(|s| s.trim())
            .map(|s| s.parse().unwrap())
            .collect();

    input_tree
}

fn find_sum(index: &mut u32, input_tree: &Vec<u32>, sum: &mut u32) -> u32 {
    let child_nodes = input_tree[*index as usize];
    let metadata_entries = input_tree[*index as usize + 1];
    *index += 2;

    // Find sum of all child nodes
    for _ in 0..child_nodes {
        find_sum(index, input_tree, sum);
    }

    // Add each entry to sum
    for _ in 0..metadata_entries {
        *sum += input_tree[*index as usize];
        *index += 1;
    }
    *sum
}

fn find_value(index: &mut u32, input_tree: &Vec<u32>) -> u32 {
    let child_nodes = input_tree[*index as usize];
    let metadata_entries = input_tree[*index as usize + 1];
    let mut child_values = Vec::new();
    let mut value = 0;
    *index += 2;

    if child_nodes > 0 {

        // Save each child's value
        for _ in 0..child_nodes {
            let child_value = find_value(index, input_tree);
            if child_value != 0 {
                child_values.push(child_value);
            }
        }
        
        // Find value using references to child values
        for _ in 0..metadata_entries {
            let reference = input_tree[*index as usize];
            if 0 < reference && reference <= child_values.len() as u32 {
                value += child_values[reference as usize - 1];
            }
            *index += 1;
        }
    }
    else {
        // Find value by summing all entries
        for _ in 0..metadata_entries {
            value += input_tree[*index as usize];
            *index += 1;
        }
    }
    value
}


fn part_one() {
    let input_tree = get_input_tree();
    let sum = find_sum(&mut 0, &input_tree, &mut 0);
    println!("{}", sum);
}


fn part_two() {
    let input_tree = get_input_tree();
    let value = find_value(&mut 0, &input_tree);
    println!("{}", value);
}

fn main() {
    part_one();
}