use std::{collections::HashMap, fs};

struct Crate {
    text: String,
}

struct PlayingField {
    stacks: HashMap<u32, Vec<Crate>>,
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
        let mut row_iter = value.split("\n").collect::<Vec<&str>>().into_iter().rev();

        let stack_count = row_iter
            .next()
            .unwrap()
            .split("   ")
            .into_iter()
            .last()
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap();

        let stack_map = {
            let mut hashmap = HashMap::new();
            for index in 1..=stack_count {
                hashmap.insert(index, vec![]);
            }

            while let Some(line) = row_iter.next() {
                let mut chars = line.chars().skip(1);

                for index in 1..=stack_count {
                    match chars.next().unwrap() {
                        ' ' => unreachable!(),
                        value => hashmap.get_mut(&index).unwrap().push(Crate {
                            text: value.to_string(),
                        }),
                    }
                    chars = chars.skip(4);
                }
            }

            hashmap
        };

        PlayingField { stacks: stack_map }

        // 4n + 2 for n = i-1
        // .for_each(|slice| println!("{}", slice));
    }
}

fn main() {
    let input_file = "example.txt";
    let input_text = fs::read_to_string(input_file).unwrap();
    let pf: PlayingField = input_text.split("\n\n").next().unwrap().into();
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
