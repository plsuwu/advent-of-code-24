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
        self.rules.entry(a).or_insert_with(HashSet::new).insert(b);
    }

    fn correct_ordering(&self, update: &[u32]) -> Vec<u32> {
        let mut graph: HashMap<u32, HashSet<u32>> = HashMap::new();
        let pages: HashSet<_> = update.iter().copied().collect();

        for &page in &pages {
            if let Some(after_pages) = self.rules.get(&page) {
                for &after in after_pages {
                    if pages.contains(&after) {
                        graph.entry(page).or_insert_with(HashSet::new).insert(after);
                    }
                }
            }
        }

        let mut in_deg: HashMap<u32, usize> = pages.iter().map(|&page| (page, 0)).collect();
        for (_, after_pages) in &graph {
            for &after in after_pages {
                *in_deg.entry(after).or_insert(0) += 1;
            }
        }

        let mut res = Vec::new();
        let mut queue: VecDeque<_> = in_deg
            .iter()
            .filter(|(_, &count)| count == 0)
            .map(|(&page, _)| page)
            .collect();

        while let Some(page) = queue.pop_front() {
            res.push(page);

            if let Some(after_pages) = graph.get(&page) {
                for &after in after_pages {
                    let count = in_deg.get_mut(&after).unwrap();
                    *count -= 1;
                    if *count == 0 {
                        queue.push_back(after);
                    }
                }
            }
        }

        return res;
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

            if !man.valid_ordering(&update) {
                let corr = man.correct_ordering(&update);
                let mid = corr.len() / 2;

                acc += corr[mid];
            }
        });

    println!("{:?}", acc);

    return Ok(());
}
