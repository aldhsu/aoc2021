use crate::{board::Board, Error};

pub struct Game {
    calls: Vec<u32>,
    boards: Vec<Board>,
}

impl std::str::FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (calls, rest) = s
            .split_once("\n\n")
            .ok_or_else(|| Error::ParseError("couldn't get moves".into()))?;
        let boards = rest
            .split("\n\n")
            .map(|board| board.parse())
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(Self {
            calls: calls
                .split(',')
                .map(|call| {
                    call.parse()
                        .map_err(|_| Error::ParseError("can't read call".into()))
                })
                .collect::<Result<_, _>>()?,
            boards,
        })
    }
}

impl Game {
    pub fn part1(&mut self) -> u32 {
        let mut score = 0;
        'calls: for call in &self.calls {
            for board in &mut self.boards {
                if let Some(value) = board.mark(call) {
                    score = value * board.unmarked_sum();
                    break 'calls;
                }
            }
        }
        score
    }

    pub fn part2(&mut self) -> u32 {
        let mut won = std::collections::HashSet::new();
        let length = self.boards.len();

        let mut score = 0;
        'calls: for call in &self.calls {
            for (i, board) in self.boards.iter_mut().enumerate() {
                if won.contains(&i) {
                    continue;
                }

                if let Some(value) = board.mark(call) {
                    won.insert(i);
                    score = value * board.unmarked_sum();
                    if won.len() == length {
                        break 'calls;
                    }
                }
            }
        }
        score
    }
}
