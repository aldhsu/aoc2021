use anyhow::{Context, Error, Result};
use std::{fmt::Display, str::FromStr};

#[derive(Eq, PartialEq, Copy, Clone)]
enum Cuke {
    South,
    East,
}

impl Cuke {
    fn apply<'a>(
        &self,
        x: &'a usize,
        y: &'a usize,
        map: &'a [Vec<Tile>],
    ) -> Option<((usize, usize), (usize, usize))> {
        let length = map.len();
        let width = map.first().map(|line| line.len()).unwrap_or(0);

        let next = match self {
            Cuke::South => (*x, (y + 1) % length),
            Cuke::East => ((x + 1) % width, *y),
        };

        if let Tile { occupant: Some(_) } = map[next.1][next.0] {
            None
        } else {
            Some(((*x, *y), next))
        }
    }
}

impl FromStr for Cuke {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            ">" => Cuke::East,
            "v" => Cuke::South,
            _ => return None.with_context(|| format!("Unknown cuke {:?}", s)),
        })
    }
}

struct Tile {
    occupant: Option<Cuke>,
}

impl FromStr for Tile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "." => Self { occupant: None },
            occupied => Self {
                occupant: Some(occupied.parse()?),
            },
        })
    }
}

struct Map {
    inner: Vec<Vec<Tile>>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse())
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { inner })
    }
}

impl Map {
    fn step(&mut self) -> usize {
        fn change_direction(map: &mut Map, cuke_type: Cuke) -> usize {
            let mut changes = vec![];

            for (y, line) in map.inner.iter().enumerate() {
                for (x, tile) in line.iter().enumerate() {
                    if tile.occupant == Some(cuke_type) {
                        if let Some(Some(change)) = tile
                            .occupant
                            .as_ref()
                            .map(|occupant| occupant.apply(&x, &y, &map.inner))
                        {
                            changes.push(change);
                        }
                    }
                }
            }

            let changes_count = changes.len();

            for (before, after) in changes {
                let occupant = map.inner[before.1][before.0].occupant.take();
                map.inner[after.1][after.0].occupant = occupant;
            }

            changes_count
        }

        change_direction(self, Cuke::East) + change_direction(self, Cuke::South)
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self
            .inner
            .iter()
            .map(|line| {
                line.iter()
                    .map(|tile| match tile {
                        Tile {
                            occupant: Some(Cuke::South),
                        } => 'v',
                        Tile {
                            occupant: Some(Cuke::East),
                        } => '>',
                        _ => '.',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", result)
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut map = input.parse::<Map>()?;
    let mut count = 1;

    while map.step() > 0 {
        count += 1;
    }

    Ok(count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let input = r#"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"#;
        assert_eq!(part1(input).unwrap(), 58)
    }
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("part1 {}", part1(input)?);
    Ok(())
}
