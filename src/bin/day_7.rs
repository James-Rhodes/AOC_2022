use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
struct Directory {
    sub_dirs: HashMap<String, Rc<RefCell<Directory>>>,
    files: Vec<u32>,
    parent_dir: Option<Rc<RefCell<Directory>>>,
    size: u32,
}

impl Directory {
    pub fn new() -> Directory {
        return Directory {
            sub_dirs: HashMap::new(),
            files: Vec::new(),
            parent_dir: None,
            size: 0,
        };
    }

    pub fn add_sub_dir(&mut self, dir_name: String, new_sub_dir: Rc<RefCell<Directory>>) {
        self.sub_dirs.insert(dir_name, new_sub_dir);
    }
    pub fn add_file(&mut self, new_file_size: u32) {
        self.files.push(new_file_size);
    }

    pub fn calc_dir_size(&mut self) -> u32 {
        self.size += self.files.iter().sum::<u32>();

        for (_, val) in &self.sub_dirs {
            self.size += val.borrow_mut().calc_dir_size();
        }
        return self.size;
    }

    pub fn add_if_less_than_100_000(&self, accumulator: &mut u32) {
        for (_, sub_dir) in &self.sub_dirs {
            sub_dir.borrow().add_if_less_than_100_000(accumulator);
        }
        if self.size < 100000 {
            *accumulator += self.size;
        }
    }

    pub fn find_smallest_size_above_threshold(&self, smallest: &mut u32, threshold: u32) {
        for (_, sub_dir) in &self.sub_dirs {
            sub_dir
                .borrow()
                .find_smallest_size_above_threshold(smallest, threshold);
        }
        if self.size < *smallest && self.size > threshold {
            *smallest = self.size;
        }
    }
}
fn main() {
    let input_text = std::fs::read_to_string("./../../inputs/input_day_7.txt").unwrap();

    // First load the file structure into a hella dodgey tree structure
    let root = Rc::new(RefCell::new(Directory::new()));
    let mut current = Rc::clone(&root);
    // Skip two because we know we are on root directory and performed ls
    for line in input_text.split("\r\n").skip(2) {
        if line.starts_with("dir ") {
            handle_dir(line, Rc::clone(&current));
        } else if line.starts_with("$ ") {
            current = match handle_command(line, &current) {
                Some(x) => x,
                None => current,
            };
        } else {
            handle_file(line, Rc::clone(&current));
        }
    }

    println!("Root Dir Size: {}", root.borrow_mut().calc_dir_size());

    println!("Part One: {}", part_one(&root));
    println!("Part Two: {}", part_two(&root));
}

fn handle_file(file_string: &str, current_node: Rc<RefCell<Directory>>) {
    current_node.borrow_mut().add_file(
        file_string
            .split_once(" ")
            .unwrap()
            .0
            .parse::<u32>()
            .unwrap(),
    );
}

fn handle_dir(dir_string: &str, current_node: Rc<RefCell<Directory>>) {
    let subdir = Rc::new(RefCell::new(Directory::new()));
    subdir.borrow_mut().parent_dir = Some(Rc::clone(&current_node));
    current_node.borrow_mut().add_sub_dir(
        String::from(dir_string.split_once(" ").unwrap().1),
        Rc::clone(&subdir),
    );
}

fn handle_command(
    command_str: &str,
    current_node: &Rc<RefCell<Directory>>,
) -> Option<Rc<RefCell<Directory>>> {
    let command: Vec<&str> = command_str.split(" ").collect();
    if command.len() == 2 {
        // ls command
        return None;
    }

    // cd command
    if command[2] == ".." {
        return match &current_node.as_ref().borrow().parent_dir {
            Some(x) => Some(Rc::clone(&x)),
            _ => unreachable!(),
        };
    } else {
        return match &current_node.as_ref().borrow().sub_dirs.get(command[2]) {
            Some(x) => Some(Rc::clone(&x)),
            _ => unreachable!(),
        };
    }
}

fn part_one(root_node: &Rc<RefCell<Directory>>) -> u32 {
    // Iterate through tree, if the current dir is less than 100_000 add it to the total
    let mut total: u32 = 0;

    root_node.borrow_mut().add_if_less_than_100_000(&mut total);

    return total;
}

fn part_two(root_node: &Rc<RefCell<Directory>>) -> u32 {
    // Iterate through tree, if the current size is less than the current smallest (begins at large num)
    // and is also large enough to free the required space then it is the new smallest. This is repeated
    // for every node until our answer is found

    let amount_of_space_to_free: u32 = 30000000 - (70000000 - root_node.borrow().size);

    let mut size_of_smallest = std::u32::MAX;

    root_node
        .borrow()
        .find_smallest_size_above_threshold(&mut size_of_smallest, amount_of_space_to_free);

    return size_of_smallest;
}
