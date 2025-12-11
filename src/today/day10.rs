use std::ops::BitXor;

use combinations::Combinations;
use log::{debug, trace};
use regex::Regex;

/// Each set of lights is a number, lsb first. Each button completes a bitwise XOR against the
/// indicator bar.
///
/// - Build the desired number from the reverse of the indicator lights.
/// - Run through combinations of buttons picking increasing number of buttons until we get the
///   target number from an XOR.
/// - XOR is commutative, so it's never necessary to press a button more than once.
pub fn part1(contents: &str) -> usize {
    let machines = Vec::from_iter(contents.trim().split('\n').map(Machine::new));

    // Possible series:
    // ([0], [1], [2], ... [len-1], [0,1], [0,2], ..., [0,len-1], [1, 2], [1, 3], ...)
    // I can take advantage of commutativity to just do this all in one big computation.
    // 0
    // 0 xor 1 -> 0,1
    // 0,1 xor 0 -> 1
    // 1 xor 2 -> 1,2
    // 1,2 xor 1 -> 2
    // ..
    //
    // 0
    // 01
    // 012
    // 12
    // 1
    // 2
    // 02

    // 0001
    // 0010
    // 0100
    // 1000
    // 0011
    // 0101
    // 1001
    // 0110
    // 1010
    // 1100
    // 0111
    // 1011
    // 1101
    // 1110
    // 1111
    let mut sum = 0;
    'machine: for machine in machines {
        let n_buttons = machine.buttons.len();
        for pick_n in 1..n_buttons + 1 {
            for combination in Combinations::new(Vec::from_iter(0..n_buttons), pick_n) {
                let mut indicators = 0;
                for idx in combination {
                    indicators = indicators.bitxor(machine.buttons[idx]);
                }
                if indicators == machine.desired_indicators {
                    sum += pick_n;
                    continue 'machine;
                }
            }
        }
    }

    sum
}

/// (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
pub fn part2(contents: &str) -> usize {
    let machines = Vec::from_iter(contents.trim().split('\n').map(Machine::new));
    let mut sum = 0;
    'machine: for machine in machines {
        let n_buttons = machine.buttons.len();
        for pick_n in 1..n_buttons + 1 {
            for combination in Combinations::new(Vec::from_iter(0..n_buttons), pick_n) {
                let mut indicators = 0;
                for idx in combination {
                    indicators = indicators.bitxor(machine.buttons[idx]);
                }
                if indicators == machine.desired_indicators {
                    sum += pick_n;
                    continue 'machine;
                }
            }
        }
    }

    sum
}

trait FromLittleEndian: Sized {
    fn from_little_endian(word: &str) -> Result<Self, std::num::ParseIntError>;
}

impl FromLittleEndian for u16 {
    fn from_little_endian(word: &str) -> Result<Self, std::num::ParseIntError> {
        u16::from_str_radix(&String::from_iter(word.chars().rev()), 2)
    }
}

struct Machine {
    desired_indicators: u16,
    buttons: Vec<u16>,
    joltages: Vec<u32>,
}

impl Machine {
    fn new(entry_from_manual: &str) -> Self {
        let re = Regex::new(r"^\[([#.]+)\] ((?:\((?:\d+,?)+\) )+)\{((?:\d,?)+)\}")
            .expect("Hardcoded regex should be valid.");

        let mut parts = re.captures_iter(entry_from_manual);

        let (_, [desired_indicators, buttons, joltages]) = parts
            .next()
            .expect("Should have indicator lights")
            .extract();

        debug!("Indicators: {desired_indicators}");
        debug!("Buttons: {buttons}");
        debug!("Joltages: {joltages}");

        let desired_indicators =
            u16::from_little_endian(&desired_indicators.replace(".", "0").replace("#", "1"))
                .expect("Should parse just fine");

        let buttons = Vec::from_iter(buttons.trim().split(" ").map(|button| {
            // Trim off the parentheses
            let button = &button[1..button.len() - 1];
            let mut bits = Vec::from_iter(
                button
                    .split(",")
                    .map(|x| x.parse::<u16>().expect("Should be numeric.")),
            );

            // Sort descending. e.g. (4, 2, 1).
            bits.sort_by(|a, b| b.cmp(a));

            trace!("Bits: {bits:?}");

            let mut word = String::new();
            let word_length = bits[0] + 1;
            // Grab the least significant 1 bit from the end of the vec.
            let mut next_true = bits.pop().expect("Should have contents.");
            for bit in 0..word_length {
                trace!("Bit: {bit}, next true: {next_true}");
                if bit == next_true {
                    word.push('1');
                    next_true = match bits.pop() {
                        Some(x) => x,
                        None => break,
                    };
                    continue;
                }
                word.push('0');
            }

            u16::from_little_endian(&word).expect("Should parse")
        }));

        let joltages = Vec::from_iter(
            joltages
                .split(',')
                .map(|x| x.parse::<u32>().expect("Should be numeric.")),
        );

        Self {
            desired_indicators,
            buttons,
            joltages,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    #[test]
    fn test_part1() {
        let _ = env_logger::Builder::new()
            .filter_module("advent_of_code_2025::today::day10", log::LevelFilter::Trace)
            .try_init();
        assert_eq!(part1(INPUT), 7);
    }

    #[test]
    fn test_part2() {
        let _ = env_logger::Builder::new()
            .filter_module("advent_of_code_2025::today::day10", log::LevelFilter::Trace)
            .try_init();
        assert_eq!(part2(INPUT), 33);
    }

    #[test]
    fn test_machine_parses_desired_indicators() {
        let _ = env_logger::Builder::new()
            .filter_module("advent_of_code_2025::today::day10", log::LevelFilter::Trace)
            .try_init();
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let expected = 6; // 0x110;

        let sut = Machine::new(input);
        let actual = sut.desired_indicators;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_machine_parses_buttons() {
        let _ = env_logger::Builder::new()
            .filter_module("advent_of_code_2025::today::day10", log::LevelFilter::Trace)
            .try_init();
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let expected = vec![0b1000, 0b1010, 0b100, 0b1100, 0b101, 0b11];

        let sut = Machine::new(input);
        let actual = sut.buttons;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_machine_parses_joltages() {
        let _ = env_logger::Builder::new()
            .filter_module("advent_of_code_2025::today::day10", log::LevelFilter::Trace)
            .try_init();
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let expected = vec![3, 5, 4, 7];

        let sut = Machine::new(input);
        let actual = sut.joltages;

        assert_eq!(expected, actual);
    }
}
