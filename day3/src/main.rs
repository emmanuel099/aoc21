use std::io::{self, BufRead};

fn main() {
    let numbers: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| usize::from_str_radix(&line.unwrap(), 2).unwrap())
        .collect();

    //const BITS: usize = 5;
    const BITS: usize = 12;

    part1::<BITS>(&numbers);
    part2::<BITS>(&numbers);
}

// runtime: O(|numbers| * BITS + BITS)
// space: O(BITS)
fn part1<const BITS: usize>(numbers: &[usize]) {
    let (bit_sum, n) = numbers
        .iter()
        .fold(([0; BITS], 0), |(mut bit_sum, n), number| {
            for i in 0..BITS {
                bit_sum[i] += number >> (BITS - i - 1) & 1;
            }
            (bit_sum, n + 1)
        });

    let gamma_rate = (0..BITS).fold(0, |gamma, i| {
        gamma | ((2 * bit_sum[i] > n) as usize) << (BITS - i - 1)
    });
    let epsilon_rate = gamma_rate ^ ((1 << BITS) - 1);

    let power_consumption = gamma_rate * epsilon_rate;
    println!("Part 1: {}", power_consumption);
}

fn part2<const BITS: usize>(numbers: &[usize]) {
    let oxygen_generator_rating =
        find_unique_number::<BITS, true>(&numbers).expect("no oxygen generator rating");
    let co2_scrubber_rating =
        find_unique_number::<BITS, false>(&numbers).expect("no CO2 scrubber rating");

    let life_support_rating = oxygen_generator_rating * co2_scrubber_rating;
    println!("Part 2: {}", life_support_rating);
}

// runtime: O(|numbers| + |numbers| * BITS)
// space: O(BITS)
fn find_unique_number<const BITS: usize, const MSB: bool>(numbers: &[usize]) -> Option<usize> {
    let mut prefix: usize = 0;

    for b in (0..=BITS).rev() {
        let prefix_filter = !((1 << b) - 1);

        let mut last_number_with_matching_prefix = 0;
        let mut count = 0;

        let mut next_bit_ones = 0;
        let mut next_bit_zeroes = 0;

        for &number in numbers {
            let has_prefix = ((number ^ prefix) & prefix_filter) == 0;
            if has_prefix {
                last_number_with_matching_prefix = number;
                count += 1;
                if b > 0 {
                    let next_bit = (number >> b - 1) & 1;
                    if next_bit == 1 {
                        next_bit_ones += 1;
                    } else {
                        next_bit_zeroes += 1;
                    }
                }
            }
        }

        if b > 0 {
            if MSB {
                if next_bit_ones >= next_bit_zeroes {
                    prefix |= 1 << b - 1;
                }
            } else {
                if next_bit_ones < next_bit_zeroes {
                    prefix |= 1 << b - 1;
                }
            }
        }

        if count == 1 {
            return Some(last_number_with_matching_prefix);
        }
    }

    None
}
