use std::collections::{HashMap, HashSet, VecDeque};
use std::{error::Error, fs::read_to_string};

#[derive(Debug)]
struct Updates {
    rules: HashMap<u32, HashSet<u32>>,
}

impl Updates {
    fn new() -> Self {
        return Updates {
            rules: HashMap::new(),
        };
    }

    fn add_rule(&mut self, a: u32, b: u32) {
        self.rules
            .entry(a)
            .or_insert_with(|| HashSet::new())
            .insert(b);
    }

    fn valid_ordering(&self, update: &Vec<u32>) -> bool {
        for (i, &page_a) in update.iter().enumerate() {
            for &page_b in update[i + 1..].iter() {
                if let Some(is_after) = self.rules.get(&page_b) {
                    if is_after.contains(&page_a) {
                        return false;
                    }
                }
            }
        }

        return true;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("src/input.txt")?;
    let sections = input.split("\n\n").collect::<Vec<_>>();

    assert_eq!(sections.len(), 2);
    let mut man = Updates::new();

    sections[0].split('\n').for_each(|r| {
        let rule = r.split('|').collect::<Vec<_>>();
        man.add_rule(
            rule[0].parse::<u32>().unwrap(),
            rule[1].parse::<u32>().unwrap(),
        );
    });

    let mut acc = 0;
    sections[1]
        .split('\n')
        .filter(|u| !u.is_empty())
        .for_each(|u| {
            let update: Vec<u32> = u.split(',').map(|u| u.parse::<u32>().unwrap()).collect();

            if man.valid_ordering(&update) {
                acc += update[update.len() / 2];
            }
        });

    println!("{:?}", acc);
    return Ok(());
}
