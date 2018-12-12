fn largest_power(size: usize) -> (usize, usize, i32) {
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; 300]; 300];
    let serial = 6878;

    // Calculate sums for each space
    for y in 0..300 {
        for x in 0..300 {
            let rack_id: i32 = x as i32 + 11;
            let power_level: i32 = rack_id * (y as i32 + 1);
            matrix[y][x] = ((((power_level + serial) * rack_id) % 1000) / 100) - 5;
        }
    }

    // Find max sum of all 3x3 sub-matricies
    let mut matrix_strips: Vec<Vec<i32>> = vec![vec![0; 300]; 300];
    for x in 0..300 {

        let mut sum = 0;
        for y in 0..size {
            sum += matrix[y][x];
        }
        matrix_strips[0][x] = sum;

        for y in 1..(300 - size + 1) {
            sum += matrix[y + size - 1][x] - matrix[y - 1][x];
            matrix_strips[y][x] = sum;
        }
    }

    let mut max_power = std::i32::MIN;
    let mut x_start = 0;
    let mut y_start = 0;
    for y in 0..(300 - size + 1) {
        let mut sum = 0;
        for x in 0..size {
            sum += matrix_strips[y][x];
        }
        if sum > max_power {
            max_power = sum;
            x_start = 0;
            y_start = y;
        }

        for x in 1..(300 - size + 1) {
            sum += matrix_strips[y][x + size - 1] - matrix_strips[y][x - 1];
            if sum > max_power {
                max_power = sum;
                x_start = x;
                y_start = y;
            }
        }
    }
    (x_start + 1, y_start + 1, max_power)
}

fn part_one() {
    let (x_start, y_start, _) = largest_power(3);
    println!("Top Left: {}, {}", x_start, y_start);
}

fn part_two() {
    let mut largest_total_power = std::i32::MIN;
    let mut x_start = 0;
    let mut y_start = 0;
    let mut size = 0;
    for i in 1..301 {
        let (x, y, power) = largest_power(i);
        if power > largest_total_power {
            largest_total_power = power;
            size = i;
            x_start = x;
            y_start = y;
        } 
    }
    println!("Size: {}", size);
    println!("Top Left: {}, {}", x_start, y_start);
}

fn main() {
    part_two();
}
