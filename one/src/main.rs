use std::{collections::HashMap, error::Error, fs::read_to_string};

/// part one
fn get_diff(list_one: &mut Vec<isize>, list_two: &mut Vec<isize>) -> Result<isize, Box<dyn Error>> {
    list_one.sort();
    list_two.sort();

    let mut acc = 0;
    for nums in list_one.iter().zip(list_two.iter_mut()) {
        let (a, b) = nums;
        let distance = *a - *b;

        acc += distance.abs();
    }

    return Ok(acc);
}

/// part two
fn get_similarity(
    list_one: &mut Vec<isize>,
    list_two: &mut Vec<isize>,
) -> Result<isize, Box<dyn Error>> {
    list_two.sort();

    let mut scores = HashMap::new();
    for num in list_two.iter() {
        if !scores.contains_key(num) {
            scores.insert(*num, *num);
        } else {
            let curr = scores.get(num).unwrap() + num;
            scores.insert(*num, curr);
        }
    }

    let mut acc = 0;
    for num in list_one.iter() {
        let exists = match scores.get(num) {
            Some(val) => *val,
            None => 0,
        };

        acc += exists;
    }

    return Ok(acc);
}

fn main() -> Result<(), Box<dyn Error>> {
    let buffer = read_to_string("./src/input.txt")?;

    let mut list_one = Vec::new();
    let mut list_two = Vec::new();

    for line in buffer.lines() {
        let mut lists = line
            .split_whitespace()
            .filter(|c| *c != " ")
            .map(|c| c.parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        list_two.push(lists.pop().unwrap());
        list_one.push(lists.pop().unwrap());
    }

    let diff = get_diff(&mut list_one, &mut list_two);
    println!("part 1 - accumulated distance\n     {}", diff.unwrap());

    let sim = get_similarity(&mut list_one, &mut list_two).unwrap();
    println!("part 2 - similarity score\n     {}", sim);

    return Ok(());
}
