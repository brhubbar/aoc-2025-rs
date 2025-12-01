/// Get the password
///
/// - The dial starts at 50.
/// - Each line of the contents tells you to go left (negative) or right (positive).
/// - Each time you land on zero (dial ranges 0-99), count it.
/// - Password is the count.
pub fn part1(contents: &str) -> u32 {
    let mut dial = 50;
    let mut count = 0;
    for line in contents.lines() {
        let sign: i32 = if line.starts_with("L") { -1 } else { 1 };
        let distance: i32 = line[1..]
            .parse()
            .expect(r"Turn does not take the form `[LR]\d+`!");
        dial += sign * distance;
        dial = wrap(dial).dial;
        if dial == 0 {
            count += 1;
        }
    }
    count
}

/// Get the password
///
/// - The dial starts at 50.
/// - Each line of the contents tells you to go left (negative) or right (positive).
/// - Each time you pass zero (dial ranges 0-99), count it.
/// - Password is the count.
///
/// 6911 was too high.
/// 6772 was too low.
pub fn part2(contents: &str) -> u32 {
    let mut dial = 50;
    let mut count = 0;
    for line in contents.lines() {
        let sign: i32 = if line.starts_with("L") { -1 } else { 1 };
        let distance: i32 = line[1..]
            .parse()
            .expect(r"Turn does not take the form `[LR]\d+`!");
        // Correct for the wrap that happens when turning left from zero.
        if dial == 0 && sign == -1 {
            count -= 1
        }
        let result = wrap(dial + sign * distance);
        dial = result.dial;
        count += result.click_to_zero_count;
        // println!("{line} -> {dial}: {count}");
    }
    count
}

fn wrap(dial: i32) -> Wrap {
    // Short circuit leftward turns of less than 100 to 0.
    if dial == 0 {
        return Wrap {
            dial: 0,
            click_to_zero_count: 1,
        };
    }

    let mut wrapped = dial;
    let mut count = 0;
    loop {
        if wrapped >= 100 {
            wrapped -= 100;
            count += 1;
        } else if wrapped < 0 {
            wrapped += 100;
            count += 1;
            // leftward turns of greater than 100 to 0 need one extra count (e.g. 55 - 155 clicks on
            // zero twice).
            if wrapped == 0 {
                count += 1;
                break;
            }
        } else {
            break;
        }
    }
    Wrap {
        dial: wrapped,
        click_to_zero_count: count,
    }
}

struct Wrap {
    dial: i32,
    click_to_zero_count: u32,
}
