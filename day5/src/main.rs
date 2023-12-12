use std::{
    cmp::Ordering,
    collections::HashMap,
    io::{self, BufRead},
    str::FromStr,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("invalid point format, expected 'x,y'")]
    InvalidPointFormat,
    #[error("invalid line segment format, expected 'x1,y1 -> x2,y2'")]
    InvalidLineSegmentFormat,
    #[error("invalid number")]
    InvalidNumber(#[from] std::num::ParseIntError),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    pub fn translate(&self, dx: isize, dy: isize) -> Position {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

impl FromStr for Position {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Position, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(ParseError::InvalidPointFormat)?;
        let x = x.parse()?;
        let y = y.parse()?;
        Ok(Position { x, y })
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
struct LineSegment {
    start: Position,
    end: Position,
}

impl LineSegment {
    pub fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    pub fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    pub fn positions(&self) -> LineInterpolator {
        LineInterpolator::new_end_inclusive(self.start, self.end)
    }
}

impl FromStr for LineSegment {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<LineSegment, Self::Err> {
        let (start, end) = s
            .split_once(" -> ")
            .ok_or(ParseError::InvalidLineSegmentFormat)?;
        let start = start.parse()?;
        let end = end.parse()?;
        Ok(LineSegment { start, end })
    }
}

struct LineInterpolator {
    curr: Position,
    end: Position,
    end_inclusive: bool,
}

impl LineInterpolator {
    fn new_end_inclusive(start: Position, end: Position) -> LineInterpolator {
        Self {
            curr: start,
            end,
            end_inclusive: true,
        }
    }
}

impl Iterator for LineInterpolator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == self.end {
            if self.end_inclusive {
                self.end_inclusive = false;
                return Some(self.end);
            }
            return None;
        }

        let pos = self.curr;
        let dx = match pos.x.cmp(&self.end.x) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        let dy = match pos.y.cmp(&self.end.y) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        self.curr = self.curr.translate(dx, dy);
        Some(pos)
    }
}

fn count_overlapping_positions(positions: &[Position]) -> usize {
    let diagram =
        positions
            .iter()
            .fold(HashMap::with_capacity(positions.len()), |mut map, &pos| {
                let count = map.entry(pos).or_insert(0);
                *count += 1;
                map
            });

    diagram.values().filter(|&count| *count > 1).count()
}

fn main() {
    let lines: Vec<LineSegment> = io::stdin()
        .lock()
        .lines()
        .filter_map(|s| s.unwrap().parse().ok())
        .collect();

    let positions_part1: Vec<Position> = lines
        .iter()
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .flat_map(|line| line.positions().collect::<Vec<_>>())
        .collect();
    println!("Part 1: {}", count_overlapping_positions(&positions_part1));

    let positions_part2: Vec<Position> = lines
        .iter()
        .flat_map(|line| line.positions().collect::<Vec<_>>())
        .collect();
    println!("Part 2: {}", count_overlapping_positions(&positions_part2));
}
