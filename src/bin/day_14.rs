use std::collections::HashMap;
#[derive(Debug, Clone)]
enum Block {
    Rock,
    Sand,
}
fn main() {
    let input_text = std::fs::read_to_string("./inputs/input_day_14.txt").unwrap();
    println!("Part One: {}", part_one(&input_text));
    println!("Part Two: {}", part_two(&input_text));
}

fn part_one(input_text: &str) -> i32 {
    let rock_lines = get_rock_lines(input_text);
    let mut map = build_map(&rock_lines);

    let sand_emitter_pos: (i32, i32) = (500, 0);

    let mut sand_units: i32 = 0;

    while move_sand(&sand_emitter_pos, &mut map) {
        sand_units += 1;
    }
    return sand_units;
}

fn part_two(input_text: &str) -> i32 {
    let rock_lines = get_rock_lines(input_text);
    let mut map = build_map(&rock_lines);

    let sand_emitter_pos: (i32, i32) = (500, 0);

    let mut sand_units: i32 = 1;

    let cave_bottom_height = map
        .clone()
        .into_iter()
        .map(|(key, _val)| key.1)
        .max()
        .unwrap()
        + 2;
    while move_sand_part_two(&sand_emitter_pos, &mut map, cave_bottom_height) {
        sand_units += 1;
    }
    return sand_units;
}

fn get_rock_lines(input_text: &str) -> Vec<Vec<(i32, i32)>> {
    let rock_lines: Vec<Vec<(i32, i32)>> = input_text
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coords| {
                    let (x, y) = coords.split_once(",").unwrap();
                    let x = x.parse::<i32>().unwrap();
                    let y = y.parse::<i32>().unwrap();

                    return (x, y);
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .collect();

    return rock_lines;
}

fn build_map(rock_lines: &Vec<Vec<(i32, i32)>>) -> HashMap<(i32, i32), Block> {
    let mut map: HashMap<(i32, i32), Block> = HashMap::new();

    for rock_line in rock_lines.iter() {
        for i in 0..(rock_line.len() - 1) {
            let (mut x1, mut y1) = rock_line[i];
            let (x2, y2) = rock_line[i + 1];

            let delta_x = i32::signum(x2 as i32 - x1 as i32);
            let delta_y = i32::signum(y2 as i32 - y1 as i32);

            while (x1 != x2) ^ (y1 != y2) {
                map.insert((x1, y1), Block::Rock);

                x1 += delta_x;
                y1 += delta_y;
            }
        }
        map.insert(rock_line[rock_line.len() - 1], Block::Rock); // Last in the array still needs to be added
    }
    return map;
}

fn move_sand(emitter_pos: &(i32, i32), map: &mut HashMap<(i32, i32), Block>) -> bool {
    let mut current_pos = emitter_pos.clone();
    let mut num_iterations = 0;
    while num_iterations < 1000 {
        let next_pos = (current_pos.0, current_pos.1 + 1);

        if map.contains_key(&next_pos) {
            if !map.contains_key(&(next_pos.0 - 1, next_pos.1)) {
                return move_sand(&(next_pos.0 - 1, next_pos.1), map);
            } else if !map.contains_key(&(next_pos.0 + 1, next_pos.1)) {
                return move_sand(&(next_pos.0 + 1, next_pos.1), map);
            } else if !map.contains_key(&current_pos) {
                map.insert(current_pos, Block::Sand); // Has settled
                return true;
            }
        }
        current_pos = next_pos;
        num_iterations += 1;
    }
    return false;
}
fn move_sand_part_two(
    emitter_pos: &(i32, i32),
    map: &mut HashMap<(i32, i32), Block>,
    cave_bottom_height: i32,
) -> bool {
    let mut current_pos = emitter_pos.clone();
    loop {
        let next_pos = (current_pos.0, current_pos.1 + 1);

        if map.contains_key(&next_pos) {
            if !map.contains_key(&(next_pos.0 - 1, next_pos.1)) {
                return move_sand_part_two(&(next_pos.0 - 1, next_pos.1), map, cave_bottom_height);
            } else if !map.contains_key(&(next_pos.0 + 1, next_pos.1)) {
                return move_sand_part_two(&(next_pos.0 + 1, next_pos.1), map, cave_bottom_height);
            } else if !map.contains_key(&current_pos) {
                map.insert(current_pos, Block::Sand); // Has settled

                if current_pos.0 == 500 && current_pos.1 == 0 {
                    return false;
                }
                return true;
            }
        }

        if next_pos.1 == cave_bottom_height {
            map.insert(current_pos, Block::Sand);
            return true;
        }
        current_pos = next_pos;
    }
}
#[test]
fn part_one_test() {
    let input_text = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    assert_eq!(24, part_one(input_text));
}

#[test]
fn part_two_test() {
    let input_text = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    assert_eq!(93, part_two(input_text));
}
