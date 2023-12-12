use std::{
    cmp,
    collections::HashMap,
    io::{self, BufRead},
};

fn count_elements(
    first_polymer_element: char,
    pairs: &HashMap<String, usize>,
) -> HashMap<char, usize> {
    pairs.iter().fold(
        {
            let mut count = HashMap::new();
            count.insert(first_polymer_element, 1);
            count
        },
        |mut count, (key, value)| {
            let second_pair_element = key[1..].chars().next().unwrap();
            *count.entry(second_pair_element).or_insert(0) += value;
            count
        },
    )
}

fn min_max_elements(first_polymer_element: char, pairs: &HashMap<String, usize>) -> (usize, usize) {
    count_elements(first_polymer_element, pairs)
        .iter()
        .fold((usize::MAX, usize::MIN), |(min, max), (_, &count)| {
            (cmp::min(min, count), cmp::max(max, count))
        })
}

fn pairs_of_polymer(polymer: &str) -> HashMap<String, usize> {
    let mut pairs = HashMap::with_capacity(polymer.len() - 1);
    for i in 0..polymer.len() - 1 {
        let pair = &polymer[i..i + 2];
        *pairs.entry(String::from(pair)).or_insert(0) += 1;
    }
    pairs
}

fn grow_polymer(
    initial_pairs: HashMap<String, usize>,
    rules: &HashMap<&str, &str>,
    steps: usize,
) -> HashMap<String, usize> {
    (0..steps).fold(initial_pairs, |pairs, _| {
        let mut next_pairs = HashMap::with_capacity(pairs.len() * 2);
        for (pair, count) in pairs {
            if let Some(insert) = rules.get(&pair[..]) {
                let mut pair1 = String::with_capacity(insert.len() + 1);
                pair1.push_str(&pair[..1]);
                pair1.push_str(insert);
                *next_pairs.entry(pair1).or_insert(0) += count;

                let mut pair2 = String::with_capacity(insert.len() + 1);
                pair2.push_str(insert);
                pair2.push_str(&pair[1..]);
                *next_pairs.entry(pair2).or_insert(0) += count;
            } else {
                *next_pairs.entry(pair).or_insert(0) += count;
            }
        }
        next_pairs
    })
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|s| s.unwrap()).collect();

    let polymer_template = &lines[0];
    let insertion_rules: HashMap<&str, &str> = lines
        .iter()
        .skip(2)
        .map(|s| s.split_once(" -> ").unwrap())
        .collect();

    let polymer1_pairs = grow_polymer(pairs_of_polymer(polymer_template), &insertion_rules, 10);
    let (min, max) = min_max_elements(polymer_template.chars().next().unwrap(), &polymer1_pairs);
    println!("Part 1: {}", max - min);

    let polymer2_pairs = grow_polymer(polymer1_pairs, &insertion_rules, 30);
    let (min, max) = min_max_elements(polymer_template.chars().next().unwrap(), &polymer2_pairs);
    println!("Part 2: {}", max - min);
}
