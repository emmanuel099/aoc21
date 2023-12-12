use std::io::{self, BufRead};

fn main() {
    let depths: Vec<usize> = io::stdin()
        .lock()
        .lines()
        .filter_map(|line| line.unwrap().parse().ok())
        .collect();

    part1(&depths);
    part2(&depths);
}

fn part1(depths: &[usize]) {
    println!("Part 1: {}", number_of_depth_increases(depths));
}

fn part2(depths: &[usize]) {
    let windowed_depths: Vec<_> = depths.windows(3).map(|w| w.iter().sum()).collect();
    println!("Part 2: {}", number_of_depth_increases(&windowed_depths));
}

fn number_of_depth_increases(depths: &[usize]) -> usize {
    depths.windows(2).filter(|w| w[0] < w[1]).count()
}
