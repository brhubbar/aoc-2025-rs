use std::{env, process};

fn main() {
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
    println!("Part1: {result}");
}

mod day1;
mod day2;
mod today;
mod utils;
