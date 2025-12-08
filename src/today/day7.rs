use log::trace;
use std::collections::{HashMap, HashSet};

/// Keep track of every x coordinate of a beam, then just walk down row-by-row.
pub fn part1(contents: &str) -> u32 {
    let mut lines = contents.trim().split('\n');
    let first = lines.next().expect("Should have a line");

    let mut indices: HashSet<usize> = HashSet::with_capacity(first.len());
    indices.insert(
        first
            .find('S')
            .expect("Should have an S in the first line."),
    );

    let mut sum = 0;

    for line in lines {
        trace!("Indices: {indices:?}");
        let mut next_indices: HashSet<usize> = HashSet::with_capacity(first.len());
        for idx in indices {
            if &line[idx..idx + 1] == "^" {
                sum += 1;
                next_indices.insert(idx - 1);
                next_indices.insert(idx + 1);
            } else {
                next_indices.insert(idx);
            }
        }
        indices = next_indices;
    }

    sum
}
pub fn part2(contents: &str) -> u64 {
    let mut lines = contents.trim().split('\n');
    let first = lines.next().expect("Should have a line");

    let mut indices: HashMap<usize, u64> = HashMap::with_capacity(first.len());
    indices.insert(
        first
            .find('S')
            .expect("Should have an S in the first line."),
        1,
    );

    // We start with one timeline
    let mut sum = 1;

    for line in lines {
        trace!("Indices: {indices:?}");
        let mut next_indices: HashMap<usize, u64> = HashMap::with_capacity(first.len());
        for (idx, count) in indices {
            if &line[idx..idx + 1] == "^" {
                // Each time we split, we add a timeline.
                sum += count;
                trace!("New sum: {sum}");
                increment_key(&mut next_indices, idx - 1, Some(count));
                increment_key(&mut next_indices, idx + 1, Some(count));
            } else {
                increment_key(&mut next_indices, idx, Some(count));
            }
        }
        indices = next_indices;
    }

    sum
}

fn increment_key(map: &mut HashMap<usize, u64>, key: usize, count: Option<u64>) {
    map.insert(key, map.get(&key).unwrap_or(&0u64) + count.unwrap_or(1));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn test_part1() {
        let _ = env_logger::Builder::new()
            .filter_module("advent_of_code_2025::today::day7", log::LevelFilter::Trace)
            .try_init();
        assert_eq!(part1(INPUT), 21);
    }

    #[test]
    fn test_part2() {
        let _ = env_logger::Builder::new()
            .filter_module("advent_of_code_2025::today::day7", log::LevelFilter::Trace)
            .try_init();
        assert_eq!(part2(INPUT), 40);
    }
}
