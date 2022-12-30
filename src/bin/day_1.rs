use std::fs;
#[derive(Debug)]
pub struct Elf {
    pub snack_calories: Vec<i32>,
    pub total_calories: i32,
}
fn main() {
    println!("Hello world");
    let input_text = fs::read_to_string("./inputs/input_day_1.txt")
        .expect("The file could not be read or found!");

    let mut all_elves: Vec<Elf> = Vec::new();

    let mut current_elf: Elf = Elf {
        snack_calories: Vec::new(),
        total_calories: 0,
    };
    let mut largest_calories = 0;
    for line in input_text.split("\n") {
        let num: i32 = match line[0..(line.len() - 1)].parse() {
            Ok(n) => n,
            _ => -1,
        };

        if num != -1 {
            current_elf.snack_calories.push(num);
            current_elf.total_calories += num;
        } else {
            if largest_calories < current_elf.total_calories {
                largest_calories = current_elf.total_calories;
            }

            all_elves.push(current_elf);
            current_elf = Elf {
                snack_calories: Vec::new(),
                total_calories: 0,
            };
        }
    }

    println!("{:?}", all_elves);

    // PART ONE ANSWER
    println!("Largest number of calories = {}", largest_calories);

    all_elves.sort_by(|a, b| b.total_calories.cmp(&a.total_calories));
    let mut top_three_total_calories = 0;
    for i in 0..3 {
        top_three_total_calories += all_elves[i].total_calories;
    }

    // PART TWO ANSWER
    println!("{}", top_three_total_calories);
}
