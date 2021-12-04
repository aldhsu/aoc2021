fn part1(input: &str) -> u32 {
    let first_line = input.lines().next().unwrap();
    let line_len = first_line.len();

    let counts = input.lines().fold(vec![0; line_len], |mut memo, line| {
        for (i, char) in line.chars().enumerate() {
            if char == '1' {
                memo[i] += 1;
            }
        }
        memo
    });

    let line_mid = input.lines().count() / 2;
    let bits = counts.iter().map(|count| count > &line_mid);

    let gamma = bits.clone().fold(0u32, |memo, bit| {
        memo.wrapping_shl(1).wrapping_add(bit as u32)
    });

    let epsilon = bits.clone().fold(0u32, |memo, bit| {
        memo.wrapping_shl(1).wrapping_add(!bit as u32)
    });

    gamma * epsilon
}

fn part2(input: &str) -> u32 {
    let first_line = input.lines().next().unwrap();
    let line_len = first_line.len();

    fn ones_at_index<'a>(
        lines: impl Iterator<Item = &'a str> + Clone,
        index: usize,
    ) -> std::cmp::Ordering {
        let total = lines.clone().count();

        let sub_count = lines
            .filter(|line| matches!(line.chars().nth(index), Some('1')))
            .count();

        let mut line_mid = total / 2;
        if total % 2 == 1 {
            line_mid += 1;
        }

        sub_count.cmp(&line_mid)
    }

    fn find_line(
        mut lines: Vec<&str>,
        func: impl Fn(std::cmp::Ordering) -> char,
        line_len: usize,
    ) -> &str {
        let mut current_position = 0;
        loop {
            let bit = (func)(ones_at_index(lines.iter().cloned(), current_position));
            let next = lines
                .iter()
                .filter(|line| {
                    if let Some(c) = line.chars().nth(current_position) {
                        c == bit
                    } else {
                        false
                    }
                })
                .cloned()
                .collect::<Vec<_>>();

            if next.is_empty() || current_position == line_len {
                break lines.last().cloned().unwrap();
            } else {
                lines = next;
            }

            current_position += 1;
        }
    }

    let oxygen = find_line(
        input.lines().collect::<Vec<_>>(),
        |ord| match ord {
            std::cmp::Ordering::Less => '0',
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => '1',
        },
        line_len,
    );
    let co2 = find_line(
        input.lines().collect::<Vec<_>>(),
        |ord| match ord {
            std::cmp::Ordering::Less => '1',
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => '0',
        },
        line_len,
    );

    u32::from_str_radix(oxygen, 2).unwrap() * u32::from_str_radix(co2, 2).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2_works() {
        let input = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;
        assert_eq!(part2(input), 230)
    }
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}
