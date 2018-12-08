use std::io::{self, BufRead};
use std::collections::BTreeMap;

fn build_dependency_map() -> BTreeMap<char, Vec<char>> {
    let stdin = io::stdin();
    let mut dependency_map = BTreeMap::new();

    for line in stdin.lock().lines() {
        let instruction = line.unwrap();
        let dependency = instruction.chars().nth(5).unwrap();
        let dependent = instruction.chars().nth(36).unwrap();

        if !dependency_map.contains_key(&dependency) {
            dependency_map.insert(dependency, Vec::new());
        }
        if !dependency_map.contains_key(&dependent) {
            dependency_map.insert(dependent, vec![dependency]);
        }
        else {
            let mut dependencies = dependency_map.get_mut(&dependent).unwrap();
            dependencies.push(dependency);
        }
    }
    dependency_map
}

fn part_one() {
    let mut solution = String::new();
    let mut dependency_map = build_dependency_map();

    while dependency_map.len() != 0 {

        // Find first element that has all dependencies met
        // and add it to solution
        let mut iteration_done = false;
        for (dependent, dependencies) in dependency_map.iter_mut() {
            if !iteration_done {
                if dependencies.is_empty() {
                    solution.push(*dependent);
                    iteration_done = true;
                }
                else {
                    let mut deps_satisfied = true;
                    for dependency in dependencies {
                        if !solution.contains(*dependency) {
                            deps_satisfied = false;
                        }
                    }
                    if deps_satisfied {
                        solution.push(*dependent);
                        iteration_done = true;
                    }
                }
            }
        }
        // Remove any used chars from the map
        for letter in solution.chars() {
            dependency_map.remove(&letter);
        }
    }
    println!("{}", solution);
}

fn part_two() {
    let mut solution = String::new();
    let mut dependency_map = build_dependency_map();
    let solution_size = dependency_map.len();
    let total_workers = 5;
    let mut active_workers: Vec<(char, u8)> = Vec::new();
    let mut counter = 0;

    while solution.len() < solution_size {
        // Update each active worker
        for worker in &mut active_workers {
            let (step, time) = worker;
            *time -= 1;
            if *time == 0 {
                solution.push(*step);
            }
        }
        active_workers.retain(|(_, t)| *t != 0);

        // Check if a new active worker is needed
        for _ in active_workers.len()..total_workers {
            let mut iteration_done = false;
            let mut letters_to_remove = Vec::new();
            for (dependent, dependencies) in dependency_map.iter_mut() {
                if !iteration_done {
                    if dependencies.is_empty() {
                        letters_to_remove.push(*dependent);
                        active_workers.push((*dependent, (*dependent as u8 - 64) + 60));
                        iteration_done = true;
                    }
                    else {
                        let mut deps_satisfied = true;
                        for dependency in dependencies {
                            if !solution.contains(*dependency) {
                                deps_satisfied = false;
                            }
                        }
                        if deps_satisfied {
                            letters_to_remove.push(*dependent);
                            active_workers.push((*dependent, (*dependent as u8 - 64) + 60));
                            iteration_done = true;
                        }
                    }
                }
            }
            // Remove any used chars from the map
            for letter in letters_to_remove {
                dependency_map.remove(&letter);
            }
        }
        counter += 1;
    }
    println!("{}", counter - 1);
}

fn main() {
    part_two();
}