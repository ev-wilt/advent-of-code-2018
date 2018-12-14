use std::io::{self, BufRead};

struct Cart {
    position: (usize, usize),
    direction: char,
    covered_tile: char,
    turn_index: usize
}

fn build_track() -> Vec<Vec<char>> {
    let stdin = io::stdin();
    let mut track = Vec::new();
    for line in stdin.lock().lines() {
        let row = line.unwrap().chars().collect();
        track.push(row);
    }
    track
}

fn get_carts(track: &Vec<Vec<char>>) -> Vec<Cart> {
    let mut carts = Vec::new();
    for tuple_y in track.iter().enumerate() {
        let (y, row) = tuple_y;
        for tuple_x in row.iter().enumerate() {
            let (x, &tile) = tuple_x;
            match tile {
                'v' | '^' => {
                    let cart = Cart {
                        position: (x, y),
                        direction: tile,
                        covered_tile: '|',
                        turn_index: 0
                    };
                    carts.push(cart);
                },
                '<' | '>' => {
                    let cart = Cart {
                        position: (x, y),
                        direction: tile,
                        covered_tile: '-',
                        turn_index: 0
                    };
                    carts.push(cart);
                },
                _ => {}
            }
        }
    }
    carts
}

fn part_one() {
    let mut track = build_track();
    let mut carts = get_carts(&track);
    let mut crash_occurred = false;

    while !crash_occurred {
        for cart in &mut carts {

            // Determine cart's next direction
            match (cart.covered_tile, cart.direction) {
                ('+', _) => {
                    match (cart.turn_index, cart.direction) {
                        (0, '>') => cart.direction = '^',
                        (0, '<') => cart.direction = 'v',
                        (0, '^') => cart.direction = '<',
                        (0, 'v') => cart.direction = '>',
                        (2, '>') => cart.direction = 'v',
                        (2, '<') => cart.direction = '^',
                        (2, '^') => cart.direction = '>',
                        (2, 'v') => cart.direction = '<',
                        _ => {}
                    }
                    cart.turn_index = (cart.turn_index + 1) % 3;
                },
                ('/', '^') => cart.direction = '>',
                ('/', 'v') => cart.direction = '<',
                ('/', '<') => cart.direction = 'v',
                ('/', '>') => cart.direction = '^',                
                ('\\', '^') => cart.direction = '<',
                ('\\', 'v') => cart.direction = '>',
                ('\\', '<') => cart.direction = '^',
                ('\\', '>') => cart.direction = 'v',
                ('|', _) | ('-',_) => {},
                _ => panic!("Invalid tile, {}", cart.covered_tile)
            }

            // Replace cart with the tile it was covering
            let (cart_x, cart_y) = cart.position;
            track[cart_y][cart_x] = cart.covered_tile;

            // Update cart's position on track
            let cart_states = vec!['^', 'v', '<', '>'];
            let (new_x, new_y) = &mut cart.position;
            match cart.direction {
                '^' =>  *new_y -= 1,
                'v' => *new_y += 1,
                '<' => *new_x -= 1,
                '>' => *new_x += 1,
                _ => panic!("Invalid cart direction, {}", cart.direction)
            }

            // Move cart to new location
            if cart_states.contains(&track[*new_y][*new_x]) {
                println!("Crash occurred at ({}, {})", new_x, new_y);
                crash_occurred = true;
            }
            else {
                cart.covered_tile = track[*new_y][*new_x];
                track[*new_y][*new_x] = cart.direction;
            }

        }
        carts.sort_by_key(|c| (c.position.1, c.position.0));
    }
}


fn main() {
    part_one();
}
