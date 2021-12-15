use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

use anyhow::{Context, Result};

fn parse_input(input: &str) -> Result<Vec<Vec<u32>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).with_context(|| "couldn't get digit"))
                .collect::<Result<Vec<u32>>>()
        })
        .collect()
}

const OFFSETS: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

fn neighbours<'a>((x, y): &'a (usize, usize)) -> impl Iterator<Item = (usize, usize)> + 'a {
    OFFSETS.iter().filter_map(|(o_x, o_y)| {
        let x: usize = (*x as isize + o_x).try_into().ok()?;
        let y: usize = (*y as isize + o_y).try_into().ok()?;

        Some((x, y))
    })
}

fn find_path(map: &Vec<Vec<u32>>) -> Result<u32> {
    let mut open_set = BinaryHeap::new();
    open_set.push(Reverse((map[0][0], (0, 0))));
    let mut gscore_map = HashMap::new();
    let goal = {
        let x = map.first().with_context(|| "couldn't get width")?.len() - 1;
        let y = map.len() - 1;
        (x, y)
    };

    let mut came_from = HashMap::new();

    while let Some(Reverse((parent_cost, (px, py)))) = open_set.pop() {
        for (x, y) in neighbours(&(px, py)) {
            if let Some(Some(cost)) = map.get(y).map(|val| val.get(x)) {
                let actual_cost = cost + parent_cost;

                match gscore_map.entry((x, y)) {
                    std::collections::hash_map::Entry::Occupied(mut val)
                        if val.get() > &actual_cost =>
                    {
                        *val.get_mut() = actual_cost;
                    }
                    std::collections::hash_map::Entry::Vacant(e) => {
                        e.insert(actual_cost);
                    }
                    _ => continue,
                }

                came_from.insert((x, y), (px, py));
                open_set.push(Reverse((actual_cost, (x, y))))
            }
        }

        if (px, py) == goal {
            println!("finished");
            break;
        } // TODO: add the final value?
    }

    Ok(gscore_map[&goal] - map[0][0])
}

fn part1(input: &str) -> Result<u32> {
    let map = parse_input(input)?;
    find_path(&map)
}

fn tile_map(map: Vec<Vec<u32>>) -> Result<Vec<Vec<u32>>> {
    let mut new_map = (0..5)
        .map(|y| {
            (0..5)
                .map(|x| {
                    let mut current_map = map.clone();

                    for line in current_map.iter_mut() {
                        for cell in line {
                            let mut num = *cell + x + y;

                            if num > 9 {
                                num %= 9;
                            }
                            *cell = num
                        }
                    }
                    current_map
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for line in &mut new_map {
        let (first_map, rest) = line
            .split_first_mut()
            .with_context(|| "can't get first map")?;

        for (i, line) in first_map.iter_mut().enumerate().rev() {
            for vec in rest.iter_mut() {
                line.append(&mut vec.pop().with_context(|| "couldn't append")?)
            }
        }

        for i in 0..4 {
            line.pop();
        }
    }

    Ok(new_map
        .into_iter()
        .flatten()
        .flatten()
        .collect::<Vec<Vec<u32>>>())
}

fn part2(input: &str) -> Result<u32> {
    let map = parse_input(input)?;
    let tiled_map = tile_map(map)?;
    find_path(&tiled_map)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let input = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;
        assert_eq!(part1(input).unwrap(), 40);
    }

    #[test]
    fn part2_works() {
        let input = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;
        assert_eq!(part2(input).unwrap(), 315);
    }

    #[test]
    fn tiling_works() {
        let input = vec![vec![9]];
        assert_eq!(
            tile_map(input).unwrap(),
            vec![
                vec![9, 1, 2, 3, 4],
                vec![1, 2, 3, 4, 5],
                vec![2, 3, 4, 5, 6],
                vec![3, 4, 5, 6, 7],
                vec![4, 5, 6, 7, 8],
            ]
        )
    }
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input)?);
    println!("part2: {}", part2(input)?);
    Ok(())
}
