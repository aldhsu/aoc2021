use std::collections::HashMap;

use anyhow::{Context, Result};

type Rules = HashMap<(char, char), char>;

fn parse_input(input: &str) -> Result<(Vec<char>, Rules)> {
    let (polymer, rules) = input
        .split_once("\n\n")
        .with_context(|| "couldn't get rules and polymer")?;
    let rules = rules
        .lines()
        .map(|line| {
            line.split_once(" -> ")
                .with_context(|| "couldn't get tuple and insert")
                .map(|(tuple, insert)| {
                    let mut iter = tuple.chars();
                    (
                        (iter.next().unwrap(), iter.next().unwrap()),
                        insert.chars().next().unwrap(),
                    )
                })
        })
        .collect::<Result<_>>()?;

    let polymer = polymer.chars().collect();

    Ok((polymer, rules))
}

fn find_result(input: &str, iterations: usize) -> Result<usize> {
    let (polymer, rules) = parse_input(input)?;
    let mut pairs: HashMap<(char, char), usize> = HashMap::new();

    for chars in polymer.windows(2) {
        let mut key = chars.iter().cloned();
        let key = (key.next().unwrap(), key.next().unwrap());
        *pairs.entry(key).or_insert(0) += 1;
    }

    for _ in 0..iterations {
        let mut next_gen = HashMap::new();

        dbg!(&pairs);
        for (k, count) in pairs.into_iter() {
            if let Some(new) = rules.get(&k) {
                for pair in [(k.0, *new), (*new, k.1)].into_iter() {
                    *next_gen.entry(pair).or_insert(0) += count;
                }
            } else {
                *next_gen.entry(k).or_insert(0) += count;
            }
        }

        pairs = next_gen;
    }
    dbg!(&pairs);

    let mut left_counts: HashMap<char, usize> = HashMap::new();
    for ((left, _), count) in &pairs {
        *left_counts.entry(*left).or_insert(0) += count;
    }

    let mut right_counts: HashMap<char, usize> = HashMap::new();
    for ((_, right), count) in &pairs {
        *right_counts.entry(*right).or_insert(0) += count;
    }

    for (k, mut v) in right_counts.into_iter() {
        if let Some(left_val) = left_counts.get_mut(&k) {
            if *left_val != v {
                *left_val = *left_val.max(&mut v);
            }
        }
    }

    let mut counts_as_vec = left_counts.into_iter().collect::<Vec<_>>();
    counts_as_vec.sort_by_key(|(_, count)| *count);
    let result = counts_as_vec[counts_as_vec.len() - 1].1 - counts_as_vec[0].1;

    Ok(result)
}

fn part1(input: &str) -> Result<usize> {
    find_result(input, 10)
}

fn part2(input: &str) -> Result<usize> {
    find_result(input, 40)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2_works() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        assert_eq!(part2(input).unwrap(), 2188189693529)
    }

    #[test]
    fn part1_works() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        assert_eq!(part1(input).unwrap(), 1588)
    }
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("part2 {}", part2(input)?);
    Ok(())
}
