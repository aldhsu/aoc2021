#[derive(Debug)]
enum Error {
    Part1Error(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

fn part1(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let nums = input
        .trim()
        .split(',')
        .map(|num| num.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;

    let avg = (nums.iter().sum::<u32>() as f64 / nums.len() as f64).round() as u32;

    const SLOP: f64 = 0.6;
    let candidates = (avg - ((avg as f64 * SLOP) as u32)..(avg + (avg as f64 * SLOP) as u32)).map(|alignment_point|{
        nums.iter().map(|i| (alignment_point as i32 - *i as i32).abs() as u32).sum::<u32>()
    }).collect::<Vec<_>>();

    Ok(candidates.into_iter().min().unwrap())
}

fn part2(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let nums = input
        .trim()
        .split(',')
        .map(|num| num.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;

    let avg = (nums.iter().sum::<u32>() as f64 / nums.len() as f64).round() as u32;

    const SLOP: f64 = 0.6;
    let candidates = (avg - ((avg as f64 * SLOP) as u32)..(avg + (avg as f64 * SLOP) as u32)).map(|alignment_point|{
        nums.iter().map(|i| {
            let base = (alignment_point as i32 - *i as i32).abs() as u32;
            (0..=base).sum::<u32>()
        }).sum::<u32>()
    }).collect::<Vec<_>>();

    Ok(candidates.into_iter().min().unwrap())
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn part1_works() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(part1(input).unwrap(), 37)
    }

    #[test]
    fn part2_works() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(part2(input).unwrap(), 168)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input)?);
    println!("part2: {}", part2(input)?);
    Ok(())
}
