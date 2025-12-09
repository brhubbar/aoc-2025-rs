use kiddo::{KdTree, SquaredEuclidean};
use log::trace;
use std::collections::HashSet;

/// 7106 is too low
pub fn part1(contents: &str) -> usize {
    // let tree: KdTree<FixedU32<U0>, 3> = Vec::from_iter(contents.trim().split('\n').map(|line| {
    //     Vec::from_iter(
    //         line.split(',')
    //             .map(|num| FixedU32::from(num.parse::<u32>().expect("Should be a number."))),
    //     )
    // }))
    // .into();

    let points: Vec<[i64; 3]> = Vec::from_iter(contents.trim().split('\n').map(|line| {
        let mut numbers = line
            .split(',')
            .map(|x| x.parse::<i64>().expect("Should be a number."));
        [
            numbers.next().expect("Should have a first number"),
            numbers.next().expect("Should have a second number"),
            numbers.next().expect("Should have a third number"),
        ]
    }));

    // let tree: KdTree<_, 3> = (&points).into();

    let mut distances: Vec<Distance> = Vec::with_capacity(points.len() / 2 * (points.len() + 1));

    for (idx_a, a) in points.iter().enumerate() {
        // let nearest = tree
        //     .nearest_n::<SquaredEuclidean>(point, 2)
        //     .pop()
        //     .expect("Should have two results.")
        //     .item;

        // trace!("{idx}: {nearest}");

        for (idx_b, b) in points.iter().enumerate().skip(idx_a + 1) {
            let dist = (a[0] - b[0]).pow(2) + (a[1] - b[1]).pow(2) + (a[2] - b[2]).pow(2);

            distances.push(Distance {
                a: idx_a,
                b: idx_b,
                dist,
            });
        }
    }

    distances.sort_by_key(|x| x.dist);
    for distance in &distances {
        trace!("{distance}");
    }

    let mut circuits: Vec<HashSet<usize>> = Vec::new();

    let n_connections = if points.len() < 500 { 10 } else { 1000 };
    for (n_made, distance) in distances.iter().enumerate() {
        if n_made == n_connections {
            break;
        }

        trace!("Circuits: {circuits:?}");
        // Each end of the connection may be in a separate circuit.
        let mut sets: Vec<&mut HashSet<usize>> = circuits
            .iter_mut()
            .filter(|circuit| circuit.contains(&distance.a) || circuit.contains(&distance.b))
            .collect();
        trace!("Sets: {sets:?}");

        let set: &mut HashSet<usize> = match sets.len() {
            0 => {
                // Neither is in a circuit; create a new one.
                let set = HashSet::new();
                circuits.push(set);
                circuits.last_mut().unwrap()
            }
            1 => {
                // One or both are already in a single circuit.
                sets[0]
            }
            2 => {
                // They are in different circuits; we need to merge them.
                let set = sets.pop().expect("The length is known to be 2");
                let other = sets.pop().expect("The length is known to be 2");
                set.extend(other.drain());
                set
            }
            _ => {
                panic!("More than two sets found!");
            }
        };

        set.insert(distance.a);
        set.insert(distance.b);

        trace!("{circuits:?}");
    }

    let mut sizes: Vec<usize> = circuits.iter().map(|circuit| circuit.len()).collect();
    sizes.sort();
    sizes[sizes.len() - 3..].iter().product()
}
pub fn part2(contents: &str) -> u32 {
    1
}

struct Distance {
    a: usize,
    b: usize,
    dist: i64,
}

impl std::fmt::Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <-{}-> {}", self.a, self.dist, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn test_part1() {
        let _ = env_logger::Builder::new()
            .filter_module("advent_of_code_2025::today::day8", log::LevelFilter::Trace)
            .try_init();
        assert_eq!(part1(INPUT), 40);
    }

    #[test]
    fn test_part2() {
        let _ = env_logger::Builder::new()
            .filter_module("advent_of_code_2025::today::day8", log::LevelFilter::Trace)
            .try_init();
        assert_eq!(part2(INPUT), 1);
    }
}
