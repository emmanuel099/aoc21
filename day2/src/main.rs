use std::{
    io::{self, BufRead},
    str::FromStr,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("invalid format, expected '<command> <steps>' but was '{0}'")]
    InvalidFormat(String),
    #[error("invalid number")]
    InvalidNumber(#[from] std::num::ParseIntError),
    #[error("invalid command '{0}'")]
    InvalidCommand(String),
}

enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        if let Some((cmd, x)) = input.split_once(' ') {
            let x: i64 = x.trim().parse()?;
            match cmd {
                "forward" => Ok(Self::Forward(x)),
                "down" => Ok(Self::Down(x)),
                "up" => Ok(Self::Up(x)),
                _ => Err(ParseError::InvalidCommand(cmd.to_owned())),
            }
        } else {
            Err(ParseError::InvalidFormat(input.to_owned()))
        }
    }
}

#[derive(Debug, PartialEq, Default)]
struct Position {
    pub horizontal: i64,
    pub depth: i64,
}

fn execute_course_part1(initial_pos: Position, course: &[Command]) -> Position {
    course.iter().fold(initial_pos, |pos, cmd| match cmd {
        Command::Forward(x) => Position {
            horizontal: pos.horizontal + x,
            ..pos
        },
        Command::Down(x) => Position {
            depth: pos.depth + x,
            ..pos
        },
        Command::Up(x) => Position {
            depth: pos.depth - x,
            ..pos
        },
    })
}

#[derive(Debug, PartialEq, Default)]
struct PositionWithAim {
    pub horizontal: i64,
    pub depth: i64,
    pub aim: i64,
}

fn execute_course_part2(initial_pos: PositionWithAim, course: &[Command]) -> PositionWithAim {
    course.iter().fold(initial_pos, |pos, cmd| match cmd {
        Command::Forward(x) => PositionWithAim {
            horizontal: pos.horizontal + x,
            depth: pos.depth + pos.aim * x,
            ..pos
        },
        Command::Down(x) => PositionWithAim {
            aim: pos.aim + x,
            ..pos
        },
        Command::Up(x) => PositionWithAim {
            aim: pos.aim - x,
            ..pos
        },
    })
}

fn main() {
    let course: Vec<Command> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let final_pos1 = execute_course_part1(Position::default(), &course);
    println!("Part 1: {}", final_pos1.horizontal * final_pos1.depth);

    let final_pos2 = execute_course_part2(PositionWithAim::default(), &course);
    println!("Part 2: {}", final_pos2.horizontal * final_pos2.depth);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_course_part1() {
        use Command::*;
        let course = vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
        let pos = execute_course_part1(Position::default(), &course);
        assert_eq!(
            pos,
            Position {
                horizontal: 15,
                depth: 10,
            }
        );
    }

    #[test]
    fn test_execute_course_part2() {
        use Command::*;
        let course = vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
        let pos = execute_course_part2(PositionWithAim::default(), &course);
        assert_eq!(
            pos,
            PositionWithAim {
                horizontal: 15,
                depth: 60,
                aim: 10,
            }
        );
    }
}
