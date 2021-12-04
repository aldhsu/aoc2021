mod error;
use error::Error;

mod game;
use game::Game;

mod board;

fn main() -> Result<(), Error> {
    let input = include_str!("../input.txt");
    let mut game = input.parse::<Game>()?;
    println!("part1: {}", game.part1());
    println!("part1: {}", game.part2());
    Ok(())
}
