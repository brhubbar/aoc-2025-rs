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

/// The operator is left justified.
pub fn part2(contents: &str) -> u64 {
    let mut ops_map: HashMap<&str, fn(u64, u64) -> u64> = HashMap::new();
    ops_map.insert("*", u64::saturating_mul);
    ops_map.insert("+", u64::saturating_add);

    let mut iterator = contents.trim().split('\n').rev();
    let operations: Vec<char> = iterator
        .next()
        .expect("Should have a line.")
        .chars()
        .collect();
    trace!("Ops: {operations:?}");
    // Flip the lines upside down so most-significant digit comes first.
    let lines: Vec<Vec<char>> = iterator.map(|line| line.chars().collect()).rev().collect();
    trace!("Lines: {lines:?}");

    let length = lines
        .iter()
        .map(|line| line.len())
        .max()
        .expect("Should compute a max.");

    let mut sum = 0;

    let mut idx_iterator = (0..length).rev();

    'outer: loop {
        let mut numbers: Vec<u64> = Vec::new();
        // Popping from the right.
        let mut operation: char = ' ';

        while operation == ' ' {
            let Some(idx) = idx_iterator.next() else {
                break 'outer;
            };
            // Here we account for varying lengths of arguments due to whitespace trimming.
            operation = *operations.get(idx).unwrap_or(&' ');
            numbers.push(
                String::from_iter(lines.iter().map(|line| line.get(idx).unwrap_or(&' ')))
                    .trim()
                    .parse()
                    .expect("Should be a number"),
            );
        }

        trace!("Numbers: {numbers:?}");
        trace!("Operation: {operation}");
        // Operation is now + or *.
        match operation {
            '*' => {
                sum += numbers.iter().product::<u64>();
            }
            '+' => {
                sum += numbers.iter().sum::<u64>();
            }
            _ => panic!("Unexpected operation!"),
        }

        // Skip the empty column after an operator.
        match idx_iterator.next() {
            Some(_) => (),
            None => break 'outer,
        };
    }

    sum
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
        assert_eq!(part1(INPUT), 4277556);
    }

    #[test]
    fn test_part2() {
        env_logger::Builder::new()
            .filter_level(LevelFilter::Trace)
            .init();
        assert_eq!(part2(INPUT), 3263827);
    }
}
