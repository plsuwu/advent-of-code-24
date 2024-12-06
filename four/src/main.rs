use std::error::Error;

const DIRS: [(i32, i32); 8] = [
    (0, 1),     // [c, n]

    (0, -1),    // [n, c]

    (1, 0),     // [n, _],
                // [c, _]

    (-1, 0),    // [c, _],
                // [n, _]

    (1, 1),     // [_, n],
                // [c, _],

    (-1, -1),   // [_, c],
                // [n, _],

    (1, -1),    // [n, _]
                // [_, c]

    (-1, 1),    // [c, _],
                // [_, n]
];

fn is_valid(ix: i32, jx: i32, grid: &Vec<Vec<i32>>) -> bool {
    return ix >= 0 && jx >= 0 && ix < grid.len() as i32 && jx < grid[0].len() as i32;
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

                    if
                        !is_valid(ix, jx, grid)
                        || grid[ix as usize][jx as usize] != prev + 1
                    {
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

fn x_counter(grid: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if
                is_valid(
                    (i + 2).try_into().unwrap(),
                    (j + 2).try_into().unwrap(),
                    grid
                )
            {

                // [x, _, _]
                // [_, y, _]
                // [_, _, z]
                let a = format!(
                    "{}{}{}",
                    grid[i][j],
                    grid[i + 1][j + 1],
                    grid[i + 2][j + 2]
                );

                // [_, _, x]
                // [_, y, _]
                // [z, _, _]
                let b = format!(
                    "{}{}{}",
                    grid[i][j + 2],
                    grid[i + 1][j + 1],
                    grid[i + 2][j]
                );

                if
                    (&a == "123" || &a == "321")
                    && (&b == "123" || &b == "321")
                {
                    count += 1
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

    let res_1 = counter(&grid);
    println!("part one: '{}'", res_1);

    let res_2 = x_counter(&grid);
    println!("part two: '{}'", res_2);

    return Ok(());
}
