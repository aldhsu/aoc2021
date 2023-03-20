use std::{cell::RefCell, rc::Rc};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::newline;
use nom::combinator::complete;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;

#[derive(PartialEq, Eq, Debug)]
struct Snail {
    id: usize,
    inner: Box<(SnailKind, SnailKind)>,
}

#[derive(PartialEq, Eq, Debug)]
enum SnailKind {
    Array { depth: usize, value: Snail, parent_id: usize },
    Num { depth: usize, value: i32, parent_id: usize },
}

// Snail container has a flattened representation of all Snail::Num
// tracks by references?
struct SnailContainer {
    items: Vec<Rc<RefCell<Snail>>>,
}

struct IdGen {
    gen: usize,
}

impl IdGen {
    fn generate(&mut self) -> usize {
        self.gen += 1;
        self.gen
    }
}

fn parse_array(s: &str, depth: usize, id_gen: &mut IdGen) -> IResult<&str, SnailKind> {
    let (s, snail) = delimited(tag("["), |s| parse_snail(s, depth, id_gen.gen()), tag("]"))(s)?;
    Ok((
        s,
        SnailKind::Array {
            parent_id: 1,
            depth,
            value: snail,
        },
    ))
}

#[test]
fn parse_array_works() {
    assert_eq!(
        parse_array("[1,2]", 0),
        Ok((
            "",
            SnailKind::Array {
                parent_id: 1,
                depth: 0,
                value: Snail {
                    id: 2,
                    inner: Box::new((
                        SnailKind::Num { depth: 1, value: 1 },
                        SnailKind::Num { depth: 1, value: 2 },
                    ))
                },
            }
        ))
    );
}

fn parse_num(s: &str, depth: usize) -> IResult<&str, SnailKind> {
    let (s, digits) = digit1(s)?;
    let value = digits.parse::<i32>().expect("should be able to get nums");
    Ok((s, SnailKind::Num { depth, value }))
}

fn parse_snailkinds(s: &str, depth: usize) -> IResult<&str, SnailKind> {
    let (s, sk) = alt((|s| parse_num(s, depth), |s| parse_array(s, depth)))(s)?;
    Ok((s, sk))
}

fn parse_snail(s: &str, depth: usize) -> IResult<&str, Snail> {
    // 0, [1, 2]
    let parser = |s| parse_snailkinds(s, depth + 1);
    let (s, (left, _, right)) = tuple((parser, tag(","), parser))(s)?;
    Ok((s, Snail {
        inner: Box::new((left, right)),
    }))
}

fn parse_snails(s: &str) -> IResult<&str, SnailContainer> {
    let (s, snail) = parse_snail(s, 0)?;
    todo!()
}

fn main() {
    println!("Hello, world!");
}
