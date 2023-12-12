use itertools::Itertools;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let heightmap: Vec<Vec<usize>> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let low_points = find_local_minimas_2d(&heightmap);

    let total_risk_level: usize = low_points
        .iter()
        .map(|low_point| low_point.height + 1)
        .sum();
    println!("Part 1: {}", total_risk_level);

    let top_three_basin_sizes: usize = low_points
        .iter()
        .map(|low_point| basin_size(&heightmap, &low_point))
        .sorted()
        .rev()
        .take(3)
        .product();
    println!("Part 2: {}", top_three_basin_sizes);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct LocalMinimum {
    pub pos: Position,
    pub height: usize,
}

fn adjacent_positions_2d<Row>(heightmap: &[Row], pos: Position) -> Vec<Position>
where
    Row: AsRef<[usize]>,
{
    let cols = heightmap.len();
    let rows = heightmap[0].as_ref().len();

    let mut adjacent_positions = Vec::with_capacity(4);
    if pos.x > 0 {
        adjacent_positions.push(Position {
            x: pos.x - 1,
            ..pos
        });
    }
    if pos.x < rows - 1 {
        adjacent_positions.push(Position {
            x: pos.x + 1,
            ..pos
        });
    }
    if pos.y > 0 {
        adjacent_positions.push(Position {
            y: pos.y - 1,
            ..pos
        });
    }
    if pos.y < cols - 1 {
        adjacent_positions.push(Position {
            y: pos.y + 1,
            ..pos
        });
    }
    adjacent_positions
}

fn min_adjacent_height_2d<Row>(heightmap: &[Row], pos: Position) -> Option<usize>
where
    Row: AsRef<[usize]>,
{
    adjacent_positions_2d(heightmap, pos)
        .into_iter()
        .map(|pos| heightmap[pos.y].as_ref()[pos.x])
        .min()
}

fn find_local_minimas_2d<Row>(heightmap: &[Row]) -> Vec<LocalMinimum>
where
    Row: AsRef<[usize]>,
{
    let cols = heightmap.len();
    let rows = heightmap[0].as_ref().len();

    (0..cols)
        .cartesian_product(0..rows)
        .filter_map(|(y, x)| {
            let pos = Position { x, y };
            let min_adjacent_height = min_adjacent_height_2d(heightmap, pos)?;
            let height = heightmap[y].as_ref()[x];
            if height < min_adjacent_height {
                Some(LocalMinimum { pos, height })
            } else {
                None
            }
        })
        .collect()
}

fn basin_size<Row>(heightmap: &[Row], low_point: &LocalMinimum) -> usize
where
    Row: AsRef<[usize]>,
{
    let cols = heightmap.len();
    let rows = heightmap[0].as_ref().len();

    let mut basin_locations: HashSet<Position> = HashSet::with_capacity(cols * rows);
    basin_locations.insert(low_point.pos);

    let mut queue: Vec<(Position, usize)> = Vec::new();
    queue.push((low_point.pos, low_point.height));

    while let Some((pos, height)) = queue.pop() {
        for adjacent_pos in adjacent_positions_2d(heightmap, pos) {
            let adjacent_height = heightmap[adjacent_pos.y].as_ref()[adjacent_pos.x];
            if adjacent_height >= height
                && adjacent_height < 9
                && basin_locations.insert(adjacent_pos)
            {
                queue.push((adjacent_pos, adjacent_height));
            }
        }
    }

    basin_locations.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const TEST_HEIGHTMAP: &[&[usize]] = &[
        &[2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
        &[3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
        &[9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
        &[8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
        &[9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
    ];

    #[test]
    fn test_find_local_minimas_2d() {
        assert_eq!(
            vec![
                LocalMinimum {
                    pos: Position { x: 1, y: 0 },
                    height: 1
                },
                LocalMinimum {
                    pos: Position { x: 9, y: 0 },
                    height: 0
                },
                LocalMinimum {
                    pos: Position { x: 2, y: 2 },
                    height: 5
                },
                LocalMinimum {
                    pos: Position { x: 6, y: 4 },
                    height: 5
                },
            ],
            find_local_minimas_2d(TEST_HEIGHTMAP)
        )
    }

    #[rstest]
    #[case(LocalMinimum{pos: Position{x:1, y:0}, height:1}, 3)]
    #[case(LocalMinimum{pos: Position{x:9, y:0}, height:0}, 9)]
    #[case(LocalMinimum{pos: Position{x:2, y:2}, height:5}, 14)]
    #[case(LocalMinimum{pos: Position{x:6, y:4}, height:5}, 9)]
    fn test_basin_size(#[case] low_point: LocalMinimum, #[case] expected_size: usize) {
        assert_eq!(expected_size, basin_size(TEST_HEIGHTMAP, &low_point));
    }
}
