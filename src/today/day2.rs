use log::{debug, trace, warn};
/// Add together all ids in the ranges which have the same first and second halves.
/// E.g. 4040, 12431243.
///
/// To find these:
///
/// - Number must have an even number of digits.
///     - If not, add or subtract within range to the nearest number with the correct number of
///       digits.
/// - Take the first half of the number (e.g. 40 from 4000)
/// - If the first half is < second half, round up by 1. E.g. (4050 -> 40 < 50 -> 41)
///
/// Given 40, multiply by 101 to get 4040
/// Given 400, multiply by 1001 to get 400400.
/// So for 40-51, I need (40+41+42+...) * 101.
///
/// To do this mathematically, 4000 / x = 40, x = 100 = 10**2
///
/// The minimum number for 4 digits is 10**(n_digits/2-1) = 10**1
///
/// To convert from half to full: half * ((minimum_for_n_digits * 10) + 1)
///                          e.g. 40   * ((10**1 * 10) + 1)
///                          e.g. 40   * ((10**2) + 1)
///                          e.g. 40   * ((101)
///                          e.g. 4040
///
/// To convert from full to half: full / (minimum_for_n_digits * 10)
///                          e.g. 4000 / (10**1 * 10)
///                          e.g. 4000 / (10**2)
///                          e.g. 40
///
/// To build the bounds, build the low and high minimum numbers. If those are outside the range of
/// the numbers, adjust the halves by one inward to the range.
///
///
pub fn part1(contents: &str) -> u64 {
    let spans = contents.trim().split(',');
    let mut sum: u64 = 0;
    for span in spans {
        let mut ends = span.split('-');
        let low = ends
            .next()
            .expect("Range should take the shape {low}-{high}");
        let high = ends
            .next()
            .expect("Range should take the shape {low}-{high}");
        trace!(target: "part1", "{low}, {high}");
        let (low, high) = filter_range(low, high);
        if low == "0" && high == "0" {
            warn!(target: "part1", "!! SKIPPING {low}-{high}");
            continue;
        }

        let low = split_num(low, true);
        let high = split_num(high, false);
        trace!(target: "part1", "{low}-{high}");

        for x in low..high {
            trace!("{x},");
            let x = x.to_string();
            let mut id = String::new();
            id.push_str(&x);
            id.push_str(&x);
            sum += id.parse::<u64>().expect("id should be numeric.")
        }
    }
    sum
}

/// So do this again, but the sequence repeats two or more times - therefore, no longer working with
/// just halves.
///
/// 40000
///
/// - 44444
///
/// For odd lengths, it's just a single digit
///
/// 400_000 - 500_000
///
/// - 4 4 4 4 4 4
/// - 40 40 40
/// - 404 404
///
/// 450_000 - 500_000
///
/// - 45 45 45
/// - 450 450
///
/// So I think I can re-use my string manipulation, I just have to push more times.
pub fn part2(contents: &str) -> u64 {
    let valid_component_lengths: Vec<Vec<u32>> = vec![
        vec![],        // 0
        vec![],        // 1
        vec![1],       // 2
        vec![1],       // 3
        vec![1, 2],    // 4
        vec![1],       // 5
        vec![1, 3],    // 6
        vec![1],       // 7
        vec![1, 2, 4], // 8
        vec![1, 3],    // 9
        vec![1, 2, 5], // 10
    ];

    let spans = contents.trim().split(',');
    let mut sum: u64 = 0;
    for span in spans {
        let mut ends = span.split('-');
        let low = ends
            .next()
            .expect("Range should take the shape {low}-{high}");
        let high = ends
            .next()
            .expect("Range should take the shape {low}-{high}");
        trace!(target: "part2", "{low}, {high}");
        let (low, high) = filter_range(low, high);
        if low == "0" && high == "0" {
            warn!(target: "part2", "!! SKIPPING {low}-{high}");
            continue;
        }

        let len = high.len();
        for valid_component_length in &valid_component_lengths[len] {
            let substring_length: usize = (valid_component_length * 2)
                .try_into()
                .expect("Should fit in a usize.");
            let low = split_num(&truncate(low, substring_length), true);
            let high = split_num(&truncate(high, substring_length), false);
            trace!(target: "part2", "Parts: {low}-{high}");

            for x in low..high {
                trace!(target: "part2", "Part: {x}");
                let x = x.to_string();
                let mut id = String::new();
                for _ in 0..(len / (substring_length / 2)) {
                    id.push_str(&x);
                }
                trace!(target: "part2", "Id: {id}");
                sum += id.parse::<u64>().expect("id should be numeric.")
            }
        }
    }
    sum
}

/// Check if the range can possibly have a repeated number in it (there is a number with an even
/// number of digits within the range).
fn filter_range<'a>(low: &'a str, high: &'a str) -> (&'a str, &'a str) {
    let n_digits_low = low.len();
    let n_digits_high = high.len();

    if n_digits_low % 2 == 1 && n_digits_high % 2 == 1 {
        if n_digits_low == n_digits_high {
            return ("0", "0");
        }
        // We never hit this, so I don't need to deal with this logic. Whoop, whoop.
        panic!("NOT THE SAME ODD LENGTH")
    }
    (low, high)
}

/// Return the number as needed for computing the repeated ids.
fn split_num(num: &str, is_low: bool) -> u64 {
    let len: u32 = num.len().try_into().expect("Length should fit into a u32.");
    let num = if len % 2 == 1 {
        if is_low {
            debug!("! ROUNDING UP");
            (10_u64.pow(len)).to_string()
        } else {
            debug!("! ROUNDING DOWN");
            ((10_u64.pow(len - 1)) - 1).to_string()
        }
    } else {
        String::from(num)
    };

    let half = num.len() / 2;

    let mut first_half: u64 = num[..half].parse().expect("Num should parse to a u64.");
    let second_half: u64 = num[half..].parse().expect("Num should parse to a u64.");

    debug!("{num} ({len}) -> {first_half} {second_half}");

    if is_low && first_half < second_half || !is_low && first_half <= second_half {
        first_half += 1;
    };

    first_half
}

fn truncate(string: &str, length: usize) -> String {
    String::from_iter(string.chars().take(length))
}
