use log::trace;

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
pub fn part2(contents: &str) -> u32 {
    1
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
        assert_eq!(part2(INPUT), 1);
    }
}
