use log::trace;
use regex::Regex;

// 552 too low.
pub fn part1(contents: &str) -> u32 {
    let mut shapes: Vec<&str> = Vec::from_iter(contents.trim().split("\n\n"));
    // The last one is all of the trees.
    let trees = Vec::from_iter(
        shapes
            .pop()
            .expect("Should have more than one whitespace line.")
            .split('\n')
            .map(Tree::new),
    );

    trees
        .iter()
        .filter(|tree| {
            let has = tree.area();
            let required = tree.required_non_overlapping_area();

            trace!("Has: {has}, Needs: {required}");
            has >= required
        })
        .count()
        .try_into()
        .expect("Should fit in u32")
}
pub fn part2(contents: &str) -> u32 {
    1
}

#[derive(PartialEq, std::fmt::Debug)]
struct Tree {
    x: u32,
    y: u32,
    gifts: Vec<u32>,
}

impl Tree {
    fn new(line: &str) -> Self {
        let re = Regex::new(r"^(\d+)x(\d+): ([\d ]+)$").expect("Hardcoded regex should be valid.");

        let results = re.captures(line).expect("Should find results.");

        Tree {
            x: results.get(1).unwrap().as_str().parse().unwrap(),
            y: results.get(2).unwrap().as_str().parse().unwrap(),
            gifts: Vec::from_iter(
                results
                    .get(3)
                    .unwrap()
                    .as_str()
                    .trim()
                    .split(' ')
                    .map(|s| s.parse::<u32>().unwrap()),
            ),
        }
    }

    fn area(&self) -> u32 {
        self.x * self.y
    }

    fn required_non_overlapping_area(&self) -> u32 {
        self.gifts.iter().sum::<u32>() * 9
    }

    fn required_perfectly_overlapping_area(&self, shapes: &Vec<&str>) -> u32 {
        let areas = Vec::from_iter(
            shapes
                .iter()
                .map(|x| x.chars().filter(|c| *c == '#').count()),
        );

        let mut sum: usize = 0;
        for idx in 0..self.gifts.len() {
            sum += &(self.gifts[idx] as usize) * &areas[idx];
        }
        sum.try_into().expect("Should fit in u32.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

    #[test]
    fn test_part1() {
        let _ = env_logger::Builder::new()
            .filter_module("advent_of_code_2025::today::day12", log::LevelFilter::Trace)
            .try_init();
        assert_eq!(part1(INPUT), 2);
    }

    #[test]
    fn test_part2() {
        let _ = env_logger::Builder::new()
            .filter_module("advent_of_code_2025::today::day12", log::LevelFilter::Trace)
            .try_init();
        assert_eq!(part2(INPUT), 1);
    }

    #[test]
    fn test_parses_tree() {
        let input = "4x4: 0 0 0 0 2 0";
        let expected = Tree {
            x: 4,
            y: 4,
            gifts: vec![0, 0, 0, 0, 2, 0],
        };

        let actual = Tree::new(input);

        assert_eq!(expected, actual);
    }
}
