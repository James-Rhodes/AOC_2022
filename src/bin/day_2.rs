fn main() {
    let input_text = std::fs::read_to_string("./inputs/input_day_2.txt")
        .expect("The file could not be found or read");

    let game_moves = input_text.split("\r\n");
    let game_moves_for_part_2 = game_moves.clone();

    let result: i32 = game_moves
        .map(|single_game_moves| {
            let (opponent_move, my_move) = single_game_moves.split_once(" ").unwrap();
            return calculate_my_score(my_move, opponent_move);
        })
        .sum::<i32>();

    // Part One Solution
    println!("Part One: {:?}", result);

    let result: i32 = game_moves_for_part_2
        .map(|single_game_moves| {
            let (opponent_move, outcome) = single_game_moves.split_once(" ").unwrap();
            let my_move = get_move_for_outcome(opponent_move, outcome);
            return calculate_my_score(my_move, opponent_move);
        })
        .sum::<i32>();
    // Part Two Solution
    println!("Part Two: {:?}", result);
}

fn calculate_my_score(my_move: &str, opponent_move: &str) -> i32 {
    let my_move_int = map_string_move_to_int(my_move);
    let opponent_move_int = map_string_move_to_int(opponent_move);
    let outcome = match (my_move_int % 3) - (opponent_move_int % 3) {
        -1 | 2 => 0, // Loss
        1 | -2 => 6, // Win
        0 => 3,      // Draw
        _ => 0,      // Exhaustive case
    };

    return my_move_int + outcome;
}

fn map_string_move_to_int(player_move: &str) -> i32 {
    return match player_move {
        "A" | "X" => 1, // Rock
        "B" | "Y" => 2, // Paper
        "C" | "Z" => 3, // Scissors
        _ => -1000,     // Exhaustive Case
    };
}

fn get_move_for_outcome<'a>(opponent_move: &'a str, outcome: &'a str) -> &'a str {
    if outcome == "X" {
        // Lose
        return match opponent_move {
            "A" => "C",
            "B" => "A",
            "C" => "B",
            _ => "",
        };
    } else if outcome == "Y" {
        // Draw
        return opponent_move;
    } else {
        // Win
        return match opponent_move {
            "A" => "B",
            "B" => "C",
            "C" => "A",
            _ => "",
        };
    }
}
