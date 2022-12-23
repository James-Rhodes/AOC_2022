use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
enum DataTypes {
    Integer(u32),
    List(Vec<DataTypes>),
}

fn main() {
    let text_input = std::fs::read_to_string("./../../inputs/input_day_13.txt").unwrap();

    println!("Part One: {}", part_one(&text_input));

    let mut text_input = std::fs::read_to_string("./../../inputs/input_day_13.txt").unwrap();

    text_input.push_str("\r\n\r\n[[2]]\r\n[[6]]"); // Adding the decoding strings
    println!("Part Two: {}", part_two(&text_input));
}

fn part_one(input_file: &str) -> u32 {
    let pairs = input_file
        .split("\r\n\r\n")
        .map(|pair| pair.lines().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut sum = 0;
    for (index, pair) in pairs.iter().enumerate() {
        let left = get_list_data_representation(pair[0]).0;
        let right = get_list_data_representation(pair[1]).0;

        let result = in_right_order(&left, &right);
        let result = match result {
            Some(val) => val,
            None => true,
        };
        if result {
            sum += index as u32 + 1;
        }
    }
    return sum;
}

fn part_two(input_file: &str) -> u32 {
    let mut packets = input_file
        .split("\r\n\r\n")
        .map(|pair| pair.lines().collect::<Vec<&str>>())
        .flatten()
        .map(|line| get_list_data_representation(line).0)
        .collect::<Vec<DataTypes>>();

    packets.sort_by(|a, b| match in_right_order(a, b) {
        Some(result) => {
            if result {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        }
        None => {
            return Ordering::Equal;
        }
    });

    let mut result = 1;
    let decoder_packet_2 = get_list_data_representation("[[2]]").0;
    let decoder_packet_6 = get_list_data_representation("[[6]]").0;
    for (index, packet) in packets.iter().enumerate() {
        if *packet == decoder_packet_2 || *packet == decoder_packet_6 {
            result *= index + 1;
        }
    }
    return result as u32;
}

fn in_right_order(left: &DataTypes, right: &DataTypes) -> Option<bool> {
    let mut curr_bool = None;
    match (&left, &right) {
        (DataTypes::List(l_str), DataTypes::List(r_str)) => {
            let mut i = 0;
            while i < l_str.len() && i < r_str.len() && curr_bool.is_none() {
                curr_bool = match (&l_str[i], &r_str[i]) {
                    (DataTypes::Integer(l), DataTypes::Integer(r)) => {
                        if l < r {
                            return Some(true);
                        } else if l > r {
                            return Some(false);
                        }
                        None
                    }
                    (DataTypes::List(l), DataTypes::List(r)) => {
                        in_right_order(&DataTypes::List(l.to_vec()), &DataTypes::List(r.to_vec()))
                    }
                    (DataTypes::List(l), DataTypes::Integer(r)) => in_right_order(
                        &DataTypes::List(l.clone()),
                        &DataTypes::List(vec![DataTypes::Integer(*r)]),
                    ),
                    (DataTypes::Integer(l), DataTypes::List(r)) => in_right_order(
                        &DataTypes::List(vec![DataTypes::Integer(*l)]),
                        &DataTypes::List(r.clone()),
                    ),
                };

                i += 1;
            }
            if i == r_str.len() && i < l_str.len() && curr_bool.is_none() {
                return Some(false);
            } else if i == l_str.len() && i < r_str.len() && curr_bool.is_none() {
                return Some(true);
            }
        }
        (_, _) => unreachable!(),
    }

    return curr_bool;
}

fn get_list_data_representation(line: &str) -> (DataTypes, usize) {
    let chars_list = line.chars().collect::<Vec<char>>();
    let mut current_list: Vec<DataTypes> = vec![];
    let mut i = 1;
    while i < chars_list.len() {
        if chars_list[i] == '[' {
            let temp = get_list_data_representation(&line[i..]);
            i += temp.1;

            let temp = match temp.0 {
                DataTypes::List(sub_list) => Some(sub_list),
                _ => None,
            };
            current_list.push(DataTypes::List(temp.unwrap()))
        } else if chars_list[i] == ']' {
            return (DataTypes::List(current_list), i);
        } else if chars_list[i].is_numeric() {
            let mut current_num = chars_list[i].to_string();
            if i + 1 < chars_list.len() && chars_list[i + 1].is_numeric() {
                current_num.push(chars_list[i + 1]);
                i += 1;
            }
            let current_num = current_num.parse::<u32>().unwrap();
            current_list.push(DataTypes::Integer(current_num));
        }
        i += 1;
    }

    return (DataTypes::List(current_list), 0);
}

#[test]
fn part_one_test() {
    let input_text = std::fs::read_to_string("./inputs/tests/test_day_13.txt").unwrap();

    assert_eq!(part_one(&input_text), 13);
}

#[test]
fn part_two_test() {
    let mut input_text = std::fs::read_to_string("./inputs/tests/test_day_13.txt").unwrap();
    input_text.push_str("\r\n\r\n[[2]]\r\n[[6]]");
    assert_eq!(part_two(&input_text), 140);
}
