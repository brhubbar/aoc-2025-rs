use log::{debug, trace};
/// Input is divided into two parts:
///
/// - Ranges of fresh ingredient ids
/// - Ingredient Ids
///
/// Check if Ingredients fall into the fresh category.
pub fn part1(contents: &str) -> u32 {
    let mut split = contents.split("\n\n").take(2);

    let ranges = split.next().expect("Should have a first entry.");
    let ids = split.next().expect("Should have a second entry.");

    let mut ranges: Vec<Range> = ranges
        .split('\n')
        .map(|x| {
            let mut parts = x.split('-');
            Range(
                parts
                    .next()
                    .expect("Should have a lower bound.")
                    .parse::<u128>()
                    .expect("Should be a number."),
                parts
                    .next()
                    .expect("Should have an upper bound.")
                    .parse::<u128>()
                    .expect("Should be a number."),
            )
        })
        .collect();

    ranges.sort_by_key(|x| x.0);
    trace!("{ranges:?}");

    let ids: Vec<u128> = ids
        .trim()
        .split('\n')
        .map(|x| x.parse::<u128>().expect("Should be a number."))
        .collect();
    trace!("{ids:?}");

    let mut sum = 0;

    'id_check: for id in ids {
        let idx = ranges.partition_point(|x| x.0 <= id);
        debug!("{id}: below {idx}");

        for range in ranges[0..idx].iter().rev() {
            if id <= range.1 {
                debug!("{id} is in {range:?}");
                sum += 1;
                continue 'id_check;
            }
        }
    }

    sum
}
pub fn part2(contents: &str) -> u32 {
    1
}

#[derive(Debug)]
struct Range(u128, u128);
