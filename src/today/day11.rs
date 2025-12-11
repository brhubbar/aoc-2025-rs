use log::trace;
use std::collections::HashMap;
/// Build a map of device -> devices
/// Grab starting device
/// count_ways_out() -> For each -> device, count_ways_out()
/// Cache results in a second hash map.
pub fn part1(contents: &str) -> u32 {
    let device_map: HashMap<&str, Vec<&str>> =
        HashMap::from_iter(contents.trim().split('\n').map(|line| {
            let mut iter = line.split(": ");
            let from = iter.next().expect("Should have a first entry");
            let to = Vec::from_iter(
                iter.next()
                    .expect("Should have a second entry")
                    .trim()
                    .split(' '),
            );
            (from, to)
        }));

    trace!("Map: {device_map:?}");

    let mut cache: HashMap<&str, u32> = HashMap::with_capacity(device_map.len());

    count_ways_out("you", &device_map, &mut cache)
}
pub fn part2(contents: &str) -> u32 {
    1
}

fn count_ways_out(
    start: &str,
    map: &HashMap<&str, Vec<&str>>,
    cache: &mut HashMap<&str, u32>,
) -> u32 {
    if start == "out" {
        return 1;
    }

    match cache.get(start) {
        Some(&count) => count,
        None => {
            let target_devices = map.get(start).expect("Device should exist.");

            target_devices
                .iter()
                .map(|device| count_ways_out(device, map, cache))
                .sum()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    const INPUT_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"

    #[test]
    fn test_part1() {
        let _ = env_logger::Builder::new()
            .filter_module("advent_of_code_2025::today::day11", log::LevelFilter::Trace)
            .try_init();
        assert_eq!(part1(INPUT_1), 5);
    }

    #[test]
    fn test_part2() {
        let _ = env_logger::Builder::new()
            .filter_module("advent_of_code_2025::today::day11", log::LevelFilter::Trace)
            .try_init();
        assert_eq!(part2(INPUT_2), 2);
    }
}
