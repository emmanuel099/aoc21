use std::io::{self, BufRead};

fn step(map: &mut [Vec<char>]) -> bool {
    let h = map.len();
    let w = map[0].len();

    let mut has_moved = false;

    // move east-facing
    for row in map.iter_mut() {
        let mut moveable = Vec::with_capacity(w);
        for i in 0..w {
            if (row[i], row[(i + 1) % w]) == ('>', '.') {
                has_moved = true;
                moveable.push(i);
            }
        }
        for i in moveable {
            row[i] = '.';
            row[(i + 1) % w] = '>';
        }
    }

    // move south-facing
    for col in 0..w {
        let mut moveable = Vec::with_capacity(w);
        for i in 0..h {
            if (map[i][col], map[(i + 1) % h][col]) == ('v', '.') {
                has_moved = true;
                moveable.push(i);
            }
        }
        for i in moveable {
            map[i][col] = '.';
            map[(i + 1) % h][col] = 'v';
        }
    }

    has_moved
}

fn main() {
    let mut map: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|s| s.unwrap().chars().collect())
        .collect();

    let mut steps = 1;
    while step(&mut map) {
        steps += 1;
    }
    println!("Part 1: {}", steps);
}
