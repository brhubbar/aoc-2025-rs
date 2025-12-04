use log::{debug, trace};
use std::cmp;

pub fn part1(contents: &str) -> u64 {
    let banks = contents.trim().split('\n');

    let mut joltage = 0;

    for bank in banks {
        joltage += build_battery(bank, 2)
    }

    joltage
}

/// 3123444444484 is too low
pub fn part2(contents: &str) -> u64 {
    let banks = contents.trim().split('\n');

    let mut joltage = 0;

    for bank in banks {
        joltage += build_battery(bank, 12)
    }

    joltage
}

#[derive(Debug)]
struct Found {
    idx: usize,
    value: char,
}

fn build_battery(bank: &str, n_cells: u16) -> u64 {
    // Grab a slice of all but the last one, find the max. Can do this against character codes.
    let bank_size = bank.len();

    let mut bank_joltage = String::new();

    let mut left_bound = 0;

    for right_bound in (0..n_cells).rev() {
        let max = max_char(&bank[left_bound..bank_size - usize::from(right_bound)]);
        trace!("{max:?}");
        bank_joltage.push(max.value);
        // Add one to exclude the max from the search since the lower bound is inclusive. Add
        // instead of set because the index is being measured from the left bound (passing a slice).
        left_bound += max.idx + 1;
    }

    debug!("Bank: {bank} Joltage: {bank_joltage}");

    bank_joltage
        .parse::<u64>()
        .expect("Should be a two-digit number.")
}

fn max_char(string: &str) -> Found {
    trace!("{string}");
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
