use std::collections::HashSet;

fn find_marker_index(message: &str, window_size: usize) -> Option<usize> {
    let chars = message.chars().collect::<Vec<char>>();
    if let Some(index) = chars.windows(window_size).enumerate().find(|(_, window)| {
        let set: HashSet<char> = HashSet::from_iter(window.iter().cloned());
        if set.len() == window_size {
            return true;
        }
        false
    }) {
        return Some(index.0 + window_size);
    }
    None
}

fn main() {
    let file = include_str!("../input.txt");

    println!(
        "Part 1: {}",
        find_marker_index(file, 4).expect("No marker found")
    );
    println!(
        "Part 2: {}",
        find_marker_index(file, 14).expect("No marker found")
    );
}
