fn main() {
    // let filename = "example.txt";
    let file = include_str!("../input.txt");
    // Part 1
    // A Rock => 1
    // B Paper => 2
    // C Scissors => 3
    // X Rock
    // Y Paper
    // Z Scissors
    // Win + 6
    // Draw + 3
    // Lose + 0
    let part1_score: u32 = file
        .lines()
        .map(|line| {
            let plays = line.split(" ").collect::<Vec<&str>>();
            let opponent = plays.get(0).expect("Opponent plays move");
            let player = plays.get(1).expect("Player makes move");
            let score_from_play = match *player {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => 0,
            };
            let score_from_opponent = match (*opponent, *player) {
                ("A", "X") => 3,
                ("A", "Y") => 6,
                ("A", "Z") => 0,
                ("B", "X") => 0,
                ("B", "Y") => 3,
                ("B", "Z") => 6,
                ("C", "X") => 6,
                ("C", "Y") => 0,
                ("C", "Z") => 3,
                _ => 0,
            };
            score_from_play + score_from_opponent
        })
        .sum();
    println!("Part 1: {}", part1_score);

    // Part 2
    // X Lose
    // Y Draw
    // Z Win
    // Win + 6
    // Draw + 3
    // Lose + 0
    let part2_score: u32 = file
        .lines()
        .map(|line| {
            let plays = line.split(" ").collect::<Vec<&str>>();
            let opponent = plays.get(0).expect("Opponent plays move");
            let outcome = plays.get(1).expect("Player has outcome");
            let my_move = match (*opponent, *outcome) {
                ("A", "X") => "Scissors",
                ("A", "Y") => "Rock",
                ("A", "Z") => "Paper",
                ("B", "X") => "Rock",
                ("B", "Y") => "Paper",
                ("B", "Z") => "Scissors",
                ("C", "X") => "Paper",
                ("C", "Y") => "Scissors",
                ("C", "Z") => "Rock",
                _ => "",
            };
            let move_score = match my_move {
                "Rock" => 1,
                "Paper" => 2,
                "Scissors" => 3,
                _ => 0,
            };
            let outcome_score = match *outcome {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => 0,
            };
            move_score + outcome_score
        })
        .sum();
    println!("Part 2: {}", part2_score);
}
