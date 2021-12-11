fn expected_open(input: char) -> char {
    match input {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => unreachable!(),
    }
}

fn expected_close(input: char) -> char {
    match input {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn match_input(input: char, candidate: Option<char>) -> Option<bool> {
    Some(expected_open(input) == candidate?)
}

fn part2(input: &str) -> u64 {
    let mut part2: Vec<u64> = input
        .lines()
        .filter_map(|line| {
            let mut score = 0;
            let mut queue = vec![];
            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => {
                        queue.push(c);
                    }
                    ')' | ']' | '}' | '>' => match match_input(c, queue.pop()) {
                        Some(true) => {}
                        Some(false) => return None,
                        None => break,
                    },
                    _ => unreachable!(),
                }
            }

            if !queue.is_empty() {
                while let Some(unmatched) = queue.pop() {
                    let val = match expected_close(unmatched) {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    };
                    score *= 5;
                    score += val;
                }
                Some(score)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    part2.sort_unstable();
    part2[part2.len() / 2]
}

fn main() {
    let input = include_str!("../input.txt");

    let part1 = input
        .lines()
        .map(|line| {
            let mut queue = vec![];
            let mut score = 0;

            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => {
                        queue.push(c);
                    }
                    ')' | ']' | '}' | '>' => match match_input(c, queue.pop()) {
                        Some(true) => {}
                        Some(false) => {
                            score = match c {
                                ')' => 3,
                                ']' => 57,
                                '}' => 1197,
                                '>' => 25137,
                                _ => unreachable!(),
                            };
                            break;
                        }
                        None => break,
                    },
                    _ => unreachable!(),
                }
            }

            score
        })
        .sum::<u32>();

    println!("part1: {}", part1);
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2_works() {
        let input = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;
        assert_eq!(part2(input), 288957)
    }
}
