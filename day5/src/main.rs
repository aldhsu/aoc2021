use std::ops::Sub;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct Coord(usize, usize);

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::str::FromStr for Coord {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .ok_or_else(|| "couldn't get start and end".to_owned())?;
        Ok(Self(
            x.parse::<usize>()
                .map_err(|_| "Couldn't get x".to_owned())?,
            y.parse().map_err(|_| "Couldn't get y".to_owned())?,
        ))
    }
}

#[derive(Debug)]
struct Vent {
    start: Coord,
    end: Coord,
}

impl Vent {
    fn is_straight(&self) -> bool {
        matches!((self.end.0 == self.start.0, self.end.1 == self.start.1), (true, _) | (_, true))
    }

    fn all_coords(&self) -> /* Vec<Coord> */ Box<dyn Iterator<Item = Coord>> {
        let Coord(mut start_x, mut start_y) = self.start;
        let Coord(mut end_x, mut end_y) = self.end;

        if self.is_straight() {
            if start_x > end_x {
                std::mem::swap(&mut start_x, &mut end_x)
            }

            if start_y > end_y {
                std::mem::swap(&mut start_y, &mut end_y)
            }

            Box::new((start_x..=end_x)
                .flat_map(move |x| (start_y..=end_y).map(move |y| Coord(x, y))))
        } else {
            let x_range: Box<dyn Iterator<Item = usize>> = match start_x.cmp(&end_x) {
                std::cmp::Ordering::Less => Box::new(start_x..=end_x),
                std::cmp::Ordering::Greater => Box::new((end_x..=start_x).rev()),
                std::cmp::Ordering::Equal => unreachable!(),
            };

            let y_range: Box<dyn Iterator<Item = usize>> = match start_y.cmp(&end_y) {
                std::cmp::Ordering::Less => Box::new(start_y..=end_y),
                std::cmp::Ordering::Greater => Box::new((end_y..=start_y).rev()),
                std::cmp::Ordering::Equal => unreachable!(),
            };

            Box::new((x_range)
                     .zip(y_range)
                     .map(|(x, y)| Coord(x, y)))
        }
    }
}

impl std::str::FromStr for Vent {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(" -> ")
            .ok_or_else(|| "couldn't get start and end".to_owned())?;

        Ok(Self {
            start: left.parse()?,
            end: right.parse()?,
        })
    }
}

struct StraightMap {
    inner: std::collections::HashMap<Coord, usize>,
}

impl StraightMap {
    fn find_greater_than_one(&self) -> usize {
        self.inner.iter().filter(|(_, &count)| count > 1).count()
    }
}

impl std::str::FromStr for StraightMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner = std::collections::HashMap::new();

        let vents = s
            .lines()
            .map(|line| line.parse::<Vent>())
            .collect::<Result<Vec<Vent>, _>>()?;

        vents
            .into_iter()
            .filter(|vent| vent.is_straight())
            .flat_map(|vent| vent.all_coords())
            .for_each(|coord| {
                *inner.entry(coord).or_insert(0) += 1;
            });

        Ok(Self { inner })
    }
}

struct DiagonalMap {
    inner: std::collections::HashMap<Coord, usize>,
}

impl DiagonalMap {
    fn find_greater_than_one(&self) -> usize {
        self.inner.iter().filter(|(_, &count)| count > 1).count()
    }
}

impl std::str::FromStr for DiagonalMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner = std::collections::HashMap::new();

        let vents = s
            .lines()
            .map(|line| line.parse::<Vent>())
            .collect::<Result<Vec<Vent>, _>>()?;

        vents
            .into_iter()
            .flat_map(|vent| vent.all_coords())
            .for_each(|coord| {
                *inner.entry(coord).or_insert(0) += 1;
            });

        Ok(Self { inner })
    }
}

fn part1(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let straight_map: StraightMap = input.parse()?;
    Ok(straight_map.find_greater_than_one())
}

fn part2(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let diagonal_map: DiagonalMap = input.parse()?;
    Ok(diagonal_map.find_greater_than_one())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let input = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;
        assert_eq!(part1(input).unwrap(), 5)
    }

    #[test]
    fn part2_works() {
        let input = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;
        assert_eq!(part2(input).unwrap(), 12)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input)?);
    println!("part2: {}", part2(input)?);
    Ok(())
}
