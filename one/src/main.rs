use std::{error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let buffer = read_to_string("./src/input.txt")?;

    let mut list_one = Vec::new();
    let mut list_two = Vec::new();

    for line in buffer.lines() {
        let mut lists = line
            .split_whitespace()
            .filter(|c| *c != " ")
            .collect::<Vec<_>>();

        list_two.push(lists.pop());
        list_one.push(lists.pop());
    }

    list_one.sort();
    list_two.sort();

    let mut acc = 0;
    for (i, num) in list_one.iter().enumerate() {
        let distance: isize = str::parse::<isize>(num.unwrap()).unwrap()
            - str::parse::<isize>(list_two[i].unwrap()).unwrap();

        acc += distance.abs();
    }

    println!("{}", acc);
    return Ok(());
}
