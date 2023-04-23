use std::collections::HashMap;

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Player {
    score: u128,
    position: u128,
}

impl Player {
    fn with_position(position: u128) -> Self {
        Self {
            position: position - 1,
            ..Default::default()
        }
    }
    fn win(&self) -> bool {
        self.score >= 1000
    }

    fn take_move(&mut self, moves: u128) {
        self.position = (self.position + moves) % 10;
        self.score += self.position + 1;
    }
}

struct Dice {
    iterator: std::iter::Cycle<std::ops::RangeInclusive<u128>>,
    roll_count: u128,
}

impl Iterator for Dice {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        self.roll_count += 1;
        self.iterator.next()
    }
}

impl Default for Dice {
    fn default() -> Self {
        Dice {
            iterator: (1..=100).cycle(),
            roll_count: 0,
        }
    }
}

fn part1(p1_start: u128, p2_start: u128) -> Option<u128> {
    let mut dice: Dice = Default::default();
    let mut player1 = Player::with_position(p1_start);
    let mut player2 = Player::with_position(p2_start);

    loop {
        let moves = dice.next()? + dice.next()? + dice.next()?;
        player1.take_move(moves);
        if player1.win() {
            break;
        }

        let moves = dice.next()? + dice.next()? + dice.next()?;
        player2.take_move(moves);
        if player2.win() {
            break;
        }
    }
    let loser = dbg!(player1.score.min(player2.score));
    let rolls = dice.roll_count;

    Some(loser * rolls)
}

#[test]
fn part1_works() {
    assert_eq!(part1(4, 8), Some(739785))
}

#[derive(Clone, Copy)]
enum Freq {
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl From<Freq> for u128 {
    fn from(value: Freq) -> Self {
        match value {
            Freq::Three => 1,
            Freq::Four => 3,
            Freq::Five => 6,
            Freq::Six => 7,
            Freq::Seven => 6,
            Freq::Eight => 3,
            Freq::Nine => 3,
        }
    }
}

const ROLLS: [Freq; 7] = [Freq::Three, Freq::Four, Freq::Five, Freq::Six, Freq::Seven, Freq::Eight, Freq::Nine];

type GameHash = HashMap<(Player, Player), u128>;

fn simulate(games: GameHash, player1_turn: bool) -> (GameHash, u128) {
    let mut next_games = GameHash::default();
    let mut victories = 0;

    for (game, count) in games.into_iter() {
        for roll in ROLLS {
            let mut game = game.clone();
            if player1_turn {
                game.0.take_move(roll.into());
            } else {
                game.1.take_move(roll.into());
            }
            let new_count: u128 = count * Into::<u128>::into(roll);

            if game.0.win() || game.1.win() {
                victories += new_count;
            } else {
                *next_games.entry(game).or_default() += new_count;
            }
        }
    }

    (next_games, victories)
}

fn part2(p1_start: u128, p2_start: u128) -> u128 {
    let mut games = GameHash::default();
    games.insert((Player::with_position(p1_start), Player::with_position(p2_start)), 1);
    let mut victory_array = (0, 0);
    let mut player1_turn = true;

    while !games.is_empty() {
        let (new_games, victories) = simulate(games, player1_turn);

        if player1_turn {
            victory_array.0 += victories;
        } else {
            victory_array.1 += victories;
        }
        player1_turn = !player1_turn;
        games = new_games;
        break;
    }

    victory_array.0.max(victory_array.1)
}


#[test]
fn part2_works() {
    assert_eq!(part2(4, 8), 444356092776315)
}

fn main() {
    let part1 = part1(3, 10).unwrap();
    println!("part1: {part1}");

    let part2 = part2(3, 10);
    println!("part2: {part2}");
}

