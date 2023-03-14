use std::{collections::HashMap, fmt::Display, ops::RangeInclusive};

fn distance_to_time(distance: i32, cache: &mut HashMap<i32, Option<i32>>) -> Option<i32> {
    if let Some(val) = cache.get(&distance) {
        *val
    } else {
        let mut distance = distance;
        let mut time = 0;
        for sub in 1.. {
            if distance >= sub {
                distance -= sub;
                time += 1;
            } else {
                break;
            }
        }

        let val = if distance == 0 { Some(time) } else { None };

        cache.insert(distance, val);
        val
    }
}
#[test]
fn distance_to_time_works() {
    let mut cache = HashMap::new();
    assert_eq!(distance_to_time(5050, &mut cache), Some(100))
}

fn velocity_to_distance(vel: i32) -> i32 {
    (1 + vel) * (vel / 2) + (vel % 2 * (vel / 2 + 1))
}

#[test]
fn velocity_to_distance_works() {
    assert_eq!(velocity_to_distance(100), 5050);
    assert_eq!(velocity_to_distance(101), 5151);
}

fn y_max(y_range: RangeInclusive<i32>) -> i32 {
    // if you shoot it through it will go through it again
    // the largest step you can take is one ie. taking multiple steps will result in a smaller max
    // therefore the way to work out the max is the largest step you can take for the difference
    // between the start at 0 and the end
    // upper bound is the largest step between start and end
    let min = &y_range.clone().min().unwrap_or(0);
    let velocity = if min < &0 {
        min.abs() - 1
    } else {
        y_range.max().unwrap_or(0)
    };
    velocity_to_distance(velocity)
}

#[test]
fn y_max_works() {
    assert_eq!(y_max(-10..=-5i32), 45)
}

fn part1(y_range: RangeInclusive<i32>) -> i32 {
    y_max(y_range)
}

#[test]
fn part1_works() {
    assert_eq!(part1(-10..=-5i32), 45)
}

fn distance_travellable_in(mut vel: i32, mut distance: i32) -> Option<usize> {
    let mut time = 0usize;
    while distance > 0 && vel > 0 {
        distance -= vel;
        vel -= 1;
        time += 1;
    }

    if distance == 0 {
        Some(time)
    } else {
        None
    }
}

#[test]
fn distance_travellable_in_works() {
    assert_eq!(distance_travellable_in(6, 11), Some(2))
}

fn combo_count(
    distance: i32,
    cache: &mut HashMap<i32, Vec<(usize, i32)>>,
) -> (i32, Vec<(usize, i32)>) {
    if let Some(count) = cache.get(&distance) {
        (distance, count.clone())
    } else {
        let mut combos = vec![(1, distance)];
        let mut dist_cache = HashMap::new();
        let mut hits_zero_vel = false;
        if distance_to_time(distance, &mut dist_cache).is_some() {
            hits_zero_vel = true;
        }

        for vel in (0..=(distance / 2 + 1)).rev() {
            if let Some(time) = distance_travellable_in(vel, distance) {
                combos.push((time, vel))
            }
        }
        if hits_zero_vel {
            let last = combos.last().unwrap();
            let vel = last.1;
            let time = last.0;
            for n in 1..1000 {
                combos.push((time + n, vel));
            }
        }

        cache.insert(distance, combos.clone());
        (distance, combos)

    }
}

#[test]
fn combo_count_works() {
    let mut cache = HashMap::new();
    assert_eq!(combo_count(11, &mut cache), (11, vec![(1, 11), (2, 6)]))
}

fn y_at_t(mut vel: i32, t: usize) -> i32 {
    let mut distance = 0;

    for _ in 0..t {
        distance += vel;
        vel -= 1;
    }

    distance
}

// cache numbers that are possible to reach
// distance: vec[times]
// (time, distance) bool
fn vel_reachable(
    time: usize,
    y: i32,
    cache: &mut HashMap<(usize, i32), Option<i32>>,
) -> Option<i32> {
    if time == 1 {
        return Some(y);
    }
    if let Some(val) = cache.get(&(time, y)) {
        *val
    } else {
        let val = ((y + 1)..y.abs()).find(|&vel| (y_at_t(vel, time) == y));

        cache.insert((time, y), val);

        val
    }
}

fn all_combos(x_range: RangeInclusive<i32>, y_range: RangeInclusive<i32>) -> Vec<(i32, i32)> {
    // for every x point try every number from max to min? denominator effect (max/1 + n + n)
    // get a list of times
    // with list of times work out if y can hit it?
    let mut x_cache: HashMap<i32, Vec<(usize, i32)>> = HashMap::new();
    let mut y_cache: HashMap<(usize, i32), Option<i32>> = HashMap::new();

    y_range
        .flat_map(|y| {
            x_range
                .clone()
                .flat_map(|x| {
                    let (_, times) = combo_count(x, &mut x_cache);

                    times
                        .into_iter()
                        .filter_map(|(time, x_vel)| {
                            let mut reached = vel_reachable(time, y, &mut y_cache);

                            reached
                            .map(|y_vel| {
                                (x_vel, y_vel)
                            })
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
#[derive(PartialEq, Eq)]
struct Vel(i32, i32);

impl Display for Vel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[test]
fn all_combos_test() {
    use prettydiff::diff_slice;
    let mut combos = vec![
        (6, 0),
        (6, 1),
        (6, 2),
        (6, 3),
        (6, 4),
        (6, 5),
        (6, 6),
        (6, 7),
        (6, 8),
        (6, 9),
        (7, -1),
        (7, 0),
        (7, 1),
        (7, 2),
        (7, 3),
        (7, 4),
        (7, 5),
        (7, 6),
        (7, 7),
        (7, 8),
        (7, 9),
        (8, -1),
        (8, -2),
        (8, 0),
        (8, 1),
        (9, -1),
        (9, -2),
        (9, 0),
        (10, -1),
        (10, -2),
        (11, -1),
        (11, -2),
        (11, -3),
        (11, -4),
        (12, -2),
        (12, -3),
        (12, -4),
        (13, -2),
        (13, -3),
        (13, -4),
        (14, -2),
        (14, -3),
        (14, -4),
        (15, -2),
        (15, -3),
        (15, -4),
        (20, -10),
        (20, -5),
        (20, -6),
        (20, -7),
        (20, -8),
        (20, -9),
        (21, -10),
        (21, -5),
        (21, -6),
        (21, -7),
        (21, -8),
        (21, -9),
        (22, -10),
        (22, -5),
        (22, -6),
        (22, -7),
        (22, -8),
        (22, -9),
        (23, -10),
        (23, -5),
        (23, -6),
        (23, -7),
        (23, -8),
        (23, -9),
        (24, -10),
        (24, -5),
        (24, -6),
        (24, -7),
        (24, -8),
        (24, -9),
        (25, -10),
        (25, -5),
        (25, -6),
        (25, -7),
        (25, -8),
        (25, -9),
        (26, -10),
        (26, -5),
        (26, -6),
        (26, -7),
        (26, -8),
        (26, -9),
        (27, -10),
        (27, -5),
        (27, -6),
        (27, -7),
        (27, -8),
        (27, -9),
        (28, -10),
        (28, -5),
        (28, -6),
        (28, -7),
        (28, -8),
        (28, -9),
        (29, -10),
        (29, -5),
        (29, -6),
        (29, -7),
        (29, -8),
        (29, -9),
        (30, -10),
        (30, -5),
        (30, -6),
        (30, -7),
        (30, -8),
        (30, -9),
    ];
    combos.sort();
    let mut result = all_combos(20..=30i32, -10..=-5);
    result.sort();
    result.dedup();
    let expected = combos
        .iter()
        .map(|(x, y)| Vel(*x, *y))
        .collect::<Vec<Vel>>();
    let actual = result
        .iter()
        .map(|(x, y)| Vel(*x, *y))
        .collect::<Vec<Vel>>();
    println!("{}", diff_slice(&expected, &actual));

    assert_eq!(combos.len(), 112);
    assert_eq!(result.len(), 112);
    assert_eq!(combos, result);
}

fn part2(x_range: RangeInclusive<i32>, y_range: RangeInclusive<i32>) -> usize {
    let mut combos = all_combos(x_range, y_range);
    combos.sort();
    combos.dedup();
    println!("{:?}", &combos);

    combos.len()
}

#[test]
fn part2_works() {
    assert_eq!(part2(20..=30i32, -10..=-5), 112)
}

fn main() {
    // x=185..221, y=-122..-74
    let x_range = 185..=221i32;
    let y_range = -122..=-74i32;
    println!("part1: {}", part1(y_range.clone()));
    println!("part2: {}", part2(x_range, y_range));
}
