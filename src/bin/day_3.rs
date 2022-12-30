fn main() {
    println!("Hello world");

    let input_text = std::fs::read_to_string("./inputs/input_day_3.txt")
        .expect("The file could not be read or found!");
    let mut sum: u32 = 0;
    for line in input_text.split("\n") {
        let (first_sack, second_sack) = line.split_at(line.len() / 2);
        sum += calculate_priority(get_shared_char(first_sack, second_sack));
    }
    println!("First Part: {}", sum);

    let lines: Vec<&str> = input_text.split("\r\n").collect();

    let mut sum: u32 = 0;
    for i in (0..lines.len()).step_by(3) {
        sum += calculate_priority(get_shared_char_three(lines[i], lines[i + 1], lines[i + 2]));
    }

    println!("Second Part: {}", sum);
}

fn calculate_priority(item: char) -> u32 {
    let ascii_val: u32 = item as u32;
    if ascii_val >= 97 {
        return ascii_val - 96;
    } else {
        return ascii_val - (64 - 26);
    }
}

fn get_shared_char(first: &str, second: &str) -> char {
    for char1 in first.chars() {
        for char2 in second.chars() {
            if char1 == char2 {
                return char1 as char;
            }
        }
    }
    return '*';
}

fn get_shared_char_three(first: &str, second: &str, third: &str) -> char {
    for char1 in first.chars() {
        for char2 in second.chars() {
            for char3 in third.chars() {
                if char1 == char2 && char2 == char3 && char1 == char3 {
                    return char1 as char;
                }
            }
        }
    }
    return '*';
}
