/// Get the password
///
/// - The dial starts at 50.
/// - Each line of the contents tells you to go left (negative) or right (positive).
/// - Each time you land on zero (dial ranges 0-99), count it
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
        dial %= 100;
        if dial == 0 {
            count += 1;
        }
    }
    count
}
pub fn part2(contents: &str) -> u32 {
    1
}
