use log::trace;
use std::{cmp::Ordering, rc::Rc};

pub fn part1(contents: &str) -> u64 {
    let corners: Vec<Point> = contents
        .trim()
        .split('\n')
        .map(|line| {
            Point::from_iter(
                line.split(',')
                    .map(|x| x.parse::<u64>().expect("Should be numeric")),
            )
        })
        .collect();

    trace!("Corners: {corners:?}");

    let mut max = 0;
    for (idx, corner) in corners.iter().enumerate() {
        for other_corner in corners.iter().skip(idx + 1) {
            max = std::cmp::max(max, corner.area(other_corner));
        }
    }

    max
}

/// ..............
/// .......#XXX#..
/// .......X...X..
/// ..#XXXX#...X..
/// ..X........X..
/// ..#XXXXXX#.X..
/// .........X.X..
/// .........#X#..
/// ..............
pub fn part2(contents: &str) -> u64 {
    let points: Vec<Rc<Point>> = contents
        .trim()
        .split('\n')
        .map(|line| {
            Rc::new(Point::from_iter(
                line.split(',')
                    .map(|x| x.parse::<u64>().expect("Should be numeric")),
            ))
        })
        .collect();

    let mut corners: Vec<Corner> = Vec::with_capacity(points.len());
    for idx in 0..points.len() {
        let l = points
            .get(idx.wrapping_sub(1))
            .unwrap_or_else(|| points.iter().last().expect("Should not be empty."));
        let r = points.get(idx + 1).unwrap_or_else(|| &points[0]);
        corners.push(Corner::build(&points[idx], l, r));
    }

    let mut max = 0;
    for (idx, corner) in corners.iter().enumerate() {
        // For each corner, check if each other corner is in one or both directions from the current
        // corner. The directions of the other corner don't matter here, because next we check if
        // any edges are crossed in transit to that corner.
    }

    max
}

#[derive(Debug)]
struct Point(u64, u64);

impl Point {
    fn area(&self, other: &Point) -> u64 {
        (self.0.abs_diff(other.0) + 1) * (self.1.abs_diff(other.1) + 1)
    }
}

impl FromIterator<u64> for Point {
    fn from_iter<T: IntoIterator<Item = u64>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Point(
            iter.next().expect("Should have two values"),
            iter.next().expect("Should have two values"),
        )
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_points(lhs: &Point, rhs: &Point) -> Self {
        let x = rhs.0.cmp(&lhs.0);
        let y = rhs.1.cmp(&lhs.1);

        match (x, y) {
            (Ordering::Equal, Ordering::Less) => Self::Down,
            (Ordering::Equal, Ordering::Greater) => Self::Up,
            (Ordering::Less, Ordering::Equal) => Self::Left,
            (Ordering::Greater, Ordering::Equal) => Self::Right,
            (Ordering::Equal, Ordering::Equal) => panic!("Points are coincident."),
            _ => panic!("Points are not orthogonal."),
        }
    }
}

struct Corner {
    p: Rc<Point>,
    /// Vector of the other end of each edge.
    edges: Vec<Rc<Point>>,
    directions: Vec<Direction>,
}

impl Corner {
    fn build(center: &Rc<Point>, a: &Rc<Point>, b: &Rc<Point>) -> Self {
        Self {
            p: Rc::clone(center),
            edges: vec![Rc::clone(a), Rc::clone(b)],
            directions: vec![
                Direction::from_points(center, a),
                Direction::from_points(center, b),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn test_part1() {
        let _ = env_logger::Builder::new()
            .filter_module("advent_of_code_2025::today::day9", log::LevelFilter::Trace)
            .try_init();
        assert_eq!(part1(INPUT), 50);
    }

    #[test]
    fn test_part2() {
        let _ = env_logger::Builder::new()
            .filter_module("advent_of_code_2025::today::day9", log::LevelFilter::Trace)
            .try_init();
        assert_eq!(part2(INPUT), 24);
    }
}
