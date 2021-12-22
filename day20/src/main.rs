use anyhow::{Context, Error, Result};
use std::{collections::HashSet, fmt::Display, ops::Range, str::FromStr};

struct Mask {
    inner: Vec<bool>,
    zeroth_border: bool,
}

impl Mask {
    fn get(&self, num: usize) -> &bool {
        &self.inner[num]
    }

    fn default(&self, num: usize) -> &bool {
        if num % 2 == 0 {
            &false
        } else {
            &self.zeroth_border
        }
    }
}

impl FromStr for Mask {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(s.trim().chars().count(), 512);
        let inner = s.chars().map(|c| matches!(c, '#')).collect::<Vec<_>>();
        let zeroth_border = inner[0]; // when all empty;

        Ok(Self {
            inner,
            zeroth_border,
        })
    }
}

struct Image {
    inner: HashSet<(isize, isize)>,
    ranges: (std::ops::Range<isize>, std::ops::Range<isize>),
}

impl Image {
    fn enhance(&mut self, mask: &Mask, iter_num: usize) -> Result<()> {
        let x_range = (self.ranges.0.start - 1)..(self.ranges.0.end + 1);
        let y_range = (self.ranges.1.start - 1)..(self.ranges.1.end + 1);

        let mask_default = mask.default(iter_num);

        let inner = &self.inner;
        let new = x_range
            .clone()
            .flat_map(|x| {
                let x_r = self.ranges.0.clone();
                let y_r = self.ranges.1.clone();

                y_range.clone().filter_map(move |y| {
                    let num = all_cells(&(x, y))
                        .map(|cell| {
                            if x_r.contains(&cell.0) && y_r.contains(&cell.1) {
                                inner.contains(&cell)
                            } else {
                                *mask_default
                            }
                        })
                        .fold(0, fold_to_num);

                    mask.get(num).then(|| (x, y))
                })
            })
            .collect::<HashSet<(isize, isize)>>();

        self.ranges = (x_range, y_range);
        self.inner = new;
        Ok(())
    }

    fn count(&self) -> usize {
        self.inner.len()
    }
}

fn fold_to_num(mut memo: usize, bit: bool) -> usize {
    memo <<= 1;
    memo += bit as usize;
    memo
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (ref x_range, ref y_range) = &self.ranges;

        let result = (y_range.clone())
            .map(|y: isize| {
                x_range
                    .clone()
                    .map(|x: isize| match self.inner.get(&(x, y)) {
                        Some(_) => '█',
                        _ => ' ',
                    })
                    .chain(std::iter::once('\n'))
                    .collect::<String>()
            })
            .collect::<String>();

        write!(f, "{}", result)
    }
}

const OFFSETS: [(isize, isize); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn all_cells(coord: &(isize, isize)) -> impl Iterator<Item = (isize, isize)> + '_ {
    OFFSETS
        .iter()
        .map(|(o_x, o_y)| (coord.0 + o_x, coord.1 + o_y))
}

impl FromStr for Image {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let y_range = 0..s.lines().count() as isize;
        let x_range = 0..s
            .lines()
            .next()
            .with_context(|| "coudln't get x len")?
            .chars()
            .count() as isize;

        let inner = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, c)| matches!(c, '#').then(|| (x as isize, y as isize)))
            })
            .collect::<HashSet<_, _>>();

        Ok(Self {
            inner,
            ranges: ((x_range), (y_range)),
        })
    }
}

fn parse_input(input: &str) -> Result<(Mask, Image)> {
    let (mask, map) = input
        .split_once("\n\n")
        .with_context(|| "couldn't get parts")?;
    let mask = mask.parse()?;
    let map = map.parse()?;

    Ok((mask, map))
}

fn part1(input: &str) -> Result<usize> {
    let (mask, mut map) = parse_input(input)?;
    println!("{}", map);
    for i in 0..2 {
        map.enhance(&mask, i)?;
        println!("{}", map);
    }

    Ok(map.count())
}

fn part2(input: &str) -> Result<usize> {
    let (mask, mut map) = parse_input(input)?;
    for i in 0..50 {
        map.enhance(&mask, i)?;
    }

    Ok(map.count())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let input = r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"#;
        assert_eq!(part1(input).unwrap(), 35);
    }

    #[test]
    fn part2_works() {
        let input = r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"#;
        assert_eq!(part2(input).unwrap(), 3351)
    }

    #[test]
    fn border_works() {
        let input = r#"###.#...#.......#...............#...............................#...............................................................#...............................................................................................................................#...............................................................................................................................................................................................................................................................

#....
.....
.....
.....
....."#;
        assert_eq!(part1(input).unwrap(), 0)
    }

    #[test]
    fn fold_to_num_works() {
        let vec = "000100010".chars().map(|c| matches!(c, '1'));
        assert_eq!(vec.fold(0, fold_to_num), 34)
    }
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("part1 {}", part1(input)?);
    println!("part2 {}", part2(input)?);
    Ok(())
}
