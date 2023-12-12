use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    io::{self, BufRead},
};

fn adjacent_nodes(node: usize, width: usize, height: usize) -> Vec<usize> {
    let x = node % width;
    let y = node / width;

    let mut nodes = Vec::with_capacity(4);
    if x > 1 {
        nodes.push(y * width + (x - 1));
    }
    if x < width - 1 {
        nodes.push(y * width + (x + 1));
    }
    if y > 1 {
        nodes.push((y - 1) * width + x);
    }
    if y < height - 1 {
        nodes.push((y + 1) * width + x);
    }
    nodes
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct PathState {
    risk: usize,
    node: usize,
}

impl Ord for PathState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .risk
            .cmp(&self.risk)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn abs_diff(x: usize, y: usize) -> usize {
    if x < y {
        y - x
    } else {
        x - y
    }
}

fn manhattan_distance(width: usize, n1: usize, n2: usize) -> usize {
    let (x1, y1) = (n1 % width, n1 / width);
    let (x2, y2) = (n2 % width, n2 / width);
    abs_diff(x1, x2) + abs_diff(y1, y2)
}

fn heuristic(width: usize, start: usize, end: usize) -> usize {
    manhattan_distance(width, start, end)
}

// this implements A* search
fn lowest_risk(map: &[Vec<usize>], start: usize, end: usize) -> Option<usize> {
    let width = map[0].len();
    let height = map.len();
    let node_count = width * height;

    let mut heap = BinaryHeap::new();
    let mut total_risk: Vec<usize> = vec![usize::MAX; node_count];

    total_risk[start] = 0;
    heap.push(PathState {
        risk: 0,
        node: start,
    });

    while let Some(PathState { risk, node }) = heap.pop() {
        if node == end {
            return Some(risk);
        }

        for v in adjacent_nodes(node, width, height) {
            let new_risk = total_risk[node] + map[v / width][v % width];
            if new_risk < total_risk[v] {
                heap.push(PathState {
                    risk: new_risk + heuristic(width, v, end),
                    node: v,
                });
                total_risk[v] = new_risk;
            }
        }
    }

    None
}

fn expand_row(row: &[usize], n: usize, first_tile_row: bool) -> Vec<usize> {
    let mut full_row = Vec::with_capacity(row.len() * n);
    if first_tile_row {
        for &value in row {
            full_row.push(value);
        }
    } else {
        for &value in row {
            if value + 1 > 9 {
                full_row.push(1);
            } else {
                full_row.push(value + 1);
            }
        }
    }
    for tile in 1..n {
        for col in 0..row.len() {
            let prev_tile_value = full_row[(tile - 1) * row.len() + col];
            if prev_tile_value + 1 > 9 {
                full_row.push(1);
            } else {
                full_row.push(prev_tile_value + 1);
            }
        }
    }
    full_row
}

fn expand_map(first_tile: &[Vec<usize>], n: usize) -> Vec<Vec<usize>> {
    let mut full_map = Vec::with_capacity(first_tile.len() * n);
    for i in 0..first_tile.len() {
        full_map.push(expand_row(&first_tile[i], n, true));
    }
    for tile in 1..n {
        for i in 0..first_tile.len() {
            let prev_tile_row = &full_map[(tile - 1) * first_tile.len() + i];
            let expanded_row = expand_row(prev_tile_row, n, false);
            full_map.push(expanded_row);
        }
    }
    full_map
}

fn main() {
    let map: Vec<Vec<usize>> = io::stdin()
        .lock()
        .lines()
        .map(|s| {
            s.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();

    let top_left = 0;
    let bottom_right = map[0].len() * map.len() - 1;
    println!("Part 1: {:?}", lowest_risk(&map, top_left, bottom_right));

    let exanded_map = expand_map(&map, 5);
    let top_left = 0;
    let bottom_right = exanded_map[0].len() * exanded_map.len() - 1;
    println!(
        "Part 2: {:?}",
        lowest_risk(&exanded_map, top_left, bottom_right)
    );
}
