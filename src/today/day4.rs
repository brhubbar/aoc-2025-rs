use log::debug;
/// Generate an equivalent array of scores. The score for a space is the number of adjacent
/// rolls. Iterate through the array, for each '@', add 1 to the score of the surrounding spaces. Do
/// that by computing the indices forward and backward, truncating out-of-bounds ones.
///
/// For a grid:
///
/// 0 1 2
/// 3 4 5
/// 6 7 8
///
/// The adjacent locations are -1, +1, -width, -width-1, -width+1, +width, +width-1, +width+1
pub fn part1(contents: &str) -> usize {
    let width = contents
        .split('\n')
        .next()
        .expect("Should have at least one newline.")
        .len();
    debug!(target: "part1", "Floor width: {width:?}");

    // Flatten into a single row so we can enumerate and map 1:1 to the scores array.
    let flattened = contents.trim().replace("\n", "");
    let length = flattened.len();

    let scores = score_floor(&flattened, width, length);

    debug!(target: "part1", "{scores:?}");

    scores
        .into_iter()
        .filter(|x| *x < 4)
        .collect::<Vec<u32>>()
        .len()
}
pub fn part2(contents: &str) -> usize {
    let width = contents
        .split('\n')
        .next()
        .expect("Should have at least one newline.")
        .len();

    // Flatten into a single row so we can enumerate and map 1:1 to the scores array.
    let mut flattened = contents.trim().replace("\n", "");
    let length = flattened.len();

    let mut sum = 0;
    loop {
        let scores = score_floor(&flattened, width, length);
        let mut removed = 0;
        for (idx, score) in scores.into_iter().enumerate() {
            if score < 4 {
                flattened.replace_range(idx..idx + 1, "x");
                removed += 1;
            }
        }
        if removed == 0 {
            return sum;
        }

        sum += removed;
        // // Rebuild from scratch, dropping the rolls removed. This isn't super efficient but Rust is
        // // picky about indexing into strings otherwise.
        // flattened = for score in scores {
        //     if score > 4 {
        //         flattened
        //     }
        // }
    }
}

fn score_floor(floor: &str, width: usize, length: usize) -> Vec<u32> {
    let mut scores = vec![0; length];

    for (idx, character) in floor.chars().enumerate() {
        if character != '@' {
            // Mark this as inaccessible to skip counting it.
            scores[idx] = 9;
            continue;
        }
        let adjacents = get_adjacent_idxs(idx, width, length);
        for adjacent in adjacents {
            scores[adjacent] += 1;
        }
    }

    scores
}

fn get_adjacent_idxs(idx: usize, width: usize, length: usize) -> Vec<usize> {
    let is_on_left = idx.is_multiple_of(width);
    let is_on_right = idx % width == width - 1;

    let mut adjacents = Vec::with_capacity(8);

    // Use wrapping sub to allow overflow (negative values become ~u32.MAX)
    if !is_on_left {
        // Top left.
        adjacents.push(idx.wrapping_sub(width + 1));
        // mid left.
        adjacents.push(idx.wrapping_sub(1));
        // bottom left.
        adjacents.push(idx + width - 1);
    }
    if !is_on_right {
        // Top right.
        adjacents.push(idx.wrapping_sub(width - 1));
        // mid right.
        adjacents.push(idx + 1);
        // bottom right.
        adjacents.push(idx + width + 1);
    }

    // Top.
    adjacents.push(idx.wrapping_sub(width));
    // Bottom.
    adjacents.push(idx + width);
    // Filter out anything out of bounds above or below. Only need one comparison thanks to overflow
    // above.
    adjacents.into_iter().filter(|x| *x < length).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 43);
    }
}
