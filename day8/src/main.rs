use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let entries: Vec<Entry> = io::stdin()
        .lock()
        .lines()
        .map(|line| Entry::from_str(&line.unwrap()))
        .collect();

    println!("Part 1: {}", count_one_four_seven_and_eight(&entries));
    println!("Part 2: {}", repair_and_sum_up(&entries));
}

struct Entry {
    pub signal_patterns: Vec<String>,
    pub output_values: Vec<String>,
}

impl Entry {
    pub fn from_str(input: &str) -> Entry {
        let (pattern, output) = input.split_once(" | ").unwrap();
        let signal_patterns = pattern.split(' ').map(|s| s.to_string()).collect();
        let output_values = output.split(' ').map(|s| s.to_string()).collect();
        Self {
            signal_patterns,
            output_values,
        }
    }
}

fn count_one_four_seven_and_eight(entries: &[Entry]) -> usize {
    entries
        .iter()
        .map(|entry| {
            entry
                .output_values
                .iter()
                .filter(|s| matches!(s.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

fn repair_and_sum_up(entries: &[Entry]) -> usize {
    entries
        .iter()
        .map(|entry| {
            let wiring = reconstruct_wiring(&entry);
            entry.output_values.iter().fold(0, |agg, value| {
                agg * 10 + digit_with_correction(value, &wiring)
            })
        })
        .sum()
}

#[derive(Default, Clone, Debug)]
struct SegmentCount {
    pub count: [u8; 10],
}

impl SegmentCount {
    pub fn new(s: &str) -> SegmentCount {
        let mut count = [0; 10];
        for &c in s.as_bytes() {
            count[Self::index_of_char(c)] += 1;
        }
        SegmentCount { count }
    }

    pub fn union(mut self, other: &SegmentCount) -> SegmentCount {
        for i in 0..10 {
            self.count[i] += other.count[i];
        }
        self
    }

    pub fn intersect(mut self, other: &SegmentCount) -> SegmentCount {
        for i in 0..10 {
            self.count[i] = self.count[i].min(other.count[i]);
        }
        self
    }

    pub fn expect(mut self, other: &SegmentCount) -> SegmentCount {
        for i in 0..10 {
            self.count[i] -= other.count[i];
        }
        self
    }

    pub fn filter_count(mut self, n: u8) -> SegmentCount {
        for i in 0..10 {
            if self.count[i] != n {
                self.count[i] = 0;
            }
        }
        self
    }

    pub fn without(mut self, c: char) -> SegmentCount {
        self.count[Self::index_of_char(c as u8)] = 0;
        self
    }

    pub fn expect_unique(&self) -> Option<char> {
        let mut c = None;
        for i in 0..10 {
            if self.count[i] > 0 && c.is_none() {
                if c.is_none() {
                    c = Some(Self::char_of_index(i as u8));
                } else {
                    return None;
                }
            }
        }
        c
    }

    fn index_of_char(c: u8) -> usize {
        (c - b'a') as usize
    }

    fn char_of_index(i: u8) -> char {
        (i + b'a') as char
    }
}

fn sorted(mut v: Vec<char>) -> Vec<char> {
    v.sort_unstable();
    v
}

fn reconstruct_wiring(entry: &Entry) -> HashMap<Vec<char>, usize> {
    let one = entry
        .signal_patterns
        .iter()
        .filter(|p| p.len() == 2)
        .map(|p| SegmentCount::new(p))
        .next()
        .unwrap();
    let seven = entry
        .signal_patterns
        .iter()
        .filter(|p| p.len() == 3)
        .map(|p| SegmentCount::new(p))
        .next()
        .unwrap();
    let four = entry
        .signal_patterns
        .iter()
        .filter(|p| p.len() == 4)
        .map(|p| SegmentCount::new(p))
        .next()
        .unwrap();
    let two_tree_five = entry
        .signal_patterns
        .iter()
        .filter(|p| p.len() == 5)
        .map(|p| SegmentCount::new(p))
        .fold(SegmentCount::default(), |agg, pattern| agg.union(&pattern));
    let zero_six_nine = entry
        .signal_patterns
        .iter()
        .filter(|p| p.len() == 6)
        .map(|p| SegmentCount::new(p))
        .fold(SegmentCount::default(), |agg, pattern| agg.union(&pattern));
    let eight = entry
        .signal_patterns
        .iter()
        .filter(|p| p.len() == 7)
        .map(|p| SegmentCount::new(p))
        .next()
        .unwrap();

    // 1. a
    let a = seven
        .clone()
        .expect(&one)
        .filter_count(1)
        .expect_unique()
        .unwrap();

    // 2. e
    let e = two_tree_five
        .clone()
        .union(&zero_six_nine)
        .filter_count(3)
        .expect_unique()
        .unwrap();

    // 3. b
    let b_and_e = two_tree_five
        .clone()
        .intersect(&zero_six_nine)
        .filter_count(1);
    let b = b_and_e.without(e).expect_unique().unwrap();

    // 4. c
    let b_and_c = two_tree_five.clone().union(&zero_six_nine).filter_count(4);
    let c = b_and_c.without(b).expect_unique().unwrap();

    // 5. f
    let f = one.clone().without(c).expect_unique().unwrap();

    // 6. d
    let d = four
        .without(b)
        .without(c)
        .without(f)
        .expect_unique()
        .unwrap();

    // 7. g
    let g = eight
        .without(a)
        .without(b)
        .without(c)
        .without(d)
        .without(e)
        .without(f)
        .expect_unique()
        .unwrap();

    let mut wiring = HashMap::new();
    wiring.insert(sorted(vec![a, b, c, e, f, g]), 0);
    wiring.insert(sorted(vec![c, f]), 1);
    wiring.insert(sorted(vec![a, c, d, e, g]), 2);
    wiring.insert(sorted(vec![a, c, d, f, g]), 3);
    wiring.insert(sorted(vec![b, c, d, f]), 4);
    wiring.insert(sorted(vec![a, b, d, f, g]), 5);
    wiring.insert(sorted(vec![a, b, d, e, f, g]), 6);
    wiring.insert(sorted(vec![a, c, f]), 7);
    wiring.insert(sorted(vec![a, b, c, d, e, f, g]), 8);
    wiring.insert(sorted(vec![a, b, c, d, f, g]), 9);
    // dbg!(&wiring);

    wiring
}

fn digit_with_correction(output: &str, wiring: &HashMap<Vec<char>, usize>) -> usize {
    let mut s: Vec<char> = output.chars().collect();
    s.sort_unstable();
    *wiring.get(&s).unwrap()
}

#[test]
fn test_reconstuct_wiring() {
    let entry = Entry::from_str(
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
    );
    let wiring = reconstruct_wiring(&entry);

    assert_eq!(digit_with_correction("cdfeb", &wiring), 5);
    assert_eq!(digit_with_correction("fcadb", &wiring), 3);
    assert_eq!(digit_with_correction("cdfeb", &wiring), 5);
    assert_eq!(digit_with_correction("cdbaf", &wiring), 3);
}
