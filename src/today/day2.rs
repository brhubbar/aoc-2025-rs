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
pub fn part1(contents: &str) -> u128 {
    let spans: std::str::Split<'_, char> = contents.trim().split(',');
    let mut ret = 0;
    for span in spans {
        let mut ends = span.split('-');
        let low = ends
            .next()
            .expect("Range should take the shape {low}-{high}");
        let high = ends
            .next()
            .expect("Range should take the shape {low}-{high}");
        trace!(target: "part2", "{low}, {high}");

        let lo: u128 = low.parse().expect("Should be a number.");
        let hi: u128 = high.parse().expect("Should be a number.");

        for length in low.len()..high.len() + 1 {
            ret += sum_rep(lo, hi, length as u128, 2)
        }
    }
    ret
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
pub fn part2(contents: &str) -> u128 {
    let spans: std::str::Split<'_, char> = contents.trim().split(',');
    let mut ret = 0;
    for span in spans {
        let mut ends = span.split('-');
        let low = ends
            .next()
            .expect("Range should take the shape {low}-{high}");
        let high = ends
            .next()
            .expect("Range should take the shape {low}-{high}");
        trace!(target: "part2", "{low}, {high}");

        let lo: u128 = low.parse().expect("Should be a number.");
        let hi: u128 = high.parse().expect("Should be a number.");

        for length in low.len()..high.len() + 1 {
            let mut rep_counts = vec![0; length / 2 + 1];

            for l in 1..(length / 2 + 1) {
                rep_counts[l] = sum_rep(lo, hi, length as u128, l as u128)
            }
            for l in 1..(length / 2 + 1) {
                if !length.is_multiple_of(l) {
                    continue;
                }

                ret += rep_counts[l];

                for x in 1..l {
                    if l.is_multiple_of(x) {
                        ret -= rep_counts[x];
                    }
                }
            }
        }
    }
    ret
}

/// Copying a different solution from jimm89:
/// https://github.com/jimm89/AdventOfCode2025/blob/main/Day%202/Day%202.ipynb
fn sum_rep(lo: u128, hi: u128, length: u128, rep: u128) -> u128 {
    if !length.is_multiple_of(rep) {
        return 0;
    }

    let rep: u128 = length / rep;

    let lo_rep = 10u128.pow((length / rep - 1) as u32);
    let hi_rep = 10 * lo_rep - 1;

    let mut min;
    if length > (lo.to_string().len() as u128) {
        min = lo_rep;
    } else {
        min = lo;
        min /= (lo_rep * 10).pow((rep - 1) as u32);
        let mut test = min;
        for _ in 0..rep - 1 {
            test = test * (10 * lo_rep) + min
        }
        if test < lo {
            min += 1
        }
    }
    let min = min;

    let mut max;
    if length < (hi.to_string().len() as u128) {
        max = hi_rep;
    } else {
        max = hi;
        max /= (lo_rep * 10).pow((rep - 1) as u32);
        let mut test = max;
        for _ in 0..rep - 1 {
            test = test * (10 * lo_rep) + max;
        }
        if (test) > hi {
            max -= 1;
        }
    }
    let max = max;

    let mut ret = sum_range(min, max);
    let adder = ret;
    for _ in 0..(rep - 1) {
        ret = ret * (10 * lo_rep) + adder;
    }

    ret
}

fn sum_first_n(n: u128) -> u128 {
    n * (n + 1) / 2
}

fn sum_range(min: u128, max: u128) -> u128 {
    sum_first_n(max) - sum_first_n(min - 1)
}
