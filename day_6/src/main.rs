use std::io::{self, BufRead};
use std::collections::HashMap;

fn get_input_points() -> Vec<(i32, i32)> {
    let mut input_points: Vec<(i32, i32)> = Vec::new();
    let stdin = io::stdin();

    // Put input into tuple, then insert
    // into a hash map
    for line in stdin.lock().lines() {
        let vec: Vec<i32> = line
            .unwrap()
            .split(", ")
            .map(|i| i.parse::<i32>().unwrap())
            .collect();
        input_points.push((vec[0], vec[1]));
    }
    input_points
}

fn manhattan_distance(point_1: (i32, i32), point_2: (i32, i32)) -> i32 {
    let (x1, y1) = point_1;
    let (x2, y2) = point_2;
    (x2 - x1).abs() + (y2 - y1).abs()
}

fn part_one() {
    let mut areas: HashMap<(i32, i32), i32> = HashMap::new();
    let input_points = get_input_points();
    let mut possible_points = input_points.clone();
    let (mut min_x, mut min_y) = (std::i32::MAX, std::i32::MAX);
    let (mut max_x, mut max_y) = (0, 0);

    // Find grid boundaries
    for point in &input_points {
        let (x, y) = point;
        if *x < min_x { min_x = *x };
        if *x > max_x { max_x = *x };
        if *y < min_y { min_y = *y };
        if *y > max_y { max_y = *y };
    }

    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {

            // Check if point is on the boundary
            let mut infinite_area = false;
            if  x == min_x || 
                x == max_x ||
                y == min_y ||
                y == max_y { infinite_area = true }

            // Find shortest manhattan distance
            let mut min_distance = std::i32::MAX;
            let mut closest_point: (i32, i32) = (0, 0);
            for point in &input_points {
                let distance = manhattan_distance(*point, (x, y));
                if distance < min_distance {
                    min_distance = distance;
                    closest_point = *point;
                }
            }

            // Remove any input points that have infinite areas 
            // as a potential solution
            if infinite_area && possible_points.contains(&closest_point) {
                let index = possible_points
                    .iter()
                    .position(|x| *x == closest_point)
                    .unwrap();
                possible_points.remove(index);
            }

            // Save total area of any non-infinite point
            else {
                if areas.contains_key(&closest_point) {
                    let total: &mut i32 = areas.get_mut(&closest_point).unwrap();
                    *total += 1;
                }
                else {
                    areas.insert(closest_point, 1);
                }
            }
        }
    }

    // Find non-infinite point with max area
    let mut max_area = 0;
    for point in &possible_points {
        let area = areas.get(point).unwrap();
        if *area > max_area { max_area = *area }
    }

    println!("{}", max_area);
}


fn part_two() {
    let input_points = get_input_points();
    let (mut min_x, mut min_y) = (std::i32::MAX, std::i32::MAX);
    let (mut max_x, mut max_y) = (0, 0);

    // Find grid boundaries
    for point in &input_points {
        let (x, y) = point;
        if *x < min_x { min_x = *x };
        if *x > max_x { max_x = *x };
        if *y < min_y { min_y = *y };
        if *y > max_y { max_y = *y };
    }

    let mut region_size = 0;
    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {

            // Sum all distances for each input point
            let mut distance_sum = 0;
            for point in &input_points {
                distance_sum += manhattan_distance(*point, (x, y));
            }            
            if distance_sum < 10000 {
                region_size += 1;
            }
        }
    }
    println!("{}", region_size);
}

fn main() {
    part_two();
}