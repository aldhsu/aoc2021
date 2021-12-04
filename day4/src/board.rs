use crate::Error;

#[derive(Debug)]
pub struct Board {
    tiles: Vec<Tile>,
}

impl std::str::FromStr for Board {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .flat_map(|line| line.trim().split_whitespace().map(|num| num.parse()))
            .collect::<Result<_, _>>()?;
        Ok(Self { tiles })
    }
}

impl Board {
    pub fn mark(&mut self, call: &u32) -> Option<u32> {
        let (index, tile) = self
            .tiles
            .iter_mut()
            .enumerate()
            .find(|(_, tile)| tile.num == *call)?;

        tile.state = tile.state.transition();
        self.check_win(index).then(|| *call)
    }

    fn check_win(&self, index: usize) -> bool {
        let coord = index.into();

        [self.check_horizontal(&coord), self.check_vertical(&coord)]
            .iter()
            .any(|i| *i)
    }

    fn check_horizontal(&self, coord: &Coord) -> bool {
        let y = coord.1;
        (0..WIDTH).all(|x| -> bool {
            let index: usize = Coord(x, y).into();
            matches!(
                self.tiles.get(index),
                Some(Tile {
                    state: State::Marked,
                    ..
                })
            )
        })
    }

    fn check_vertical(&self, coord: &Coord) -> bool {
        let x = coord.0;
        (0..WIDTH).all(|y| -> bool {
            let index: usize = Coord(x, y).into();
            matches!(
                self.tiles.get(index),
                Some(Tile {
                    state: State::Marked,
                    ..
                })
            )
        })
    }

    pub fn unmarked_sum(&self) -> u32 {
        self.tiles
            .iter()
            .filter_map(|tile| {
                if tile.state == State::Unmarked {
                    Some(tile.num)
                } else {
                    None
                }
            })
            .sum()
    }
}

#[derive(Debug)]
struct Tile {
    state: State,
    num: u32,
}

impl std::str::FromStr for Tile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            state: State::Unmarked,
            num: s
                .parse()
                .map_err(|_| Error::ParseError(format!("Couldn't parse tile: {}", s)))?,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    Marked,
    Unmarked,
}
impl State {
    fn transition(&self) -> Self {
        match &self {
            State::Marked => State::Marked,
            State::Unmarked => State::Marked,
        }
    }
}

const WIDTH: usize = 5;
#[derive(Debug, PartialEq, Eq)]
struct Coord(usize, usize);
impl From<usize> for Coord {
    fn from(input: usize) -> Self {
        let y = input / WIDTH;
        let x = input % WIDTH;

        Self(x, y)
    }
}

impl From<Coord> for usize {
    fn from(input: Coord) -> Self {
        input.1 * WIDTH + input.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn converting_coord_works() {
        let coord = Coord(4, 4);
        let index: usize = coord.into();
        assert_eq!(24, index);

        let coord: Coord = index.into();
        assert_eq!(coord, Coord(4, 4));
    }

    #[test]
    fn marking_a_board_horizontally() {
        let input = r#"66 78  7 45 92
39 38 62 81 77
 9 73 25 97 99
87 80 19  1 71
85 35 52 26 68"#;
        let mut board: Board = input.parse().unwrap();
        board.mark(&66);
        board.mark(&78);
        board.mark(&7);
        assert_eq!(board.mark(&45), None);
        assert_eq!(board.mark(&92), Some(92));
    }

    #[test]
    fn marking_a_board_vertically() {
        let input = r#"66 78  7 45 92
39 38 62 81 77
 9 73 25 97 99
87 80 19  1 71
85 35 52 26 68"#;
        let mut board: Board = input.parse().unwrap();
        board.mark(&77);
        board.mark(&99);
        board.mark(&71);
        assert_eq!(board.mark(&68), None);
        assert_eq!(board.mark(&92), Some(92));
    }
}
