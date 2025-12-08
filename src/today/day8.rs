use kiddo::{KdTree, SquaredEuclidean};
use log::trace;

pub fn part1(contents: &str) -> u32 {
    // let tree: KdTree<FixedU32<U0>, 3> = Vec::from_iter(contents.trim().split('\n').map(|line| {
    //     Vec::from_iter(
    //         line.split(',')
    //             .map(|num| FixedU32::from(num.parse::<u32>().expect("Should be a number."))),
    //     )
    // }))
    // .into();

    let points: Vec<[f32; 3]> = Vec::from_iter(contents.trim().split('\n').map(|line| {
        let mut numbers = line
            .split(',')
            .map(|x| x.parse::<f32>().expect("Should be a number."));
        [
            numbers.next().expect("Should have a first number"),
            numbers.next().expect("Should have a second number"),
            numbers.next().expect("Should have a third number"),
        ]
    }));

    let tree: KdTree<_, 3> = (&points).into();

    for (idx, point) in points.iter().enumerate() {
        let nearest = tree
            .nearest_n::<SquaredEuclidean>(point, 2)
            .pop()
            .expect("Should have two results.")
            .item;

        trace!("{idx}: {nearest}");
    }

    1
}
pub fn part2(contents: &str) -> u32 {
    1
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
