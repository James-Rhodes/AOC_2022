fn main() {
    let input_text = std::fs::read_to_string("./inputs/input_day_5.txt").unwrap();

    let split_text: Vec<&str> = input_text.split("\r\n\r\n").collect();
    let container_config: Vec<&str> = split_text[0]
        .split("\r\n")
        .filter(|line| line[0..=1] != *" 1")
        .collect();

    let moves: Vec<&str> = split_text[1].split("\r\n").collect();
    println!("config: {:?}, moves: {:?}", container_config, moves);

    let mut container: Vec<Vec<char>> = Vec::new();
    let mut is_first_run: bool = true;
    for line in container_config.iter().rev() {
        let chars: Vec<char> = line.chars().collect();
        let mut container_index = 0;
        for i in (1..chars.len()).step_by(4) {
            if chars[i] != ' ' {
                if is_first_run {
                    container.push(vec![chars[i]]);
                } else {
                    container[container_index].push(chars[i]);
                }
            }

            container_index += !is_first_run as usize;
        }
        is_first_run = false;
    }
    println!("Before Movement:");
    for line in &container {
        println!("{:?}", line);
    }
    for line in &moves {
        let current_moves: Vec<u32> = line
            .split_whitespace()
            .filter_map(|word| word.parse::<u32>().ok())
            .collect();

        perform_movement(
            current_moves[0],
            current_moves[1],
            current_moves[2],
            &mut container,
        )
    }
    println!("After Movement:");
    for line in &container {
        println!("{:?}", line);
    }

    print!("Part One Solution: ");
    for line in &container {
        print!("{}", line.last().unwrap());
    }

    let mut container: Vec<Vec<char>> = Vec::new();
    let mut is_first_run: bool = true;
    for line in container_config.iter().rev() {
        let chars: Vec<char> = line.chars().collect();
        let mut container_index = 0;
        for i in (1..chars.len()).step_by(4) {
            if chars[i] != ' ' {
                if is_first_run {
                    container.push(vec![chars[i]]);
                } else {
                    container[container_index].push(chars[i]);
                }
            }

            container_index += !is_first_run as usize;
        }
        is_first_run = false;
    }
    println!("\nBefore Movement:");
    for line in &container {
        println!("{:?}", line);
    }
    for line in &moves {
        let current_moves: Vec<u32> = line
            .split_whitespace()
            .filter_map(|word| word.parse::<u32>().ok())
            .collect();

        perform_multiple_movement(
            current_moves[0],
            current_moves[1],
            current_moves[2],
            &mut container,
        )
    }
    println!("After Movement:");
    for line in &container {
        println!("{:?}", line);
    }

    print!("Part Two Solution: ");
    for line in &container {
        print!("{}", line.last().unwrap());
    }
}

fn perform_movement(
    num_to_move: u32,
    from_stack: u32,
    to_stack: u32,
    container: &mut Vec<Vec<char>>,
) {
    for _ in 0..num_to_move {
        let temp = container[(from_stack - 1) as usize].pop().unwrap();
        container[(to_stack - 1) as usize].push(temp);
    }
}

fn perform_multiple_movement(
    num_to_move: u32,
    from_stack: u32,
    to_stack: u32,
    container: &mut Vec<Vec<char>>,
) {
    let container_len = container[(from_stack - 1) as usize].len();
    let mut temp =
        container[(from_stack - 1) as usize].split_off(container_len - num_to_move as usize);
    container[(to_stack - 1) as usize].append(&mut temp);
}
