pub fn part1(contents: &str) -> u32 {
    1
}
pub fn part2(contents: &str) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn test_part1() {
        let _ = env_logger::Builder::new()
            .filter_module("advent_of_code_2025::today::day8", log::LevelFilter::Trace)
            .try_init();
        assert_eq!(part1(INPUT), 1);
    }

    #[test]
    fn test_part2() {
        let _ = env_logger::Builder::new()
            .filter_module("advent_of_code_2025::today::day8", log::LevelFilter::Trace)
            .try_init();
        assert_eq!(part2(INPUT), 1);
    }
}
