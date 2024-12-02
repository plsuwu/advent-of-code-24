use std::{error::Error, fs};

/// example input:
///
/// ```
/// 7 6 4 2 1
/// 1 2 7 8 9
/// 9 7 6 2 1
/// 1 3 2 4 5
/// 8 6 4 4 1
/// 1 3 6 7 9
/// ```
///
/// - the data above consists of many "reports" (1 report per line).
/// - each report (line) is a list of numbers called levels that are space-delimited
///
/// - reports are considered safe if:
///     * the levels are either all increasing or all decreasing
///     * any two adjacent levels differ by at least one and at most 3, ie:
///             `1 >= level_difference <= 3`
///
/// given the example input, reports can be found safe or unsafe by checking against the
/// above rules:
///     - 7 6 4 2 1  -> safe: all levels are decreasing by a difference of 1 >= diff <= 3
///     - 1 2 7 8 9  -> unsafe: 2 -> 7 is an increase > 3 (5)
///     - 9 7 6 2 1  -> unsafe: 6 -> 2 is a decrese > 3 (4)
///     - 1 3 2 4 5  -> unsafe: 1 -> 3 is an increase but 3 -> 2 is a decrease
///     - 8 6 4 4 1  -> unsafe: 4 -> 4 is a difference < 1 (0)
///     - 1 3 6 7 9  -> safe: all levels are increasing by 1, 2, or 3.
///
/// so, in this example, there are two `(2)` safe reports (lines).

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;

    let mut safe = 0;
    let lines = input.lines();

    for line in lines {
        let nums = line
            .split_whitespace()
            .filter(|ch| *ch != " ")
            .map(|ch| ch.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();

        let trend = nums[0] > nums[1];
        if nums
            .windows(2)
            .map(|win| {
                let curr_diff = (win[0] - win[1]).abs();
                let curr_trend = win[0] > win[1];

                return curr_diff < 4 && curr_diff != 0 && curr_trend == trend;
            })
            .all(|s| s == true)
        {
            safe += 1;
        }
    }
    println!("{}", safe);
    return Ok(());
}
