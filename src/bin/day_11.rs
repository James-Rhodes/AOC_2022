use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum Operation {
    Add(Option<u64>),
    Multiply(Option<u64>),
}

#[derive(Debug, Clone)]
struct Test {
    num: u64,
    if_divisible_throw_to: u32,
    if_not_divisible_throw_to: u32,
}

#[derive(Debug, Clone)]
struct Monkey {
    id: u32, // Use this as a stack/queue depending on the prompt
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    num_inspections: u64,
}

impl Monkey {
    fn inspect(&mut self, part_two_remainder_theorem: Option<u64>) -> Option<(u32, u64)> {
        let worry_level = self.items.pop_front();

        if worry_level.is_none() {
            return None;
        }

        let mut worry_level: u64 = worry_level.unwrap();

        worry_level = match self.operation {
            Operation::Add(num) => {
                worry_level
                    + match num {
                        Some(val) => val,
                        None => worry_level,
                    }
            }
            Operation::Multiply(num) => {
                worry_level
                    * match num {
                        Some(val) => val,
                        None => worry_level,
                    }
            }
        };
        if part_two_remainder_theorem.is_none() {
            worry_level = worry_level / 3;
        } else {
            // This is the chinese remainder theorem. It states that if the self.test.num are all coprime (no common multiples
            // other than 1) then the remainder of the worry level with all of them multiplied together is going to be the same as the
            // the remainder of the worry level with just one of the self.test.num. This means that we can take the mod of the worry worry_level
            // with this new multiplied number and not affect the if else later on. So who the monkey throws to will not change due to this mod
            worry_level = worry_level % part_two_remainder_theorem.unwrap();
        }
        self.num_inspections += 1;
        if worry_level % self.test.num == 0 {
            return Some((self.test.if_divisible_throw_to, worry_level));
        } else {
            return Some((self.test.if_not_divisible_throw_to, worry_level));
        }
    }
}

fn main() {
    let mut monkeys: Vec<Monkey> = std::fs::read_to_string("./inputs/input_day_11.txt")
        .unwrap()
        .split("\r\n\r\n")
        .map(|monkey| parse_monkey(&monkey))
        .collect();
    let mut monkeys_2 = monkeys.clone();

    println!("Part One: {}", part_one(&mut monkeys, 20));
    println!("Part Two: {}", part_two(&mut monkeys_2, 10000));
}

fn parse_monkey(monkey_string: &str) -> Monkey {
    let mut output_monkey = Monkey {
        id: 0,
        items: VecDeque::new(),
        operation: Operation::Add(None),
        test: Test {
            num: 0,
            if_divisible_throw_to: 0,
            if_not_divisible_throw_to: 0,
        },
        num_inspections: 0,
    };

    monkey_string.lines().for_each(|line| {
        if line.starts_with("Monkey ") {
            output_monkey.id = line
                .split_once(" ")
                .unwrap()
                .1
                .replace(":", "")
                .parse::<u32>()
                .unwrap();
        } else if line.starts_with("  Starting items: ") {
            output_monkey.items = line
                .replace("  Starting items: ", "")
                .split(", ")
                .map(|num| num.parse::<u64>().unwrap())
                .collect();
        } else if line.starts_with("  Operation: new = old ") {
            let op = line.replace("  Operation: new = old ", "");

            let (operator, num) = op.split_once(" ").unwrap();
            let num = match num.parse::<u64>() {
                Ok(n) => Some(n),
                Err(_) => None,
            };

            if operator == "*" {
                output_monkey.operation = Operation::Multiply(num);
            } else if operator == "+" {
                output_monkey.operation = Operation::Add(num);
            } else {
                println!("AHHHHHH The operator isn't correct");
            }
        } else if line.starts_with("  Test: divisible by ") {
            let num = line
                .replace("  Test: divisible by ", "")
                .parse::<u64>()
                .unwrap();

            output_monkey.test.num = num;
        } else if line.starts_with("    If true: throw to monkey ") {
            output_monkey.test.if_divisible_throw_to = line
                .replace("    If true: throw to monkey ", "")
                .parse::<u32>()
                .unwrap();
        } else if line.starts_with("    If false: throw to monkey ") {
            output_monkey.test.if_not_divisible_throw_to = line
                .replace("    If false: throw to monkey ", "")
                .parse::<u32>()
                .unwrap();
        }
    });

    return output_monkey;
}

fn part_one(monkeys: &mut Vec<Monkey>, num_rounds: u32) -> u64 {
    for _ in 0..num_rounds {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let returned_option = monkeys[i].inspect(None);
                if returned_option.is_none() {
                    break;
                }

                let (id, worry_level) = returned_option.unwrap();
                monkeys[id as usize].items.push_back(worry_level);
            }
        }
    }
    monkeys.sort_by(|a, b| b.num_inspections.cmp(&a.num_inspections));
    println!("{:?}", monkeys);
    return monkeys[0].num_inspections * monkeys[1].num_inspections;
}

fn part_two(monkeys: &mut Vec<Monkey>, num_rounds: u32) -> u64 {
    let remainder_theorem = monkeys
        .iter()
        .map(|monkey| monkey.test.num)
        .product::<u64>();
    for _ in 0..num_rounds {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let returned_option = monkeys[i].inspect(Some(remainder_theorem));
                if returned_option.is_none() {
                    break;
                }

                let (id, worry_level) = returned_option.unwrap();
                monkeys[id as usize].items.push_back(worry_level);
            }
        }
    }
    monkeys.sort_by(|a, b| b.num_inspections.cmp(&a.num_inspections));
    println!("{:?}", monkeys);
    return monkeys[0].num_inspections * monkeys[1].num_inspections;
}
