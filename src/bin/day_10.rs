#[derive(Debug)]
struct Instruction {
    wait_time: u32,
    add_amount: i32,
}

fn main() {
    let instructions: Vec<Instruction> = std::fs::read_to_string("./inputs/input_day_10.txt")
        .unwrap()
        .lines()
        .map(|line| match line.split_once(" ") {
            None => Instruction {
                wait_time: 1,
                add_amount: 0,
            },
            Some(add_command) => Instruction {
                wait_time: 2,
                add_amount: add_command.1.parse::<i32>().unwrap(),
            },
        })
        .collect();

    let mut clock_cycle = 0;
    let mut register_x: i32 = 1;
    let mut total = 0;
    let mut part_two_render: String = String::from("");
    for instruction in instructions {
        for _ in 0..instruction.wait_time {
            clock_cycle += 1;

            // For part one
            if clock_cycle >= 20 && (clock_cycle - 20) % 40 == 0 && clock_cycle <= 220 {
                total += clock_cycle * register_x;
            }

            // Part two
            let render_x_pos = (clock_cycle - 1) % 40; // Render x position is 1 behind the clock cycle. first cycle draws in 0th position

            // If the current render position is within 1 of the register_x (accounts for width of 3)
            if i32::abs(register_x - render_x_pos) <= 1 {
                part_two_render += "#";
            } else {
                part_two_render += ".";
            }

            if clock_cycle % 40 == 0 {
                part_two_render += "\n";
            }
        }
        register_x += instruction.add_amount;
    }

    println!("Part One: {}", total);
    println!("Part Two: \n{}", part_two_render);
}
