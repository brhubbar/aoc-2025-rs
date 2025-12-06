pub fn part1(contents: &str) -> u32 {
    1
}
pub fn part2(contents: &str) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::LevelFilter;

    const INPUT: &str = "";

    #[test]
    fn test_part1() {
        env_logger::Builder::new()
            .filter_level(LevelFilter::Trace)
            .init();
        assert_eq!(part1(INPUT), 1);
    }

    #[test]
    fn test_part2() {
        env_logger::Builder::new()
            .filter_level(LevelFilter::Trace)
            .init();
        assert_eq!(part2(INPUT), 1);
    }
}
