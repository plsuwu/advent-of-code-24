use std::error::Error;

const DIRS: [(i32, i32); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

fn is_valid(ix: i32, jx: i32, grid: &Vec<Vec<i32>>, prev: i32) -> bool {
    let bounded =
            ix >= 0
         && jx >= 0
         && ix < grid.len() as i32
         && jx < grid[0].len() as i32;

    if !bounded {
        return false;
    }

    let curr = grid[ix as usize][jx as usize];
    return bounded && curr == prev + 1;
}

fn counter(grid: &Vec<Vec<i32>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            for (dx, dy) in DIRS.iter() {
                let mut valid = true;
                let mut prev = -1;

                for k in 0..4 {
                    let ix = i as i32 + dx * k as i32;
                    let jx = j as i32 + dy * k as i32;

                    if !is_valid(ix, jx, grid, prev) {
                        valid = false;
                        break;
                    }

                    let curr = grid[ix as usize][jx as usize];

                    prev = curr;
                }

                if valid {
                    count += 1;
                }
            }
        }
    }

    return count;
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("./src/input.txt")?;
    let grid: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    return match ch {
                        'X' => 0,
                        'M' => 1,
                        'A' => 2,
                        'S' => 3,
                        _ => 4,
                    };
                })
                .collect()
        })
        .collect();

    let res = counter(&grid);
    println!("xmas appears '{}' times", res);

    return Ok(());
}
