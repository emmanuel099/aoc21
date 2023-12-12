use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    pub fn energy(&self) -> usize {
        match self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1_000,
        }
    }

    pub fn target_room(&self) -> usize {
        match self {
            Self::Amber => 0,
            Self::Bronze => 1,
            Self::Copper => 2,
            Self::Desert => 3,
        }
    }
}

fn abs_diff(x: usize, y: usize) -> usize {
    if x < y {
        y - x
    } else {
        x - y
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State<const DEPTH: usize> {
    hallway: [Option<Amphipod>; 11],
    side_rooms: [Vec<Amphipod>; 4],
    total_energy: usize,
}

impl<const DEPTH: usize> State<DEPTH> {
    pub fn is_done(&self) -> bool {
        (0..4).all(|room| !self.room_needs_move(room) && self.room_is_full(room))
    }

    pub fn room_needs_move(&self, room: usize) -> bool {
        self.side_rooms[room]
            .iter()
            .any(|amphipod| amphipod.target_room() != room)
    }

    pub fn room_is_full(&self, room: usize) -> bool {
        self.side_rooms[room].len() == DEPTH
    }

    pub fn move_room_to_hallway(mut self, from: usize, to: usize) -> State<DEPTH> {
        assert!(!self.side_rooms[from].is_empty());
        assert!(self.hallway[to].is_none());

        let amphipod = self.side_rooms[from].pop().unwrap();
        self.hallway[to] = Some(amphipod);

        let steps_up = DEPTH - self.side_rooms[from].len();
        let room_x = 2 + from * 2;
        let steps_horizontal = abs_diff(room_x, to);
        let steps = steps_up + steps_horizontal;
        let energy = steps * amphipod.energy();
        self.total_energy += energy;

        /*println!(
            "Move {:?} from room {} to hallway {} (took {} energy)",
            amphipod, from, to, energy
        );*/

        self
    }

    pub fn move_hallway_to_room(mut self, from: usize, to: usize) -> State<DEPTH> {
        assert!(self.hallway[from].is_some());
        assert!(self.side_rooms[to].len() < DEPTH);
        assert!(!self.room_needs_move(to));

        let amphipod = self.hallway[from].take().unwrap();
        self.side_rooms[to].push(amphipod);

        let steps_down = DEPTH - self.side_rooms[to].len() + 1;
        let room_x = 2 + to * 2;
        let steps_horizontal = abs_diff(from, room_x);
        let steps = steps_horizontal + steps_down;
        let energy = steps * amphipod.energy();
        self.total_energy += energy;

        /*println!(
            "Move {:?} from hallway {} to room {} (took {} energy)",
            amphipod, from, to, energy
        );*/

        self
    }
}

fn organize<const DEPTH: usize>(initial_state: State<DEPTH>) -> usize {
    let mut next_states = vec![initial_state];

    let mut min_energy = usize::MAX; // TODO find useful upper bound

    let mut visited_states = HashSet::new();

    while let Some(state) = next_states.pop() {
        if state.is_done() {
            // println!("DONE {} {}", state.total_energy, min_energy);
            min_energy = std::cmp::min(min_energy, state.total_energy);
        }

        if state.total_energy >= min_energy {
            continue;
        }

        if !visited_states.insert(state.clone()) {
            // println!("SAME STATE VISITED TWICE!!!");
            continue;
        }

        // hallway to target room
        for x in 0..11 {
            if let Some(amphipod) = state.hallway[x] {
                let target_room = amphipod.target_room();
                let target_x = 2 + target_room * 2;

                if state.room_is_full(target_room) || state.room_needs_move(target_room) {
                    continue;
                }

                let hallway_is_free = if x > target_x {
                    (target_x..x).all(|x| state.hallway[x].is_none())
                } else {
                    ((x + 1)..=target_x).all(|x| state.hallway[x].is_none())
                };

                if hallway_is_free {
                    next_states.push(
                        state
                            .clone()
                            .move_hallway_to_room(x, amphipod.target_room()),
                    );
                }
            }
        }

        // room to target room/hallway
        for room in 0..4 {
            if !state.room_needs_move(room) {
                continue;
            }

            let current_x = 2 + room * 2;

            // move left hallway
            for x in (0..current_x).rev() {
                if matches!(x, 2 | 4 | 6 | 8) {
                    continue;
                }
                if state.hallway[x].is_some() {
                    break;
                }
                next_states.push(state.clone().move_room_to_hallway(room, x));
            }

            // move right hallway
            for x in (current_x + 1)..11 {
                if matches!(x, 2 | 4 | 6 | 8) {
                    continue;
                }
                if state.hallway[x].is_some() {
                    break;
                }
                next_states.push(state.clone().move_room_to_hallway(room, x));
            }
        }
    }

    min_energy
}

fn part1() {
    let example = State::<2> {
        hallway: [None; 11],
        side_rooms: [
            vec![Amphipod::Amber, Amphipod::Bronze],
            vec![Amphipod::Desert, Amphipod::Copper],
            vec![Amphipod::Copper, Amphipod::Bronze],
            vec![Amphipod::Amber, Amphipod::Desert],
        ],
        total_energy: 0,
    };

    let input = State::<2> {
        hallway: [None; 11],
        side_rooms: [
            vec![Amphipod::Copper, Amphipod::Desert],
            vec![Amphipod::Amber, Amphipod::Amber],
            vec![Amphipod::Bronze, Amphipod::Copper],
            vec![Amphipod::Bronze, Amphipod::Desert],
        ],
        total_energy: 0,
    };

    let required_energy = organize(input);
    println!("Part 1: {}", required_energy);
}

fn part2() {
    let example = State::<4> {
        hallway: [None; 11],
        side_rooms: [
            vec![
                Amphipod::Amber,
                Amphipod::Desert,
                Amphipod::Desert,
                Amphipod::Bronze,
            ],
            vec![
                Amphipod::Desert,
                Amphipod::Bronze,
                Amphipod::Copper,
                Amphipod::Copper,
            ],
            vec![
                Amphipod::Copper,
                Amphipod::Amber,
                Amphipod::Bronze,
                Amphipod::Bronze,
            ],
            vec![
                Amphipod::Amber,
                Amphipod::Copper,
                Amphipod::Amber,
                Amphipod::Desert,
            ],
        ],
        total_energy: 0,
    };

    let input = State::<4> {
        hallway: [None; 11],
        side_rooms: [
            vec![
                Amphipod::Copper,
                Amphipod::Desert,
                Amphipod::Desert,
                Amphipod::Desert,
            ],
            vec![
                Amphipod::Amber,
                Amphipod::Bronze,
                Amphipod::Copper,
                Amphipod::Amber,
            ],
            vec![
                Amphipod::Bronze,
                Amphipod::Amber,
                Amphipod::Bronze,
                Amphipod::Copper,
            ],
            vec![
                Amphipod::Bronze,
                Amphipod::Copper,
                Amphipod::Amber,
                Amphipod::Desert,
            ],
        ],
        total_energy: 0,
    };

    let required_energy = organize(input);
    println!("Part 2: {}", required_energy);
}

fn main() {
    part1();
    part2();
}
