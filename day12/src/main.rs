use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::ops::{Deref, DerefMut};

struct Graph<'a> {
    inner: HashMap<&'a str, HashSet<&'a str>>,
}

#[derive(Clone, PartialOrd, Ord, Eq, PartialEq)]
struct Path<'a> {
    inner: Vec<&'a str>,
}

impl<'a> fmt::Debug for Path<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.inner.join(","))
    }
}

impl<'a> Path<'a> {
    fn unvisitable_nodes(&self) -> HashSet<&str> {
        self.inner
            .iter()
            .cloned()
            .filter(|item| item.chars().all(|c| c.is_lowercase()))
            .collect()
    }
}

impl<'a> DerefMut for Path<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> Deref for Path<'a> {
    type Target = Vec<&'a str>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> Graph<'a> {
    fn trace_nodes(
        &'a self,
        path: Path<'a>,
        current_node: &'a str,
        double_visit: bool,
    ) -> Option<Vec<Path<'a>>> {
        if current_node == "end" {
            return Some(vec![path]);
        };

        let next_nodes = self.inner.get(current_node)?;

        let visitable_nodes = {
            let unvisitable = path.unvisitable_nodes();

            next_nodes
                .iter()
                .filter(|&n| !unvisitable.contains(n))
                .collect::<Vec<_>>()
        };

        let mut single_visits = visitable_nodes
            .into_iter()
            .flat_map(|&node| {
                let mut new_path = path.clone();
                new_path.push(node);
                self.trace_nodes(new_path, node, double_visit)
            })
            .flatten()
            .collect::<Vec<_>>();

        let double_visit = if double_visit {
            next_nodes
                .iter()
                .filter(|node| !["start", "end"].contains(node))
                .flat_map(|&node| {
                    let mut new_path = path.clone();
                    new_path.push(node);
                    self.trace_nodes(new_path, node, false)
                })
                .flatten()
                .collect::<Vec<_>>()
        } else {
            vec![]
        };

        if single_visits.is_empty() && double_visit.is_empty() {
            return None;
        }

        single_visits.extend(double_visit);
        single_visits.sort();
        single_visits.dedup();

        Some(single_visits)
    }

    fn part1(&self) -> usize {
        let path = Path {
            inner: vec!["start"],
        };
        let nodes = self.trace_nodes(path, "start", false).unwrap();
        nodes.len()
    }

    fn part2(&self) -> usize {
        let path = Path {
            inner: vec!["start"],
        };
        let mut nodes = self.trace_nodes(path, "start", true).unwrap();
        nodes.sort();
        nodes.len()
    }
}

impl<'a> From<&'a str> for Graph<'a> {
    fn from(s: &'a str) -> Self {
        let mut inner = HashMap::new();

        for line in s.lines() {
            let (left, right) = line.split_once('-').context("no stuff").unwrap();

            inner.entry(left).or_insert_with(HashSet::new).insert(right);
            inner.entry(right).or_insert_with(HashSet::new).insert(left);
        }

        Self { inner }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let input = "start-end";
        let graph = Graph::from(input);
        assert_eq!(1, graph.part1());

        let input = "start-end
A-start";
        let graph = Graph::from(input);
        assert_eq!(1, graph.part1());

        let input = "start-a
a-b
b-end
a-c
a-end";
        let graph = Graph::from(input);
        assert_eq!(2, graph.part1());

        let input = "start-A
A-b
A-end";
        // start A b A end
        // start A end
        let graph = Graph::from(input);
        assert_eq!(2, graph.part1());
    }

    #[test]
    fn part2_works() {
        let input = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;
        let graph = Graph::from(input);
        assert_eq!(36, graph.part2());
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let graph = Graph::from(input);
    println!("part1 {}", graph.part1());
    println!("part2 {}", graph.part2());
}
