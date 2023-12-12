use std::cmp;
use std::collections::HashMap;

const BOARD_SIZE: usize = 10;

trait Roll {
    fn roll(&mut self) -> usize;
}

struct DeterministicDice {
    next: usize,
    limit: usize,
    roll_count: usize,
}

impl DeterministicDice {
    pub fn new(limit: usize) -> DeterministicDice {
        Self {
            next: 1,
            limit,
            roll_count: 0,
        }
    }
}

impl Roll for DeterministicDice {
    fn roll(&mut self) -> usize {
        let result = self.next;
        self.next += 1;
        if self.next > self.limit {
            self.next = 1;
        }
        self.roll_count += 1;
        result
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Player {
    pos: usize,
    total_score: usize,
}

impl Player {
    pub fn new(pos: usize) -> Player {
        Self {
            pos: Self::cirular_board_position(pos),
            total_score: 0,
        }
    }

    pub fn play<Dice: Roll>(&mut self, dice: &mut Dice) {
        let n: usize = (0..3).map(|_| dice.roll()).sum();
        self.moves(n);
    }

    pub fn moves(&mut self, n: usize) {
        self.pos = Self::cirular_board_position(self.pos + n);
        self.total_score += self.pos;
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn total_score(&self) -> usize {
        self.total_score
    }

    fn cirular_board_position(pos: usize) -> usize {
        (pos - 1) % BOARD_SIZE + 1
    }
}

struct GameResult {
    winner: Player,
    loser: Player,
}

fn play_game<Dice: Roll>(
    dice: &mut Dice,
    mut player1: Player,
    mut player2: Player,
    winning_score: usize,
) -> GameResult {
    loop {
        player1.play(dice);
        if player1.total_score() >= winning_score {
            break GameResult {
                winner: player1,
                loser: player2,
            };
        }

        player2.play(dice);
        if player1.total_score() >= winning_score {
            break GameResult {
                winner: player2,
                loser: player1,
            };
        }
    }
}

fn quantum() -> Vec<usize> {
    let mut q = Vec::new();
    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                q.push(i + j + k);
            }
        }
    }
    q
}

fn play_dirac_game(player1: Player, player2: Player, winning_score: usize) -> (usize, usize) {
    let mut memoization: HashMap<(Player, Player, usize), (usize, usize)> = HashMap::new();

    let mut total_player1_wins = 0;
    let mut total_player2_wins = 0;

    for dice in quantum() {
        let (player1_wins, player2_wins) =
            play_dirac_game_rec(&mut memoization, player1, player2, dice, winning_score);
        total_player1_wins += player1_wins;
        total_player2_wins += player2_wins;
    }

    (total_player1_wins, total_player2_wins)
}

fn play_dirac_game_rec(
    memoization: &mut HashMap<(Player, Player, usize), (usize, usize)>,
    mut player: Player,
    other: Player,
    dice: usize,
    winning_score: usize,
) -> (usize, usize) {
    if let Some(&(player_wins, other_wins)) = memoization.get(&(player, other, dice)) {
        return (player_wins, other_wins);
    }

    let initial_player = player; // memoize the unmodified player

    player.moves(dice);
    if player.total_score() >= winning_score {
        return (1, 0);
    }

    let mut total_player_wins = 0;
    let mut total_other_wins = 0;

    for dice in quantum() {
        let (other_wins, player_wins) =
            play_dirac_game_rec(memoization, other, player, dice, winning_score);
        total_player_wins += player_wins;
        total_other_wins += other_wins;
    }

    memoization.insert(
        (initial_player, other, dice),
        (total_player_wins, total_other_wins),
    );

    (total_player_wins, total_other_wins)
}

fn main() {
    // Example
    //let player1 = Player::new(4);
    //let player2 = Player::new(8);

    // Puzzle input
    let player1 = Player::new(8);
    let player2 = Player::new(10);

    let mut dice = DeterministicDice::new(100);
    let GameResult { loser, .. } = play_game(&mut dice, player1, player2, 1000);
    println!("Part 1: {}", loser.total_score() * dice.roll_count);

    let (player1_wins, player2_wins) = play_dirac_game(player1, player2, 21);
    println!("Part 2: {}", cmp::max(player1_wins, player2_wins));
}
