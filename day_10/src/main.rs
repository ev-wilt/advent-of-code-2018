use std::io::{self, BufRead };

struct Point {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32
}

fn build_points() -> Vec<Point> {
    let stdin = io::stdin();
    let mut points = Vec::new();

    // Put input into vector
    for line in stdin.lock().lines() {
        let star = line.unwrap();
        let x: String = star.chars().skip(10).take(6).collect();
        let y: String = star.chars().skip(18).take(6).collect();
        let dx: String = star.chars().skip(36).take(2).collect();
        let dy: String = star.chars().skip(40).take(2).collect();
        let point = Point {
            x: x.trim().parse::<i32>().unwrap(),
            y: y.trim().parse::<i32>().unwrap(),
            dx: dx.trim().parse::<i32>().unwrap(),
            dy: dy.trim().parse::<i32>().unwrap()
        };
        points.push(point);
    }
    points
}

fn part_one_two() {
    let mut points = build_points();
    let mut y_max = std::i32::MIN;
    let mut y_min = std::i32::MAX;
    let mut x_max = std::i32::MIN;
    let mut x_min = std::i32::MAX;
    let mut seconds = 0;

    while seconds != 10036 {
        y_max = std::i32::MIN;
        y_min = std::i32::MAX;
        x_max = std::i32::MIN;
        x_min = std::i32::MAX;

        for point in &mut points {
            point.x += point.dx;
            point.y += point.dy;
            if point.y > y_max { y_max = point.y; }
            if point.y < y_min { y_min = point.y; }
            if point.x > x_max { x_max = point.x; }
            if point.x < x_min { x_min = point.x; }
        }
        seconds += 1;
    }

    for y in y_min - 5..y_max + 5 {
        for x in x_min - 5..x_max + 5 {
            let mut point_here = false;
            for point in &points {
                if point.x == x && point.y == y {
                    print!("#");
                    point_here = true;
                    break;
                }
            }
            if !point_here {
                print!(" ");
            }
        }
        print!("\n");
    }
    println!("{}", seconds);
}

fn main() {
    part_one_two();
}