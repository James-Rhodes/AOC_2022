use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_till};
use nom::character::complete::{alpha1, digit1};
use nom::IResult;

#[derive(Clone,Debug)]
enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

// enum NumberResult{
//     Number(i64),
//     Monkey(&'static str)
// }

#[derive(Default, Debug)]
struct MonkeyCalculation<'a> {
    left: Option<&'a str>,
    operation: Option<Operation>,
    right: Option<&'a str>,
    value: Option<i64>,
}

fn parse_monkey_calculation(input: &'_ str) -> IResult<&'_ str, (&'_ str, MonkeyCalculation)> {
    let (input, monkey_name) = take_till(|c| c == ':')(input)?;

    let (input, _) = tag(": ")(input)?;

    let (input, monkey_or_num) = alt((alpha1, digit1))(input)?;

    if let Ok(number) = monkey_or_num.parse::<i64>() {
        let  monkey_calc = MonkeyCalculation{
            value: Some(number),
            ..Default::default()
        };
        return Ok((input, (monkey_name, monkey_calc)));
    }

    let left_side_monkey = monkey_or_num; // It is definitely a monkey

    let (input, _) = tag(" ")(input)?;

    let (input, operator) = take(1_usize)(input)?;

    let operation = match operator {
        "+" => Operation::Addition,
        "-" => Operation::Subtraction,
        "/" => Operation::Division,
        "*" => Operation::Multiplication,
        _ => panic!(),
    };

    let (input, _) = tag(" ")(input)?;

    let (input, right_side_monkey) = take(4_usize)(input)?;

    let monkey_calc = MonkeyCalculation{
        left: Some(left_side_monkey),
        right: Some(right_side_monkey),
        operation: Some(operation),
        ..Default::default()
    };

    return Ok((input, (monkey_name, monkey_calc)));
}

fn perform_monkey_calculation(
    current_monkey: &str,
    monkey_map: &HashMap<&str, MonkeyCalculation>,
) -> i64 {

    let monkey = monkey_map.get(current_monkey).unwrap();
    if monkey.value.is_some() {
        return monkey.value.unwrap();
    }

    let left_num = perform_monkey_calculation(monkey.left.unwrap(), monkey_map);
    let right_num = perform_monkey_calculation(monkey.right.unwrap(), monkey_map);

    let result = match monkey.operation {
        Some(Operation::Addition) => left_num + right_num,
        Some(Operation::Subtraction) => left_num - right_num,
        Some(Operation::Multiplication) => left_num * right_num,
        Some(Operation::Division) => left_num / right_num,
        None => panic!()
    };

    return result;
}

fn contains_human(current_monkey:&str, monkey_map: &HashMap<&str, MonkeyCalculation>) -> bool {

    if current_monkey == "humn" {
        return true;
    }

    let monkey = monkey_map.get(current_monkey).unwrap();
    if monkey.value.is_some() {
        return false;
    }

    if monkey.left.unwrap() == "humn" || monkey.right.unwrap() == "humn" {
        return true;
    }

    let left = contains_human(monkey.left.unwrap(), monkey_map);
    let right = contains_human(monkey.right.unwrap(), monkey_map);

    return left || right;
}

fn find_num_to_yell(current_monkey: &str, desired_num: i64, monkey_map: &HashMap<&str, MonkeyCalculation>) -> i64 {
    if current_monkey == "humn" {
        return desired_num;
    }

    let monkey = monkey_map.get(current_monkey).unwrap();
    let operation = monkey.operation.clone().unwrap();
    let left_monkey = monkey.left.unwrap();
    let right_monkey = monkey.right.unwrap();

    let  left_contains_human = contains_human(left_monkey, monkey_map);
    let  right_contains_human = contains_human(right_monkey, monkey_map);

    let mut left_val:Option<i64> = None;
    let mut right_val:Option<i64> = None;
    if !left_contains_human {
        left_val = Some(perform_monkey_calculation(monkey.left.unwrap(), monkey_map));
    }else if !right_contains_human {
        right_val = Some(perform_monkey_calculation(monkey.right.unwrap(), monkey_map));
    }

    // Undo the operation to figure out the value needed to receive the desired_num
    return match (left_val, operation, right_val) {
        (Some(left_val), Operation::Addition, None) => find_num_to_yell(right_monkey, desired_num - left_val, monkey_map),
        (None, Operation::Addition, Some(right_val)) => find_num_to_yell(left_monkey, desired_num - right_val, monkey_map),        
        (Some(left_val), Operation::Subtraction, None) => find_num_to_yell(right_monkey, left_val - desired_num, monkey_map),
        (None, Operation::Subtraction, Some(right_val)) => find_num_to_yell(left_monkey, desired_num + right_val, monkey_map),
        (Some(left_val), Operation::Multiplication, None) => find_num_to_yell(right_monkey, desired_num / left_val, monkey_map),        
        (None, Operation::Multiplication, Some(right_val)) =>find_num_to_yell(left_monkey, desired_num / right_val, monkey_map),       
        (Some(left_val), Operation::Division, None) => find_num_to_yell(right_monkey, left_val / desired_num, monkey_map),
        (None, Operation::Division, Some(right_val)) => find_num_to_yell(left_monkey, desired_num * right_val, monkey_map),
        _ => panic!("Either both left and right value were present or the operation was incorrect")
    }; 
}

fn main() {
    let input_text = std::fs::read_to_string("./inputs/input_day_21.txt").unwrap();

    println!("Part One: {}", part_one(&input_text));
    println!("Part Two: {}", part_two(&input_text));
}

fn part_one(input_text: &str) -> i64 {

    let mut monkey_calculations: HashMap<&str, MonkeyCalculation> = HashMap::new();
    input_text.lines().for_each(|line| {
        let (_, (key, val)) = parse_monkey_calculation(line).unwrap();
        monkey_calculations.insert(key, val);
    });

    return perform_monkey_calculation("root", &monkey_calculations);
}

fn part_two(input_text: &str) -> i64{

    let mut monkey_calculations: HashMap<&str, MonkeyCalculation> = HashMap::new();
    input_text.lines().for_each(|line| {
        let (_, (key, val)) = parse_monkey_calculation(line).unwrap();
        monkey_calculations.insert(key, val);
    });

    let mut desired_result;
    let mut num_to_yell = 0;
    let root_monkey = monkey_calculations.get("root").unwrap();

    if !contains_human(root_monkey.left.unwrap(), &monkey_calculations){
        desired_result = perform_monkey_calculation(root_monkey.left.unwrap(), &monkey_calculations);
        num_to_yell = find_num_to_yell(root_monkey.right.unwrap(), desired_result, &monkey_calculations);
    }

    if !contains_human(root_monkey.right.unwrap(), &monkey_calculations){
        desired_result = perform_monkey_calculation(root_monkey.right.unwrap(), &monkey_calculations);
        num_to_yell = find_num_to_yell(root_monkey.left.unwrap(), desired_result, &monkey_calculations);
    }

    return num_to_yell;
}

#[test]
fn test_part_one() {
    let input_text = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    assert_eq!(part_one(input_text), 152);
}

#[test]
fn test_part_two() {
    let input_text = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    assert_eq!(part_two(input_text), 301);
}
