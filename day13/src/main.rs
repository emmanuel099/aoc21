use std::{
    cmp,
    collections::HashSet,
    io::{self, BufRead},
    str::FromStr,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("invalid point format, expected 'x,y'")]
    InvalidPointFormat,
    #[error("invalid instruction format")]
    InvalidInstructionFormat,
    #[error("invalid number")]
    InvalidNumber(#[from] std::num::ParseIntError),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    pub x: usize,
    pub y: usize,
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Point, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(ParseError::InvalidPointFormat)?;
        let x = x.parse()?;
        let y = y.parse()?;
        Ok(Point { x, y })
    }
}

enum Instruction {
    FoldHorizontal { y: usize },
    FoldVertical { x: usize },
}

impl Instruction {
    pub fn transform(&self, points: &HashSet<Point>) -> HashSet<Point> {
        match self {
            Self::FoldHorizontal { y } => points
                .iter()
                .copied()
                .map(|p| {
                    if p.y > *y {
                        let dy = p.y - y;
                        Point { y: y - dy, ..p }
                    } else {
                        p
                    }
                })
                .collect(),
            Self::FoldVertical { x } => points
                .iter()
                .copied()
                .map(|p| {
                    if p.x > *x {
                        let dx = p.x - x;
                        Point { x: x - dx, ..p }
                    } else {
                        p
                    }
                })
                .collect(),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Instruction, Self::Err> {
        let (inst, n) = s
            .split_once('=')
            .ok_or(ParseError::InvalidInstructionFormat)?;
        let n = n.parse()?;
        match inst {
            "fold along y" => Ok(Instruction::FoldHorizontal { y: n }),
            "fold along x" => Ok(Instruction::FoldVertical { x: n }),
            _ => Err(ParseError::InvalidInstructionFormat),
        }
    }
}

fn print_code(points: &HashSet<Point>) {
    let size = points.iter().fold((0, 0), |(w, h), p| {
        (cmp::max(w, p.x + 1), cmp::max(h, p.y + 1))
    });

    for y in 0..size.1 {
        for x in 0..size.0 {
            if points.contains(&Point { x, y }) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!("");
    }
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|s| s.unwrap()).collect();
    let parts: Vec<_> = lines.split(|line| line.is_empty()).collect();
    let points: HashSet<Point> = parts[0].iter().map(|s| s.parse().unwrap()).collect();
    let instructions: Vec<Instruction> = parts[1].iter().map(|s| s.parse().unwrap()).collect();

    println!("Part 1: {}", instructions[0].transform(&points).len());

    let folded_points = instructions
        .iter()
        .fold(points, |points, inst| inst.transform(&points));
    println!("Part 2: {}", folded_points.len());

    print_code(&folded_points);
}
