#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Acceleration {
    pub horizontal: isize,
    pub vertical: isize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Velocity {
    pub horizontal: isize,
    pub vertical: isize,
}

impl Velocity {
    pub fn accelerate(mut self, accel: Acceleration) -> Velocity {
        self.horizontal += accel.horizontal;
        self.vertical += accel.vertical;
        self
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn step(mut self, vel: Velocity) -> Position {
        self.x += vel.horizontal;
        self.y += vel.vertical;
        self
    }

    pub fn is_below(&self, other: &Position) -> bool {
        self.y < other.y
    }
}

struct Area {
    pub top_left: Position,
    pub bottom_right: Position,
}

impl Area {
    pub fn contains(&self, pos: &Position) -> bool {
        pos.x >= self.top_left.x
            && pos.x <= self.bottom_right.x
            && pos.y >= self.bottom_right.y
            && pos.y <= self.top_left.y
    }
}

fn reaches_target_with_max_height(
    init_pos: Position,
    init_vel: Velocity,
    target: &Area,
) -> Option<(isize, Position)> {
    let mut pos = init_pos;
    let mut vel = init_vel;
    let mut max_height = pos.y;

    loop {
        if target.contains(&pos) {
            break Some((max_height, pos));
        }
        if pos.is_below(&target.bottom_right) {
            break None;
        }

        let accel = match vel.horizontal.cmp(&0) {
            std::cmp::Ordering::Equal => Acceleration {
                horizontal: 0,
                vertical: -1,
            },
            std::cmp::Ordering::Greater => Acceleration {
                horizontal: -1,
                vertical: -1,
            },
            std::cmp::Ordering::Less => Acceleration {
                horizontal: 1,
                vertical: -1,
            },
        };

        pos = pos.step(vel);
        vel = vel.accelerate(accel);

        if pos.y > max_height {
            max_height = pos.y;
        }
    }
}

fn find_best_initital_velocity(target: &Area) -> Option<(isize, Velocity)> {
    let mut best = None;

    for dx in -100..100 {
        for dy in -100..100 {
            let vel = Velocity {
                horizontal: dx,
                vertical: dy,
            };
            if let Some((height, _)) =
                reaches_target_with_max_height(Position::default(), vel, target)
            {
                best = match best {
                    Some((best_height, _)) if height > best_height => Some((height, vel)),
                    None => Some((height, vel)),
                    _ => best,
                };
            }
        }
    }

    best
}

fn count_initital_velocities_in_range(target: &Area) -> usize {
    let mut count = 0;

    for dx in -500..500 {
        for dy in -500..500 {
            let vel = Velocity {
                horizontal: dx,
                vertical: dy,
            };
            if reaches_target_with_max_height(Position::default(), vel, target).is_some() {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let target = Area {
        top_left: Position { x: 137, y: -73 },
        bottom_right: Position { x: 171, y: -98 },
    };
    println!("Part 1: {:?}", find_best_initital_velocity(&target));
    println!("Part 2: {}", count_initital_velocities_in_range(&target));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instance() {
        let target = Area {
            top_left: Position { x: 137, y: -73 },
            bottom_right: Position { x: 171, y: -98 },
        };
        assert_eq!(
            find_best_initital_velocity(&target),
            Some((
                4753,
                Velocity {
                    horizontal: 17,
                    vertical: 97
                }
            ))
        );
        assert_eq!(count_initital_velocities_in_range(&target), 1546);
    }

    #[test]
    fn test_example() {
        let target = Area {
            top_left: Position { x: 20, y: -5 },
            bottom_right: Position { x: 30, y: -10 },
        };
        assert_eq!(
            find_best_initital_velocity(&target),
            Some((
                45,
                Velocity {
                    horizontal: 6,
                    vertical: 9
                }
            ))
        );
        assert_eq!(count_initital_velocities_in_range(&target), 112);
    }

    #[test]
    fn test_reaches_target_with_max_height1() {
        let target = Area {
            top_left: Position { x: 20, y: -5 },
            bottom_right: Position { x: 30, y: -10 },
        };
        let result = reaches_target_with_max_height(
            Position::default(),
            Velocity {
                horizontal: 7,
                vertical: 2,
            },
            &target,
        );
        assert_eq!(result, Some((3, Position { x: 28, y: -7 })));
    }

    #[test]
    fn test_reaches_target_with_max_height2() {
        let target = Area {
            top_left: Position { x: 20, y: -5 },
            bottom_right: Position { x: 30, y: -10 },
        };
        let result = reaches_target_with_max_height(
            Position::default(),
            Velocity {
                horizontal: 6,
                vertical: 3,
            },
            &target,
        );
        assert_eq!(result, Some((6, Position { x: 21, y: -9 })));
    }

    #[test]
    fn test_reaches_target_with_max_height3() {
        let target = Area {
            top_left: Position { x: 20, y: -5 },
            bottom_right: Position { x: 30, y: -10 },
        };
        let result = reaches_target_with_max_height(
            Position::default(),
            Velocity {
                horizontal: 9,
                vertical: 0,
            },
            &target,
        );
        assert_eq!(result, Some((0, Position { x: 30, y: -6 })));
    }

    #[test]
    fn test_reaches_target_with_max_height4() {
        let target = Area {
            top_left: Position { x: 20, y: -5 },
            bottom_right: Position { x: 30, y: -10 },
        };
        let result = reaches_target_with_max_height(
            Position::default(),
            Velocity {
                horizontal: 17,
                vertical: -4,
            },
            &target,
        );
        assert_eq!(result, None);
    }
}
