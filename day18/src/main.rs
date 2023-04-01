#![feature(box_patterns)]
use std::{cell::RefCell, rc::Rc};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;

impl SnailKind {
    fn increment_depth(&mut self) {
        match self {
            SnailKind::Num { depth, .. } => *depth += 1,
            SnailKind::Array { depth, inner, .. } => {
                *depth += 1;
                inner.0.borrow_mut().increment_depth();
                inner.1.borrow_mut().increment_depth();
            }
        }
    }
    fn flatten(&self, vec: &mut Vec<Node>) {
        if let SnailKind::Array { inner, .. } = &self {
            match *inner.0.borrow() {
                SnailKind::Num { .. } => vec.push(inner.0.clone()),
                SnailKind::Array { .. } => inner.0.borrow().flatten(vec),
            };

            match *inner.1.borrow() {
                SnailKind::Num { .. } => vec.push(inner.1.clone()),
                SnailKind::Array { .. } => inner.1.borrow().flatten(vec),
            };
        };
    }

    fn magnitude(&self) -> i32 {
        if let SnailKind::Array { inner, .. } = &self {
            let left = match *inner.0.borrow() {
                SnailKind::Num { ref inner, .. } => 3 * inner,
                _ => inner.0.borrow().magnitude() * 3,
            };

            let right = match *inner.1.borrow() {
                SnailKind::Num { ref inner, .. } => 2 * inner,
                _ => inner.1.borrow().magnitude() * 2,
            };

            left + right
        } else {
            0
        }
    }
}
#[test]
fn magnitude_works() {
    let input = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";
    let (_, snail) = parse_array(input, 0, None).unwrap();
    let result = snail.borrow().magnitude();
    assert_eq!(result, 3488);
}

fn add(left: Node, right: Node) -> Node {
    left.borrow_mut().increment_depth();
    right.borrow_mut().increment_depth();

    let mut parent = Rc::new(RefCell::new(SnailKind::Array {
        depth: 0,
        inner: (Default::default(), Default::default()),
        parent: None,
    }));

    left.borrow_mut().set_parent(Some(parent.clone()));
    right.borrow_mut().set_parent(Some(parent.clone()));

    parent.borrow_mut().change_value((left, right));
    reduce(&mut parent);

    parent
}

fn reduce(snail: &mut Node) {
    let mut do_work = true;

    while do_work {
        do_work = false;
        if explode(snail).is_some() {
            do_work = true;
            continue;
        }
        if split(snail).is_some() {
            do_work = true;
            continue;
        }
    }
}

#[test]
fn add_works() {
    let (_, snail1) = parse_array("[1,2]", 0, None).unwrap();
    let (_, snail2) = parse_array("[3,4]", 0, None).unwrap();
    let combined = add(snail1, snail2);
    let mut vec = vec![];
    combined.borrow().flatten(&mut vec);
    let nums = vec
        .into_iter()
        .map(|item| item.borrow().get_num())
        .collect::<Vec<_>>();
    assert_eq!(nums, vec![1, 2, 3, 4])
}

fn print_node(node: &Node) {
    let mut vec = vec![];
    node.borrow().flatten(&mut vec);
    let nums = vec
        .into_iter()
        .map(|item| item.borrow().get_num())
        .collect::<Vec<_>>();
    println!("{nums:?}")
}

#[derive(PartialEq, Eq, Clone)]
enum SnailKind {
    Num {
        depth: usize,
        inner: i32,
        parent: Parent,
    },
    Array {
        depth: usize,
        inner: (Node, Node),
        parent: Parent,
    },
}

impl SnailKind {
    fn change_value(&mut self, value: (Node, Node)) {
        if let SnailKind::Array { inner, .. } = self {
            *inner = value
        } else {
            unreachable!()
        }
    }

    fn change_num(&mut self, value: i32) {
        if let SnailKind::Num { inner, .. } = self {
            *inner += value
        } else {
            dbg!(self);
            unreachable!()
        }
    }

    fn get_parent(&self) -> Option<Node> {
        match self {
            SnailKind::Num { parent, .. } => parent.clone(),
            SnailKind::Array { parent, .. } => parent.clone(),
        }
    }

    fn set_parent(&mut self, new_parent: Parent) {
        match self {
            SnailKind::Num { parent, .. } => *parent = new_parent,
            SnailKind::Array { parent, .. } => *parent = new_parent,
        }
    }

    fn get_num(&self) -> i32 {
        match self {
            SnailKind::Num { inner, .. } => *inner,
            _ => unreachable!(),
        }
    }
}

impl Default for SnailKind {
    fn default() -> Self {
        SnailKind::Num {
            depth: 0,
            inner: 0,
            parent: None,
        }
    }
}

impl std::fmt::Debug for SnailKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Array { depth, inner, .. } => f
                .debug_struct("Array")
                .field("depth", depth)
                .field("inner", inner)
                .finish(),
            Self::Num {
                depth,
                inner,
                ..
            } => f
                .debug_struct("")
                .field("depth", depth)
                .field("inner", inner)
                .finish(),
        }
    }
}

type Parent = Option<Node>;
type Node = Rc<RefCell<SnailKind>>;

fn parse_array(s: &str, depth: usize, parent: Parent) -> IResult<&str, Node> {
    let candidate = Rc::new(RefCell::new(SnailKind::Array {
        depth,
        inner: (
            Rc::new(RefCell::new(Default::default())),
            Rc::new(RefCell::new(Default::default())),
        ),
        parent,
    }));
    let (s, (left, _, right)) = delimited(
        tag("["),
        tuple((
            |s| parse_snailkinds(s, depth + 1, Rc::clone(&candidate)),
            tag(","),
            |s| parse_snailkinds(s, depth + 1, Rc::clone(&candidate)),
        )),
        tag("]"),
    )(s)?;

    candidate.borrow_mut().change_value((left, right));

    Ok((s, candidate))
}

fn parse_num(s: &str, depth: usize, parent: Parent) -> IResult<&str, Node> {
    let (s, digits) = digit1(s)?;
    let value = digits.parse::<i32>().expect("should be able to get nums");
    Ok((
        s,
        Rc::new(RefCell::new(SnailKind::Num {
            depth,
            inner: value,
            parent,
        })),
    ))
}

fn parse_snailkinds(s: &str, depth: usize, parent: Node) -> IResult<&str, Node> {
    let (s, sk) = alt((
        |s| parse_array(s, depth, Some(parent.clone())),
        |s| parse_num(s, depth, Some(parent.clone())),
    ))(s)?;
    Ok((s, sk))
}

// fn parse_snailkind<'a, 'b>(s: &'a str, depth: usize, parent: Parent) -> IResult<&'a str, SnailKind> {
//     // 0, [1, 2]
// }
//
fn explode(snail: &mut Node) -> Option<()> {
    let mut vec = vec![];
    snail.borrow().flatten(&mut vec);

    let explode = vec.iter().enumerate().find(
        |(_, snailk)| matches!(*snailk.borrow(), SnailKind::Num { depth:  d, ..} if d >= 5),
    )?;

    let mut left = 0;
    let mut right = 0;
    let mut outer_index = 0;

    let (index, item) = explode;

    if let SnailKind::Num { ref parent, .. } = *item.borrow() {
        outer_index = index;
        if let SnailKind::Array {
            inner: ref value, ..
        } = *(parent.as_ref().expect("should have parent")).borrow()
        {
            if let SnailKind::Num { inner: value, .. } = *value.0.borrow() {
                left = value;
            }

            if let SnailKind::Num { inner: value, .. } = *value.1.borrow() {
                right = value
            }
        }
    }

    if outer_index.checked_sub(1).is_some() {
        if let Some(item) = vec.get(outer_index - 1) {
            item.borrow_mut().change_num(left)
        }
    }

    if let Some(item) = vec.get(outer_index + 2) {
        item.borrow_mut().change_num(right)
    }

    if let Some(item) = vec.get(outer_index) {
        item.borrow()
            .get_parent()
            .take()
            .unwrap_or_else(|| panic!("no parent"))
            .replace_with(|old| match old {
                SnailKind::Array { depth, parent, .. } => SnailKind::Num {
                    inner: 0,
                    depth: *depth,
                    parent: parent.clone(),
                },
                _ => unreachable!(),
            });
    }

    Some(())
}

#[test]
fn exploding_works() {
    let (_, mut snail) = parse_array("[[[[1,2],3],4],5]", 0, None).unwrap();
    assert!(explode(&mut snail).is_some());
    let mut vec = vec![];
    snail.borrow().flatten(&mut vec);
    let nums = vec
        .into_iter()
        .map(|item| item.borrow().get_num())
        .collect::<Vec<_>>();
    assert_eq!(nums, vec![0, 5, 4, 5])
}

fn split(snail: &mut Node) -> Option<()> {
    // collect vec find a node that is > 10, get the reference and make a new one and swap the
    // refcell
    //
    let mut vec = vec![];
    snail.borrow().flatten(&mut vec);

    let splitter = vec
        .iter()
        .find(|snailk| matches!(*snailk.borrow(), SnailKind::Num { inner, ..} if inner >= 10))?;

    let mut left = 0;
    let mut right = 0;
    let mut outer_depth = 0;
    splitter.replace_with(|old| match old {
        SnailKind::Num {
            depth,
            inner,
            parent,
        } => {
            left = *inner / 2;
            right = (*inner + 1) / 2; // check this works? should work because it is
            outer_depth = *depth + 1;
            SnailKind::Array {
                parent: parent.clone(),
                depth: *depth,
                inner: (Default::default(), Default::default()),
            }
        }
        _ => unreachable!(),
    });

    splitter.borrow_mut().change_value((
        Rc::new(RefCell::new(SnailKind::Num {
            depth: outer_depth,
            inner: left,
            parent: Some(Rc::clone(splitter)),
        })),
        Rc::new(RefCell::new(SnailKind::Num {
            depth: outer_depth,
            inner: right,
            parent: Some(Rc::clone(splitter)),
        })),
    ));

    Some(())
}

#[test]
fn splitting_works() {
    let (_, mut snail) = parse_array("[10,5]", 0, None).unwrap();
    assert!(split(&mut snail).is_some());
    let mut vec = vec![];
    snail.borrow().flatten(&mut vec);
    let nums = vec
        .into_iter()
        .map(|item| item.borrow().get_num())
        .collect::<Vec<_>>();
    assert_eq!(nums, vec![5, 5, 5])
}

fn parse_line(s: &str) -> Node {
    let (_, snail) = parse_array(s, 0, None).unwrap();
    snail
}

fn add_all(s: &str) -> Result<Node, String> {
    s
        .trim()
        .lines()
        .map(parse_line)
        .reduce(add)
        .ok_or("nothing here".into())
}

#[test]
fn add_all_works() {
    let input = r#"[1,1]
[2,2]
[3,3]
[4,4]"#;
    let combined = add_all(input).unwrap();
    let mut vec = vec![];
    combined.borrow().flatten(&mut vec);
    let nums = vec
        .into_iter()
        .map(|item| item.borrow().get_num())
        .collect::<Vec<_>>();
    assert_eq!(nums, vec![1, 1, 2, 2, 3, 3, 4, 4]);
    let input = r#"[1,1]
[2,2]
[3,3]
[4,4]
[5,5]"#;
    let combined = add_all(input).unwrap();
    let mut vec = vec![];
    combined.borrow().flatten(&mut vec);
    let nums = vec
        .into_iter()
        .map(|item| item.borrow().get_num())
        .collect::<Vec<_>>();
    assert_eq!(nums, vec![3, 0, 5, 3, 4, 4, 5, 5]);
}

fn part1(s: &str) -> Result<i32, String> {
    let snail = add_all(s)?;
    let result = snail.borrow().magnitude();
    Ok(result)
}

#[test]
fn part1_works() {
    // assert_eq!(part1("[1,3]"), Ok(2));
    assert_eq!(
        part1(
            r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#
        ),
        Ok(4140)
    )
}

fn part2(s: &str) -> Result<i32, String> {
    let snails = s.trim().lines();
    let mut max = i32::MIN;

    for left in snails.clone() {
        for right in snails.clone() {
            if left == right {
                continue;
            }
            let left = parse_line(left);
            let right = parse_line(right);

            let mag = add(left, right).borrow().magnitude();
            max = max.max(mag);
        }
    }

    Ok(max)
}
#[test]
fn part2_works() {
    let input = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#;
    assert_eq!(part2(input).unwrap(), 3993);
}

fn main() -> Result<(), String> {
    let input = include_str!("../input.txt");
    let part1 = part1(input)?;
    println!("part1: {part1}");
    let part2 = part2(input)?;
    println!("part2: {part2}");
    Ok(())
}
