use std::{collections::HashMap, fs};

struct Crate {
    text: String,
}

struct Stack {
    stack: Vec<Crate>,
}

struct PlayingField {
    stacks: HashMap<u32, Stack>,
}
#[derive(Debug)]
struct Command {
    from: u32,
    to: u32,
    amount: u32,
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        let mut splits = value.split(" from ");
        let amount = splits
            .next()
            .unwrap()
            .replace("move ", "")
            .parse::<u32>()
            .unwrap();
        let (from, to) = {
            let mut stacks_index = splits.next().unwrap().split(" to ");
            let from = stacks_index.next().unwrap().trim().parse::<u32>().unwrap();
            let to = stacks_index.next().unwrap().trim().parse::<u32>().unwrap();
            (from, to)
        };
        Command {
            from: from,
            to: to,
            amount: amount,
        }
    }
}

impl From<&str> for PlayingField {
    fn from(value: &str) -> Self {
        todo!()
    }
}

fn main() {
    let input_file = "example.txt";
    let input_text = fs::read_to_string(input_file).unwrap();
    // For now skipping the playing field
    let commands: Vec<Command> = input_text
        .split("\n\n")
        .skip(1)
        .next()
        .unwrap()
        .split("\n")
        .into_iter()
        .map(|line| line.into())
        .collect();
    commands
        .iter()
        .for_each(|command| println!("Command: {:?}", command));
}
