use std::collections::HashSet;

#[derive(Debug)]
enum Error {
    ParseError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

const SURROUNDING: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

fn get_surrounding_coords(
    map: &[Vec<u32>],
    (coord_x, coord_y): (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    let y_len = map.len();
    let x_len = map.first().map(|line| line.len()).unwrap_or(0);

    SURROUNDING.iter().filter_map(move |(offset_x, offset_y)| {
        let y: usize = (coord_y as isize + offset_y).try_into().ok()?;
        let x: usize = (coord_x as isize + offset_x).try_into().ok()?;

        (x < x_len && y < y_len).then(|| (x, y))
    })
}

fn get_surrounding(map: &[Vec<u32>], coords: (usize, usize)) -> impl Iterator<Item = &u32> {
    get_surrounding_coords(map, coords)
        .filter_map(move |(x, y)| map.get(y).and_then(|line| line.get(x)))
}

fn part1(map: &[Vec<u32>]) -> Result<u32, Box<dyn std::error::Error>> {
    let mut risk_level = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if get_surrounding(map, (x, y)).all(|other| cell < other) {
                risk_level += 1 + cell;
            }
        }
    }

    Ok(risk_level)
}

fn part2(map: &[Vec<u32>]) -> Result<u32, Box<dyn std::error::Error>> {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut groups = vec![];

    for (y, line) in map.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if seen.contains(&(x, y)) {
                continue;
            }
            if cell == &9 {
                seen.insert((x, y));
                continue;
            }

            let mut candidates = vec![(x, y)];
            let mut count = 0;

            while let Some(coord) = candidates.pop() {
                if !seen.insert(coord) {
                    continue;
                }
                if map
                    .get(coord.1)
                    .map(|line| line.get(coord.0))
                    .map(|num| matches!(num, Some(9)))
                    .unwrap_or(false)
                {
                    continue;
                }
                count += 1;
                candidates.extend(get_surrounding_coords(map, coord));
            }

            groups.push(count)
        }
    }

    groups.sort_unstable();
    groups.reverse();
    Ok(groups.iter().take(3).product())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2_works() {
        let input = "1239123";
        let map = parse_map(input).unwrap();
        dbg!(&map);
        assert_eq!(part2(&map).unwrap(), 9);
    }
}

fn parse_map(input: &str) -> Result<Vec<Vec<u32>>, Error> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|num| num.to_digit(10).ok_or(Error::ParseError))
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input.txt");
    let map = parse_map(input)?;

    println!("part1 {}", part1(&map)?);
    println!("part2 {}", part2(&map)?);
    Ok(())
}
