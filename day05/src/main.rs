use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Crate {
    text: String,
}

#[derive(Debug)]
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
            .expect("Should have a row")
            .split("   ")
            .into_iter()
            .last()
            .expect("Should have a last item")
            .trim()
            .parse::<u32>()
            .expect("Should be parseable to int");

        let stack_map = {
            let mut hashmap = HashMap::new();
            for index in 0..stack_count {
                hashmap.insert(index + 1, vec![]);
            }

            while let Some(line) = row_iter.next() {
                let mut chars = line.chars();

                _ = chars.next();

                for index in 1..=stack_count + 1 {
                    match chars.next() {
                        Some(' ') => {}
                        Some(value) => hashmap.get_mut(&index).unwrap().push(Crate {
                            text: value.to_string(),
                        }),
                        None => {}
                    }
                    _ = chars.next();
                    _ = chars.next();
                    _ = chars.next();
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
    let input_text = fs::read_to_string(input_file).expect("File shoudl exists");
    let pf: PlayingField = input_text
        .split("\n\n")
        .next()
        .expect("Playing field should be readable")
        .into();
    dbg!(pf);
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
