#![feature(box_patterns)]

use std::{
    cmp, fmt,
    io::{self, BufRead},
    ops, str,
};

#[derive(Debug, Clone, PartialEq)]
enum SnailfishNumber {
    Regular(usize),
    Pair(Box<Self>, Box<Self>),
}

impl SnailfishNumber {
    pub fn regular(n: usize) -> SnailfishNumber {
        Self::Regular(n)
    }

    pub fn pair(lhs: SnailfishNumber, rhs: SnailfishNumber) -> SnailfishNumber {
        Self::Pair(Box::new(lhs), Box::new(rhs))
    }

    fn add_to_leftmost_regular(&mut self, n: usize) {
        match self {
            Self::Regular(m) => *m += n,
            Self::Pair(lhs, _) => lhs.add_to_leftmost_regular(n),
        }
    }

    fn add_to_rightmost_regular(&mut self, n: usize) {
        match self {
            Self::Regular(m) => *m += n,
            Self::Pair(_, rhs) => rhs.add_to_rightmost_regular(n),
        }
    }

    fn explode(&mut self, depth: usize) -> (bool, Option<usize>, Option<usize>) {
        match self {
            Self::Regular(_) => (false, None, None),
            Self::Pair(box Self::Regular(lhs), box Self::Regular(rhs)) if depth >= 4 => {
                let left_value = *lhs;
                let right_value = *rhs;
                *self = Self::regular(0);
                (true, Some(left_value), Some(right_value))
            }
            Self::Pair(lhs, rhs) => {
                let (exploded, left_value, right_value) = lhs.explode(depth + 1);
                if exploded {
                    if let Some(n) = right_value {
                        rhs.add_to_leftmost_regular(n);
                    }
                    return (true, left_value, None);
                }

                let (exploded, left_value, right_value) = rhs.explode(depth + 1);
                if exploded {
                    if let Some(n) = left_value {
                        lhs.add_to_rightmost_regular(n);
                    }
                    return (true, None, right_value);
                }

                (false, None, None)
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Regular(n) if *n >= 10 => {
                let lhs = *n / 2;
                let rhs = *n - lhs;
                *self = Self::pair(Self::regular(lhs), Self::regular(rhs));
                true
            }
            Self::Regular(_) => false,
            Self::Pair(lhs, rhs) => lhs.split() || rhs.split(),
        }
    }

    fn reduce(&mut self) {
        loop {
            let (exploded, _, _) = self.explode(0);
            if exploded {
                continue;
            }

            let split = self.split();
            if split {
                continue;
            }

            break;
        }
    }

    fn parse_regular(mut chars: str::Chars<'_>) -> (str::Chars<'_>, SnailfishNumber) {
        let s = chars.as_str();
        while chars.clone().next().map_or(false, |c| c.is_numeric()) {
            chars.next();
        }
        let n = &s[..s.len() - chars.as_str().len()];
        (chars, SnailfishNumber::regular(n.parse().unwrap()))
    }

    fn parse_pair(mut chars: str::Chars<'_>) -> (str::Chars<'_>, SnailfishNumber) {
        chars.next(); // [
        let (mut chars, lhs) = Self::parse_number(chars);
        chars.next(); // ,
        let (mut chars, rhs) = Self::parse_number(chars);
        chars.next(); // ]
        (chars, SnailfishNumber::pair(lhs, rhs))
    }

    fn parse_number(chars: str::Chars<'_>) -> (str::Chars<'_>, SnailfishNumber) {
        match chars.clone().next() {
            Some('[') => Self::parse_pair(chars),
            Some(c) if c.is_numeric() => Self::parse_regular(chars),
            _ => panic!(),
        }
    }

    pub fn parse(s: &str) -> SnailfishNumber {
        let (_, mut n) = Self::parse_number(s.chars());
        n.reduce();
        n
    }

    pub fn magnitude(&self) -> usize {
        match self {
            Self::Regular(n) => *n,
            Self::Pair(lhs, rhs) => lhs.magnitude() * 3 + rhs.magnitude() * 2,
        }
    }
}

impl ops::Add for SnailfishNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut n = Self::pair(self, other);
        n.reduce();
        n
    }
}

impl fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Regular(n) => write!(f, "{}", n),
            Self::Pair(n1, n2) => write!(f, "[{},{}]", n1, n2),
        }
    }
}

fn max_pairwise_magnitude(numbers: &[SnailfishNumber]) -> Option<usize> {
    if numbers.is_empty() {
        return None;
    }

    let mut max_magnitude = 0;

    // addition of snailfish number is not commutative -> need to consider all pairs!
    for n1 in numbers {
        for n2 in numbers {
            let sum = n1.clone() + n2.clone();
            max_magnitude = cmp::max(max_magnitude, sum.magnitude());
        }
    }

    Some(max_magnitude)
}

fn main() {
    let numbers: Vec<SnailfishNumber> = io::stdin()
        .lock()
        .lines()
        .map(|line| SnailfishNumber::parse(&line.unwrap()))
        .collect();

    let max_magnitude = max_pairwise_magnitude(&numbers).unwrap();
    let sum = numbers.into_iter().reduce(|lhs, rhs| lhs + rhs).unwrap();

    println!("Part 1: {}", sum.magnitude());
    println!("Part 2: {}", max_magnitude);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1", SnailfishNumber::regular(1))]
    #[case(
        "[1,2]",
        SnailfishNumber::pair(SnailfishNumber::regular(1), SnailfishNumber::regular(2))
    )]
    #[case(
        "[[1,2],3]",
        SnailfishNumber::pair(
            SnailfishNumber::pair(SnailfishNumber::regular(1), SnailfishNumber::regular(2)),
            SnailfishNumber::regular(3)
        )
    )]
    #[case(
        "[[1,9],[8,5]]",
        SnailfishNumber::pair(
            SnailfishNumber::pair(SnailfishNumber::regular(1), SnailfishNumber::regular(9)),
            SnailfishNumber::pair(SnailfishNumber::regular(8), SnailfishNumber::regular(5))
        )
    )]
    fn test_parse(#[case] s: &str, #[case] expected: SnailfishNumber) {
        assert_eq!(SnailfishNumber::parse(s), expected);
    }

    #[rstest]
    #[case("[1,2]", "[[3,4],5]", "[[1,2],[[3,4],5]]")]
    #[case(
        "[[[[4,3],4],4],[7,[[8,4],9]]]",
        "[1,1]",
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
    )]
    #[case(
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
        "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
        "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
    )]
    fn test_addition(#[case] lhs: &str, #[case] rhs: &str, #[case] expected: &str) {
        let result = SnailfishNumber::parse(lhs) + SnailfishNumber::parse(rhs);
        assert_eq!(result, SnailfishNumber::parse(expected));
    }

    #[rstest]
    #[case("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]")]
    #[case("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]")]
    #[case("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]")]
    #[case(
        "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
    )]
    #[case("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]")]
    #[case(
        "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[0,10]]]]",
        "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
    )]
    fn test_reduction(#[case] given: &str, #[case] expected: &str) {
        assert_eq!(
            SnailfishNumber::parse(given),
            SnailfishNumber::parse(expected)
        );
    }

    #[rstest]
    #[case("[9,1]", 29)]
    #[case("[1,9]", 21)]
    #[case("[[9,1],[1,9]]", 129)]
    #[case("[[1,2],[[3,4],5]]", 143)]
    #[case("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384)]
    #[case("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445)]
    #[case("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791)]
    #[case("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137)]
    #[case("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488)]
    fn test_magnitude(#[case] given: &str, #[case] expected: usize) {
        assert_eq!(SnailfishNumber::parse(given).magnitude(), expected);
    }

    #[test]
    fn test_sum_example() {
        let numbers = vec![
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ];
        let sum = numbers
            .into_iter()
            .map(SnailfishNumber::parse)
            .reduce(|lhs, rhs| {
                println!("{} + {}", &lhs, &rhs);
                let result = lhs + rhs;
                println!("= {}", &result);
                result
            })
            .unwrap();
        assert_eq!(
            sum,
            SnailfishNumber::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        );
    }

    #[test]
    fn test_sum_and_magnitude_example() {
        let numbers = vec![
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ];
        let sum = numbers
            .into_iter()
            .map(SnailfishNumber::parse)
            .reduce(|lhs, rhs| {
                println!("{} + {}", &lhs, &rhs);
                let result = lhs + rhs;
                println!("= {}", &result);
                result
            })
            .unwrap();
        assert_eq!(
            sum,
            SnailfishNumber::parse("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
        );
        assert_eq!(sum.magnitude(), 4140);
    }

    #[test]
    fn test_max_pairwise_magnitude() {
        let numbers: Vec<_> = vec![
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ]
        .into_iter()
        .map(SnailfishNumber::parse)
        .collect();
        assert_eq!(max_pairwise_magnitude(&numbers), Some(3993));
    }
}
