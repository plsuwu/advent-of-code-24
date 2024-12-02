use std::{error::Error, fs, str::Lines};

// not "optimal" but neither is life so die mad about it
fn check_report(nums: &Vec<isize>) -> bool {
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
        return true;
    }

    return false;
}

fn get_safe_reports(reports: Lines<'_>) -> Result<isize, Box<dyn Error>> {
    let mut safe = 0;
    for report in reports {
        let nums = report
            .split_whitespace()
            .filter(|ch| *ch != " ")
            .map(|ch| ch.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();

        if check_report(&nums) {
            safe += 1;
        } else {
            for i in 0..nums.len() {
                let mut nums_clone = nums.clone();
                nums_clone.remove(i);

                if check_report(&nums_clone) {
                    safe += 1;
                    break;
                }
            }
        }
    }

    return Ok(safe);
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    let reports = input.lines();

    let safe = get_safe_reports(reports)?;
    println!("safe lines:\n    {}", safe);

    return Ok(());
}
