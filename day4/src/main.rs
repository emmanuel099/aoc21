use itertools::Itertools;
use std::{
    io::{self, BufRead},
    str::FromStr,
};

#[derive(Clone)]
struct Board<const ROWS: usize, const COLS: usize> {
    numbers: [[Option<usize>; ROWS]; COLS],
    // The following is just a small optimization for won()
    marks_per_row: [usize; ROWS],
    marks_per_col: [usize; COLS],
}

impl<const ROWS: usize, const COLS: usize> Board<ROWS, COLS> {
    fn new(numbers: [[Option<usize>; ROWS]; COLS]) -> Self {
        Self {
            numbers,
            marks_per_row: [0; ROWS],
            marks_per_col: [0; COLS],
        }
    }

    pub fn from_lines(lines: &[BoardLine]) -> Result<Self, &'static str> {
        let mut numbers = [[None; ROWS]; COLS];

        if lines.len() != COLS {
            return Err("Wrong number of colums");
        }

        for (col, line) in lines.iter().enumerate() {
            if line.len() != ROWS {
                return Err("Wrong number of rows");
            }
            for (row, &number) in line.as_slice().iter().enumerate() {
                numbers[col][row] = Some(number);
            }
        }

        Ok(Self::new(numbers))
    }

    pub fn mark(&mut self, number: usize) {
        (0..COLS).cartesian_product(0..ROWS).for_each(|(col, row)| {
            if self.numbers[col][row] == Some(number) {
                self.numbers[col][row] = None;
                self.marks_per_row[row] += 1;
                self.marks_per_col[col] += 1;
            }
        });
    }

    pub fn won(&self) -> bool {
        self.any_row_done() || self.any_col_done()
    }

    pub fn sum_of_unmarked_numbers(&self) -> usize {
        self.numbers
            .map(|row| row.iter().filter_map(|&n| n).sum())
            .iter()
            .sum()
    }

    fn any_row_done(&self) -> bool {
        self.marks_per_row.iter().any(|&marks| marks == ROWS)
    }

    fn any_col_done(&self) -> bool {
        self.marks_per_col.iter().any(|&marks| marks == COLS)
    }
}

struct BoardLine {
    numbers: Vec<usize>,
}

impl BoardLine {
    pub fn as_slice(&self) -> &[usize] {
        &self.numbers[..]
    }

    pub fn len(&self) -> usize {
        self.numbers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.numbers.is_empty()
    }
}

impl FromStr for BoardLine {
    type Err = ();

    fn from_str(input: &str) -> Result<BoardLine, Self::Err> {
        let numbers = input
            .split_whitespace()
            .filter_map(|n| n.parse::<usize>().ok())
            .collect();
        Ok(BoardLine { numbers })
    }
}

fn play_until_first_win<const ROWS: usize, const COLS: usize>(
    mut boards: Vec<Board<ROWS, COLS>>,
    random_numbers: &[usize],
) -> Option<usize> {
    for &number in random_numbers {
        boards.iter_mut().for_each(|board| board.mark(number));
        if let Some(winner) = boards.iter().filter(|board| board.won()).next() {
            let sum = winner.sum_of_unmarked_numbers();
            let final_score = sum * number;
            return Some(final_score);
        }
    }
    None
}

fn play_until_last_win<const ROWS: usize, const COLS: usize>(
    mut boards: Vec<Board<ROWS, COLS>>,
    random_numbers: &[usize],
) -> Option<usize> {
    for &number in random_numbers {
        boards.iter_mut().for_each(|board| board.mark(number));
        if boards.len() == 1 && boards[0].won() {
            let sum = boards[0].sum_of_unmarked_numbers();
            let final_score = sum * number;
            return Some(final_score);
        }
        boards.retain(|board| !board.won());
    }
    None
}

fn main() {
    const GRID_SIZE: usize = 5;

    let lines: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let random_numbers: Vec<_> = lines[0]
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    let all_board_lines: Vec<_> = lines
        .iter()
        .skip(1)
        .map(|line| line.parse::<BoardLine>().unwrap())
        .filter(|line| !line.is_empty())
        .collect();

    let boards: Vec<Board<GRID_SIZE, GRID_SIZE>> = all_board_lines
        .chunks(GRID_SIZE)
        .map(|board_lines| Board::from_lines(board_lines).unwrap())
        .collect();

    if let Some(final_score) = play_until_first_win(boards.clone(), &random_numbers) {
        println!("Part 1: {}", final_score);
    } else {
        println!("Part 1: No winner!");
    }

    if let Some(final_score) = play_until_last_win(boards, &random_numbers) {
        println!("Part 2: {}", final_score);
    } else {
        println!("Part 2: No winner!");
    }
}
