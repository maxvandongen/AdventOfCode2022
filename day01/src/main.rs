use std::fs;

fn main() {
    let filename = "input.txt";
    let input_text = fs::read_to_string(filename).expect("file should exists");
    let chuncks: Vec<&str> = input_text.split("\n\n").collect();
    let mut sums: Vec<u32> = chuncks
        .into_iter()
        .map(|chunck| {
            chunck
                .split("\n")
                .into_iter()
                .map(|val| val.parse::<u32>().unwrap())
                .sum()
        })
        .collect();
    sums.sort();

    println!("Part 1: {}", sums.last().unwrap());
    println!(
        "Part 2: {}",
        sums.iter().rev().take(3).copied().sum::<u32>()
    );
}
