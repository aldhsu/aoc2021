use std::str::FromStr;

struct Sim {
    fish_groups: [usize; 7],
    day1_fish: usize,
    day2_fish: usize,
}

impl Sim {
    fn run(&mut self, steps: usize) -> usize {
        for _ in 0..steps {
            let new_fish = self.fish_groups[0];

            self.fish_groups.rotate_left(1);
            self.fish_groups[6] += self.day2_fish;
            self.day2_fish = self.day1_fish;
            self.day1_fish = new_fish;
        }

        self.fish_groups
            .iter()
            .chain([self.day1_fish, self.day2_fish].iter())
            .sum()
    }
}

impl FromStr for Sim {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fishes = s
            .split(',')
            .map(|num| {
                num.parse::<usize>()
                    .map_err(|_| "couldn't parse num".to_string())
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut fish_groups = [0usize; 7];

        for fish in fishes {
            fish_groups[fish] += 1;
        }

        Ok(Self {
            day1_fish: 0,
            day2_fish: 0,
            fish_groups,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_works() {
        let input = r#"3,4,3,1,2"#;
        let mut sim = input.trim().parse::<Sim>().unwrap();
        assert_eq!(5934, sim.run(80));
    }
}

fn main() -> Result<(), String> {
    let input = include_str!("../input.txt");
    let mut sim = input.trim().parse::<Sim>()?;
    println!("part1: {}", sim.run(80));
    println!("part2: {}", sim.run(176));

    Ok(())
}
