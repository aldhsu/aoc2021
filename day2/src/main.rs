enum Move {
    Forward(isize),
    Down(isize),
    Up(isize),
}

impl From<&str> for Move {
    fn from(input: &str) -> Self {
        use Move::*;

        let (direction, magnitude) = input.split_once(" ").expect("couldn't get move");
        let magnitude = magnitude.parse::<isize>().expect("couldn't get magnitude");
        match direction {
            "forward" =>  Forward(magnitude),
            "down" =>  Down(magnitude),
            "up" =>  Up(magnitude),
            _ => unreachable!(),
        }
    }
}

struct Sub {
    aim: isize,
    x: isize,
    y: isize,
}

impl Sub {
    fn apply(&mut self, m: &Move) {
        match m {
            Move::Forward(mag) => {
                self.x += mag;
                self.y += self.aim * mag;
            },
            Move::Down(mag) => self.aim += mag,
            Move::Up(mag) => self.aim -= mag,
        }
    }
}


fn main() {
    let input = include_str!("../input.txt");
    let moves = input.lines().map(|line| line.into()).collect::<Vec<Move>>();
    let mut coord = (0isize, 0isize);

    for m in &moves {
        m.apply(&mut coord)
    }

    let mut sub = Sub { aim: 0, x: 0, y: 0};
    for m in &moves {
        sub.apply(m);
    }
    let part2 = sub.x * sub.y;

    println!("part1 {:?}", coord.0 * coord.1);
    println!("part2 {:?}", part2);
}

impl Move {
    pub(crate) fn apply(&self, coord: &mut (isize, isize)) {
        use Move::*;

        match self {
            Forward(mag) => coord.0 += mag,
            Down(mag) => coord.1 += mag,
            Up(mag) => coord.1 -= mag,
        };
    }
}
