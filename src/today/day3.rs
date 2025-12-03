use log::{debug, trace, warn};
use std::cmp;

pub fn part1(contents: &str) -> u32 {
    let banks = contents.trim().split('\n');

    let mut joltage = 0;

    for bank in banks {
        // Grab a slice of all but the last one, find the max. Can do this against character codes.
        let bank_size = bank.len();

        let mut bank_joltage = String::new();

        let Found {
            idx: max_idx,
            value: max,
        } = max_char(&bank[..bank_size - 1]);

        let mut bank_joltage = String::from(max);

        let Found { idx: _, value: max } = max_char(&bank[max_idx + 1..]);

        bank_joltage.push(max);
        debug!("Bank: {bank} Joltage: {bank_joltage}");

        joltage += bank_joltage
            .parse::<u32>()
            .expect("Should be a two-digit number.")

        // Grab a slice of max + 1 to end and find the max.
        // Assemble the number.
        // Sum.
    }

    joltage
}

pub fn part2(contents: &str) -> u32 {
    1
}

struct Found {
    idx: usize,
    value: char,
}

fn max_char(string: &str) -> Found {
    let mut max = char::from_u32(0).expect("0 should be a valid character code.");
    let mut max_idx = 0;

    for (idx, char) in string.chars().enumerate() {
        let new_max = cmp::max(Some(char), Some(max))
            .expect("Max comparison of to Some() should return Some.");

        if new_max == max {
            continue;
        }

        max = new_max;
        max_idx = idx;
    }
    Found {
        idx: max_idx,
        value: max,
    }
}
