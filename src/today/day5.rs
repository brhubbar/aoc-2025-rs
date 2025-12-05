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
    trace!(target: "part1", "{ranges:?}");

    let ids: Vec<u128> = ids
        .trim()
        .split('\n')
        .map(|x| x.parse::<u128>().expect("Should be a number."))
        .collect();
    trace!(target: "part1", "{ids:?}");

    let mut sum = 0;

    'id_check: for id in ids {
        let idx = ranges.partition_point(|x| x.0 <= id);
        debug!(target: "part1", "{id}: below {idx}");

        for range in ranges[0..idx].iter().rev() {
            if id <= range.1 {
                debug!(target: "part1", "{id} is in {range:?}");
                sum += 1;
                continue 'id_check;
            }
        }
    }

    sum
}

/// I need to find overlapping ranges.
///
/// I think what I can do is check if each bound is in any other ranges..
///
/// 3-5
/// 10-14
/// 16-20
/// 12-18
///
/// |----|----|----|----|
///    xxx
///           xxxxx
///                 xxxxx
///             xxxxxxx
///             *** ***
///    xxx    xx       xx
///
/// |----|----|----|----|
///    xxx
///           xxxxxxx
///           xxxxx
///                 xxxxx
///             *** ***
///    xxx    xx       xx
///
///
/// |----|----|----|----|
///    xxx
///           xxxxxxx
///            xxxxx
///                 xxxxx
///             *** ***
///    xxx    xx       xx
///
/// 3 is out of all ranges
/// 5 is out of all ranges
/// 10 is out of all ranges
/// 14 is in 12-18
/// 16 is in 12-18
/// 20 is out of all ranges
/// 12 is in 10-14
/// 18 is in 16-20
///
/// This doesn't consider if a bound falls into multiple ranges.
///
/// Sum all ranges
///
/// Build a sum of all doubles
///
/// For all lower bounds, see if they fall into any ranges.
/// If they do, take the lesser of the upper bounds of those two ranges to compute the double
/// counted sum.
///
/// For each range
///
/// - Check if the lower bound falls into any other ranges.
/// - If it does, adjust the lower bound to the upper bound of that range + 1.
/// - Check if the upper bound falls into any other ranges.
/// - If it does, adjust the upper bound to the lower bound of that range - 1.
///
/// Sort them first
///
/// 3-5
/// 10-14
/// 12-18
/// 16-20
///
/// 3 doesn't fall in any other range
/// 5 doesn't fall in any other range
/// sum += 3 (5-3+1)
///
/// 10 doesn't fall into any other range
/// 14 falls into 12-18
/// 11 doesn't fall into any other range
/// sum += 2 (11-10+1)
///
/// 12 doesn't fall into any other range
/// 18 falls into 16-20
/// 15 doesn't fall into any other range
/// sum += 4 (15-12+1)
///
/// 16-20 is the last range
/// sum += 5 (20-16+1)
///
/// Pop ranges out as I do them so I don't double discount.
///
/// 399602954925847 is too high
/// 356792272643926 is too high
/// 345995423801866
pub fn part2(contents: &str) -> u128 {
    let mut split = contents.split("\n\n").take(2);

    let ranges = split.next().expect("Should have a first entry.");

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

    let mut sum = 0;

    'outer: for idx in 0..ranges.len() {
        let &Range(mut lower, mut upper) = &ranges[idx];
        debug!(target: "part2", "start {lower}-{upper}");

        // Skip any ranges which lie fully within (not sharing a boundary) another range. I
        // already account for ones that share a boundary when looking forward. Since this is
        // sorted, it can only be fully consumed by a range before it.
        for other_range in &ranges[0..idx] {
            if lower > other_range.0 && upper < other_range.1 {
                continue 'outer;
            }
        }

        for other_range in &ranges[idx + 1..] {
            if lower >= other_range.0 && lower <= other_range.1 {
                lower = other_range.1 + 1;
            }
            if upper >= other_range.0 && upper <= other_range.1 {
                upper = other_range.0 - 1;
            }
        }

        if upper < lower {
            debug!(target: "part2", "Skipping - fully consumed.");
            continue;
        }

        debug!(target: "part2", "trunc {lower}-{upper}");
        sum += upper - lower + 1;
        trace!(target: "part2", "sum: {sum}");
    }

    sum
}

#[derive(Debug)]
struct Range(u128, u128);
