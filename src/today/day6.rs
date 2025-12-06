use log::trace;
use regex::Regex;
use std::collections::HashMap;

pub fn part1(contents: &str) -> u64 {
    let mut ops_map: HashMap<&str, fn(u64, u64) -> u64> = HashMap::new();
    ops_map.insert("*", u64::saturating_mul);
    ops_map.insert("+", u64::saturating_add);

    let search = Regex::new(r"\s+").expect("Should be a valid regex");

    let mut iterator = contents.trim().split('\n').rev();

    let operations = iterator.next().expect("Should have a line.");
    let operations: Vec<&str> = search.split(operations).collect();
    trace!("Ops: {operations:?}");

    let mut columns: Vec<u64> = search
        .split(iterator.next().expect("Should have a line").trim())
        .map(|x| x.parse::<u64>().expect("Should be a number"))
        .collect();
    trace!("Cols: {columns:?}");

    for line in iterator {
        trace!("Next: {line}");
        let rhss = search
            .split(line.trim())
            .map(|x| x.parse::<u64>().expect("Should be a number"));

        for (idx, rhs) in rhss.enumerate() {
            let operation = ops_map.get(&operations[idx]).expect("Should be found.");
            columns[idx] = operation(columns[idx], rhs);
        }
        trace!("Cols: {columns:?}");
    }

    columns.into_iter().sum()
}
pub fn part2(contents: &str) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::LevelFilter;

    const INPUT: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

    #[test]
    fn test_part1() {
        env_logger::Builder::new()
            .filter_level(LevelFilter::Trace)
            .init();
        assert_eq!(part1(INPUT), 4277556);
    }

    #[test]
    fn test_part2() {
        env_logger::Builder::new()
            .filter_level(LevelFilter::Trace)
            .init();
        assert_eq!(part2(INPUT), 1);
    }
}
