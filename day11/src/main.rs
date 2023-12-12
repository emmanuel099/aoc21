use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let grid: Vec<usize> = io::stdin()
        .lock()
        .lines()
        .flat_map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect();

    part1(grid.clone());
    part2(grid);
}

fn part1(grid: Vec<usize>) {
    let mut octopuses = Octopuses::new(grid);
    let total_flashes: usize = (1..=100).map(|_| octopuses.step()).sum();
    println!("Part 1: {}", total_flashes);
}

fn part2(grid: Vec<usize>) {
    let mut octopuses = Octopuses::new(grid);
    let first_step_with_simultaneous_flash = (1..)
        .map(|step| (step, octopuses.step()))
        .filter(|(_, flashes)| *flashes == 100)
        .map(|(step, _)| step)
        .next();
    println!("Part 2: {:?}", first_step_with_simultaneous_flash);
}

struct Octopuses {
    grid: Vec<usize>,
    size: usize,
}

impl Octopuses {
    pub fn new(grid: Vec<usize>) -> Octopuses {
        let size = (grid.len() as f64).sqrt() as usize;
        Self { grid, size }
    }

    pub fn step(&mut self) -> usize {
        self.increase_energy();
        let flashed = self.flash_until_fixed_point();
        self.reset_flashed(&flashed);
        flashed.len()
    }

    fn increase_energy(&mut self) {
        self.grid.iter_mut().for_each(|energy| *energy += 1);
    }

    fn flash_until_fixed_point(&mut self) -> HashSet<usize> {
        let mut flashed = HashSet::with_capacity(self.size * self.size);

        loop {
            let new_flashed: Vec<_> = self
                .grid
                .iter()
                .enumerate()
                .filter_map(|(i, &energy)| {
                    if energy > 9 && flashed.insert(i) {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect();

            if new_flashed.is_empty() {
                break flashed;
            }

            new_flashed
                .iter()
                .flat_map(|&i| Self::diagonal_adjacent_indices(self.size, i))
                .for_each(|i| self.grid[i] += 1);
        }
    }

    fn reset_flashed(&mut self, flashed: &HashSet<usize>) {
        flashed.iter().for_each(|&i| self.grid[i] = 0);
    }

    fn diagonal_adjacent_indices(size: usize, idx: usize) -> Vec<usize> {
        let row = idx / size;
        let col = idx % size;

        let mut adjacent = Vec::with_capacity(8);
        if row > 0 {
            adjacent.push((row - 1) * size + col); // above
            if col > 0 {
                adjacent.push((row - 1) * size + col - 1); // above left
            }
            if col < size - 1 {
                adjacent.push((row - 1) * size + col + 1); // above right
            }
        }
        if row < size - 1 {
            adjacent.push((row + 1) * size + col); // below
            if col > 0 {
                adjacent.push((row + 1) * size + col - 1); // below left
            }
            if col < size - 1 {
                adjacent.push((row + 1) * size + col + 1); // below right
            }
        }
        if col > 0 {
            // left
            adjacent.push(row * size + col - 1);
        }
        if col < size - 1 {
            // right
            adjacent.push(row * size + col + 1);
        }
        adjacent
    }
}
