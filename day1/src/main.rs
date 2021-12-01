use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input.txt");
    let depths = input
        .lines()
        .map(|line| line.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;
    let part1 = depths
        .iter()
        .zip(depths.iter().skip(1))
        .filter(|(current, next)| current < next)
        .count();

    let iter = depths.windows(3).map(|window| window.iter().sum::<i32>());

    let part2 = iter
        .clone()
        .zip(iter.skip(1))
        .filter(|(current, next)| current < next)
        .count();

    println!("part1 {}", part1);
    println!("part2 {}", part2);

    Ok(())
}
