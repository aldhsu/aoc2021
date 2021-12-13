use anyhow::{Context, Result};
type Map = Vec<Vec<u8>>;

fn step(map: &mut Map) -> Result<usize> {
    map.iter_mut().for_each(|line| {
        line.iter_mut().for_each(|item| {
            *item += 1;
        })
    });

    let mut flashers: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
    let width = map.first().with_context(|| "no width")?.len();
    let length = map.len();

    fn try_inc(map: &mut Map, (x, y): (usize, usize)) -> Option<()> {
        *map.get_mut(y)?.get_mut(x)? += 1;
        Some(())
    }

    // find all not in flashers, the coordinates that are > 9
    // add it to a list, flashers
    // add 1 to all cells
    // repeat 1 until none can be found
    while map.iter().enumerate().any(|(y, line)| {
        line.iter()
            .enumerate()
            .any(|(x, c)| c > &9 && !flashers.contains(&(x, y)))
    }) {
        for y in 0..length {
            for x in 0..width {
                if map
                    .get(y)
                    .with_context(|| "couldn't get y")?
                    .get(x)
                    .with_context(|| "couldn't get x")?
                    > &9
                    && flashers.insert((x, y))
                {
                    for (x, y) in find_neighbours((x, y)) {
                        try_inc(map, (x, y));
                    }
                }
            }
        }
    }
    let count = flashers.len();
    for (x, y) in flashers {
        *map.get_mut(y)
            .with_context(|| "couldn't set y")?
            .get_mut(x)
            .with_context(|| "couldnt set x")? = 0;
    }

    Ok(count)
}

const OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn find_neighbours((x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    OFFSETS.iter().filter_map(move |(dx, dy)| {
        let x = x as isize + dx;
        let y = y as isize + dy;

        if x < 0 || y < 0 {
            return None;
        }
        Some((x as usize, y as usize))
    })
}

fn parse_input(input: &str) -> Result<Map> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|item| {
                    item.to_digit(10)
                        .context("can't convert to digit")
                        .map(|d| d as u8)
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_works() -> Result<()> {
        let input = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;
        let mut map = parse_input(input)?;
        assert_eq!(step(&mut map)?, 0);
        assert_eq!(step(&mut map)?, 35);
        Ok(())
    }

    #[test]
    fn part2_works() -> Result<()> {
        let input = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;
        assert_eq!(part2(input)?, 195);
        Ok(())
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut total = 0;
    let mut map = parse_input(input)?;

    for _ in 0..100 {
        total += step(&mut map)?;
    }
    Ok(total)
}

fn part2(input: &str) -> Result<usize> {
    let mut count = 0;
    let mut map = parse_input(input)?;

    loop {
        if map.iter().all(|line| line.iter().all(|c| c == &0)) {
            break
        }
        step(&mut map)?;
        count += 1;
    }

    Ok(count)
}



fn main() -> Result<()> {
    let input = include_str!("../input.txt");

    println!("part1 {}", part1(input)?);
    println!("part2 {}", part2(input)?);

    Ok(())
}
