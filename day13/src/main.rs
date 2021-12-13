use std::{collections::{HashSet, VecDeque}, fmt::Display, io::BufRead, str::FromStr};

use anyhow::{bail, Context, Error, Result};

#[derive(Eq, PartialEq, Clone, Hash)]
struct Coord(usize, usize);

impl FromStr for Coord {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .with_context(|| "couldn't get x and y coord")?;
        Ok(Self(x.parse()?, y.parse()?))
    }
}

enum Direction {
    X, // left
    Y, // up
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "x" => Self::X,
            "y" => Self::Y,
            _ => bail!("unknown direction {}", s),
        })
    }
}

struct Instruction {
    direction: Direction,
    coordinate: usize,
}

impl Instruction {
    fn apply(&self, coords: &mut [Coord]) {
        match self.direction {
            Direction::X => {
                coords
                    .iter_mut()
                    .filter(|Coord(x, _)| x > &self.coordinate)
                    .for_each(|Coord(x, _)| *x -= (*x - self.coordinate) * 2)
            }
            Direction::Y => {
                coords
                    .iter_mut()
                    .filter(|Coord(_, y)| y > &self.coordinate)
                    .for_each(|Coord(_, y)| *y -= (*y - self.coordinate) * 2)
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("fold along ")
            .with_context(|| "couldn't strip fold along")?;
        let (direction, coord) = s
            .split_once('=')
            .with_context(|| "couldn't get x and y coord")?;
        Ok(Self {
            direction: direction.parse()?,
            coordinate: coord.parse()?,
        })
    }
}

struct Manual {
    dots: Vec<Coord>,
    instructions: VecDeque<Instruction>,
}

impl Display for Manual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.dots.iter().max_by_key(|Coord(x, _)| x).unwrap().0;
        let max_y = self.dots.iter().max_by_key(|Coord(_, y)| y).unwrap().1;
        let set = self.dots.iter().cloned().collect::<HashSet<Coord>>();

        f.write_str(
            &(0..=max_y).map(|y| {
                (0..=max_x).map(|x| {
                    if set.contains(&Coord(x, y)) {
                        '█'
                    } else {
                        ' '
                    }
                }).collect::<String>()
            }).collect::<Vec<String>>().join("\n"))
    }
}

impl Manual {
    fn step(&mut self) -> Option<()> {
        let fold = self.instructions.pop_front()?;

        fold.apply(&mut self.dots);

        Some(())
    }

    fn count(&self) -> usize {
        self.dots.iter().cloned().collect::<HashSet<Coord>>().len()
    }
}

fn parse_instructions(input: &str) -> Result<Manual> {
    let (dots, instructions) = input
        .split_once("\n\n")
        .with_context(|| "couldn't split instructions from dots")?;

    let dots = dots
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Coord>>>()?;

    let instructions = instructions
        .lines()
        .map(|line| line.parse())
        .collect::<Result<VecDeque<Instruction>>>()?;

    Ok(Manual {
        dots, instructions
    })
}

fn part1(input: &str) -> Result<usize> {
    let mut manual = parse_instructions(input)?;
    manual.step();
    Ok(manual.count())
}

fn part2(input: &str) -> Result<()> {
    let mut manual = parse_instructions(input)?;
    while manual.step().is_some() {};

    print!("{}", manual);
    Ok(())
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("part1 {}", part1(input)?);
    part2(input)?;
    Ok(())
}
