use log::LevelFilter;
use std::{env, process};

fn main() {
    env_logger::Builder::new()
        .filter(Some("part1"), LevelFilter::Trace)
        .filter(Some("part2"), LevelFilter::Off)
        .filter(None, LevelFilter::Trace)
        .init();
    let mut args = env::args();
    // Path to executable.
    let _ = args.next();
    let Some(path) = args.next() else {
        println!("Please provide a PATH argument!");
        process::exit(1)
    };

    let contents = utils::load(&path).unwrap_or_else(|err| {
        println!("Something went wrong loading the file! {err}");
        process::exit(1)
    });

    let result = today::part1(&contents);
    println!("Part1: {result}");

    let result = today::part2(&contents);
    println!("Part2: {result}");
}

pub mod today;
mod utils;
