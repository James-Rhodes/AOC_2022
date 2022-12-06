use std::collections::HashSet;
fn main() {
    let input_text = std::fs::read_to_string("./../../inputs/input_day_6.txt").unwrap();

    let solution_1 = find_first_n_length_unique_window(4, &input_text);
    println!("Part One Solution: {}", solution_1);

    let solution_2 = find_first_n_length_unique_window(14, &input_text);
    println!("Part Two Solution: {}", solution_2);
}

fn find_first_n_length_unique_window(n: usize, text: &String) -> u32 {
    let binding = text.chars().collect::<Vec<char>>();

    let position_of_n_unique: usize = binding[..]
        .windows(n)
        .map(|wind| wind.into_iter().collect::<HashSet<&char>>())
        .position(|hs| hs.len() == n)
        .unwrap();

    return (position_of_n_unique + n) as u32;
}
