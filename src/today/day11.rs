use log::{info, trace, warn};
use std::collections::HashMap;
/// Build a map of device -> devices
/// Grab starting device
/// count_ways_out() -> For each -> device, count_ways_out()
/// Cache results in a second hash map.
pub fn part1(contents: &str) -> u64 {
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

    count_ways_out("you", "out", &device_map, None, None)
}

/// Assume it's an acyclic graph, otherwise my day would've been effed up.
///
/// This means we go either svr -> fft -> dac -> out *or* svr -> dac -> fft -> out.
///
/// I can confirm this by simply running that.
///
/// Then, the trip is broken into three legs - getting to fft or dac, getting to the other, getting
/// to out. Each leg converges on one point, so the total number of paths is the product of the
/// number of paths for each leg.
pub fn part2(contents: &str) -> u64 {
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

    let svr_fft = count_ways_out("svr", "fft", &device_map, None, None);
    let svr_dac = count_ways_out("svr", "dac", &device_map, None, None);
    let dac_fft = count_ways_out("dac", "fft", &device_map, None, None);
    let fft_dac = count_ways_out("fft", "dac", &device_map, None, None);
    let dac_out = count_ways_out("dac", "out", &device_map, None, None);
    let fft_out = count_ways_out("fft", "out", &device_map, None, None);
    info!("svr->fft: {svr_fft}",);
    info!("svr->dac: {svr_dac}",);
    info!("dac->fft: {dac_fft}",);
    info!("fft->dac: {fft_dac}",);
    info!("dac->out: {dac_out}",);
    info!("fft->out: {fft_out}",);

    let leg_1: u64;
    let leg_2: u64;
    let leg_3: u64;

    match (dac_fft.eq(&0), fft_dac.eq(&0)) {
        (true, false) => {
            leg_1 = svr_fft;
            leg_2 = fft_dac;
            leg_3 = dac_out;
        }
        (false, true) => {
            leg_1 = svr_dac;
            leg_2 = dac_fft;
            leg_3 = fft_out;
        }
        _ => panic!("There's a cycle or we never hit either!!"),
    }

    leg_1 * leg_2 * leg_3
}

fn count_ways_out<'a>(
    from: &'a str,
    to: &'a str,
    map: &HashMap<&'a str, Vec<&'a str>>,
    cache: Option<&mut HashMap<&'a str, u64>>,
    depth: Option<usize>,
) -> u64 {
    let cache = match cache {
        Some(c) => c,
        None => &mut HashMap::with_capacity(map.len()),
    };

    let depth = match depth {
        Some(d) => d + 1,
        None => 0,
    };

    if depth > map.len() {
        warn!("Cycle detected at {from} -> {to}.");
        return 0;
    }

    if from == to {
        return 1;
    } else if from == "out" {
        // To is not out and we made it to out, which doesn't map to anything else.
        return 0;
    }

    match cache.get(from) {
        Some(&count) => count,
        None => {
            trace!("Counting from {from} to {to} for the first time. Depth: {depth}");
            let target_devices = map.get(from).expect("Device should exist.");

            let ret = target_devices
                .iter()
                .map(|device| count_ways_out(device, to, map, Some(cache), Some(depth)))
                .sum();

            cache.insert(from, ret);

            trace!("Got {ret} for {from} to {to}. Depth: {depth}");
            ret
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
";

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
