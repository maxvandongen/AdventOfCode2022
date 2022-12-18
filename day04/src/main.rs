use std::{collections::HashSet, fs};

#[derive(Debug)]
struct JobPair {
    first: Job,
    second: Job,
}
#[derive(Debug)]
struct Job {
    spots_to_do: HashSet<usize>,
}

impl JobPair {
    fn has_unnecessary_job(self) -> bool {
        self.first.fully_contains(&self.second) || self.second.fully_contains(&self.first)
    }

    fn has_any_overlap(self) -> bool {
        self.first.has_any_overlap(&self.second)
    }
}

impl From<&str> for JobPair {
    fn from(value: &str) -> Self {
        let mut parts = value.split(",");
        JobPair {
            first: parts.next().unwrap().into(),
            second: parts.next().unwrap().into(),
        }
    }
}

impl Job {
    fn fully_contains(&self, other: &Job) -> bool {
        self.spots_to_do.is_superset(&other.spots_to_do)
    }

    fn has_any_overlap(&self, other: &Job) -> bool {
        &self.spots_to_do.intersection(&other.spots_to_do).count() > &0
    }
}

impl From<&str> for Job {
    fn from(value: &str) -> Self {
        let mut parts = value.split("-");
        let left = parts.next().unwrap().parse::<usize>().unwrap();
        let right = parts.next().unwrap().parse::<usize>().unwrap();
        Job {
            spots_to_do: HashSet::from_iter(left..=right),
        }
    }
}

fn main() {
    let input = "input.txt";
    let pairs = fs::read_to_string(input)
        .unwrap()
        .split("\n")
        .into_iter()
        .map(|line| JobPair::from(line).has_unnecessary_job())
        .filter(|b| *b)
        .count();

    println!("Found {} pairs with unnecessary jobs", pairs);

    let overlaps = fs::read_to_string(input)
        .unwrap()
        .split("\n")
        .into_iter()
        .map(|line| JobPair::from(line).has_any_overlap())
        .filter(|b| *b)
        .count();

    println!("Found {} pairs with overlapping jobs", overlaps);
}
