use std::io::{self, BufRead};

fn main() {
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let total_corruption_score: usize = lines
        .iter()
        .flat_map(|s| check_syntax(s))
        .map(|e| score_corruption_error(&e))
        .sum();
    println!("Part 1: {}", total_corruption_score);

    let autocompletions_scores: Vec<usize> = lines
        .iter()
        .map(|s| check_syntax(s))
        .filter(|errors| !contains_corruption_error(&errors))
        .map(|errors| autocompletion_score(&errors))
        .collect();
    println!("Part 2: {:?}", median(&autocompletions_scores));
}

fn score_corruption_error(syntax_error: &SyntaxError) -> usize {
    match syntax_error {
        SyntaxError { was: Some(')'), .. } => 3,
        SyntaxError { was: Some(']'), .. } => 57,
        SyntaxError { was: Some('}'), .. } => 1197,
        SyntaxError { was: Some('>'), .. } => 25137,
        _ => 0,
    }
}

fn contains_corruption_error(syntax_errors: &[SyntaxError]) -> bool {
    syntax_errors.iter().any(|e| e.was.is_some())
}

fn autocompletion_score(syntax_errors: &[SyntaxError]) -> usize {
    syntax_errors.iter().fold(0, |cost, syntax_error| {
        cost * 5 + score_incompletion_error(syntax_error)
    })
}

fn score_incompletion_error(syntax_error: &SyntaxError) -> usize {
    match syntax_error {
        SyntaxError {
            was: None,
            expected: Some(')'),
            ..
        } => 1,
        SyntaxError {
            was: None,
            expected: Some(']'),
            ..
        } => 2,
        SyntaxError {
            was: None,
            expected: Some('}'),
            ..
        } => 3,
        SyntaxError {
            was: None,
            expected: Some('>'),
            ..
        } => 4,
        _ => 0,
    }
}

fn median(xs: &[usize]) -> Option<usize> {
    if xs.is_empty() {
        return None;
    }

    let mut xs = xs.to_vec();
    xs.sort_unstable();

    if xs.len() % 2 == 0 {
        Some((xs[xs.len() / 2 - 1] + xs[xs.len() / 2]) / 2)
    } else {
        Some(xs[xs.len() / 2])
    }
}

#[derive(Debug, PartialEq)]
struct SyntaxError {
    col: usize,
    expected: Option<char>,
    was: Option<char>,
}

fn check_syntax(line: &str) -> Vec<SyntaxError> {
    let mut errors = Vec::new();

    let mut stack = Vec::with_capacity(line.len() / 2);

    for (col, c) in line.chars().enumerate() {
        if matches!(c, '(' | '[' | '{' | '<') {
            stack.push(c);
            continue;
        }

        match (stack.pop(), c) {
            (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>') => {}
            (Some('('), _) => {
                errors.push(SyntaxError {
                    col,
                    expected: Some(')'),
                    was: Some(c),
                });
            }
            (Some('['), _) => {
                errors.push(SyntaxError {
                    col,
                    expected: Some(']'),
                    was: Some(c),
                });
            }
            (Some('{'), _) => {
                errors.push(SyntaxError {
                    col,
                    expected: Some('}'),
                    was: Some(c),
                });
            }
            (Some('<'), _) => {
                errors.push(SyntaxError {
                    col,
                    expected: Some('>'),
                    was: Some(c),
                });
            }
            (None, _) => {
                errors.push(SyntaxError {
                    col,
                    expected: None,
                    was: Some(c),
                });
            }
            _ => unreachable!(),
        }
    }

    for (i, c) in stack.into_iter().rev().enumerate() {
        let col = line.len() + i;
        match c {
            '(' => {
                errors.push(SyntaxError {
                    col,
                    expected: Some(')'),
                    was: None,
                });
            }
            '[' => {
                errors.push(SyntaxError {
                    col,
                    expected: Some(']'),
                    was: None,
                });
            }
            '{' => {
                errors.push(SyntaxError {
                    col,
                    expected: Some('}'),
                    was: None,
                });
            }
            '<' => {
                errors.push(SyntaxError {
                    col,
                    expected: Some('>'),
                    was: None,
                });
            }
            _ => unreachable!(),
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("()", vec![])]
    #[case("[]", vec![])]
    #[case("{}", vec![])]
    #[case("<>", vec![])]
    #[case("([])", vec![])]
    #[case("{()()()}", vec![])]
    #[case("<([{}])>", vec![])]
    #[case("[<>({}){}[([])<>]]", vec![])]
    #[case("(((((((((())))))))))", vec![])]
    #[case("<(", vec![
            SyntaxError{col: 2, expected: Some(')'), was: None},
            SyntaxError{col: 3, expected: Some('>'), was: None}
        ])]
    #[case("(()", vec![SyntaxError{col: 3, expected: Some(')'), was: None}])]
    #[case("())", vec![SyntaxError{col: 2, expected: None, was: Some(')')}])]
    #[case("(]", vec![SyntaxError{col: 1, expected: Some(')'), was: Some(']')}])]
    #[case("{()()()>",vec![SyntaxError{col: 7, expected: Some('}'), was: Some('>')}])]
    #[case("(((()))}", vec![SyntaxError{col: 7, expected: Some(')'), was: Some('}')}])]
    #[case("<([]){()}[{}])", vec![SyntaxError{col: 13, expected: Some('>'), was: Some(')')}])]
    fn test_syntactically_valid(#[case] line: &str, #[case] expected: Vec<SyntaxError>) {
        assert_eq!(expected, check_syntax(line));
    }
}
