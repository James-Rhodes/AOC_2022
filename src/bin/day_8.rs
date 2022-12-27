use std::collections::HashSet;
fn main() {
    let input_text = std::fs::read_to_string("./inputs/input_day_8.txt").unwrap();

    let tree_grid: Vec<Vec<u32>> = input_text
        .lines()
        .map(|line| {
            line.chars()
                .map(|height| height.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    println!("Part One: {}", part_one(&tree_grid));
    println!("Part Two: {}", part_two(&tree_grid));
}

fn part_one(tree_grid: &Vec<Vec<u32>>) -> u32 {
    let mut hash_set = HashSet::<String>::new();

    horizontal_check(&tree_grid, &mut hash_set);
    vertical_check(&tree_grid, &mut hash_set);
    return hash_set.len() as u32;
}

fn horizontal_check(tree_grid: &Vec<Vec<u32>>, hs: &mut HashSet<String>) {
    for y in 0..tree_grid.len() {
        let mut current_max: i32 = -1;
        for x in 0..tree_grid[y].len() {
            if tree_grid[y][x] as i32 > current_max {
                current_max = tree_grid[y][x] as i32;
                hs.insert(String::from(x.to_string() + "," + &y.to_string()));
            }
        }
    }
    for y in 0..tree_grid.len() {
        let mut current_max: i32 = -1;
        for x in (0..tree_grid[y].len()).rev() {
            if tree_grid[y][x] as i32 > current_max {
                current_max = tree_grid[y][x] as i32;
                hs.insert(String::from(x.to_string() + "," + &y.to_string()));
            }
        }
    }
}

fn vertical_check(tree_grid: &Vec<Vec<u32>>, hs: &mut HashSet<String>) {
    for x in 0..tree_grid[0].len() {
        let mut current_max: i32 = -1;
        for y in 0..tree_grid.len() {
            if tree_grid[y][x] as i32 > current_max {
                current_max = tree_grid[y][x] as i32;
                hs.insert(String::from(x.to_string() + "," + &y.to_string()));
            }
        }
    }
    for x in 0..tree_grid[0].len() {
        let mut current_max: i32 = -1;
        for y in (0..tree_grid.len()).rev() {
            if tree_grid[y][x] as i32 > current_max {
                current_max = tree_grid[y][x] as i32;
                hs.insert(String::from(x.to_string() + "," + &y.to_string()));
            }
        }
    }
}

fn part_two(tree_grid: &Vec<Vec<u32>>) -> u32 {
    let mut largest_scenic_score: u32 = 0;
    for y in 0..tree_grid.len() {
        for x in 0..tree_grid[0].len() {
            let curr_scenic_score = get_scenic_score(&tree_grid, x, y);
            if curr_scenic_score > largest_scenic_score {
                largest_scenic_score = curr_scenic_score;
            }
        }
    }

    return largest_scenic_score;
}

fn get_scenic_score(tree_grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    if x == 0 || y == 0 || x == tree_grid[0].len() || y == tree_grid.len() {
        return 0;
    }
    let current_height = tree_grid[y][x];
    // Looking Up
    let mut view_dist_up = 0;
    for i in (0..y).rev() {
        if current_height > tree_grid[i][x] {
            view_dist_up += 1;
        } else {
            view_dist_up += 1;
            break;
        }
    }
    // Looking Down
    let mut view_dist_down = 0;
    for i in (y + 1)..tree_grid.len() {
        if current_height > tree_grid[i][x] {
            view_dist_down += 1;
        } else {
            view_dist_down += 1;
            break;
        }
    }
    // Looking Left
    let mut view_dist_left = 0;
    for i in (0..x).rev() {
        if current_height > tree_grid[y][i] {
            view_dist_left += 1;
        } else {
            view_dist_left += 1;
            break;
        }
    }
    // Looking Right
    let mut view_dist_right = 0;
    for i in (x + 1)..tree_grid[0].len() {
        if current_height > tree_grid[y][i] {
            view_dist_right += 1;
        } else {
            view_dist_right += 1;
            break;
        }
    }
    return view_dist_up * view_dist_down * view_dist_left * view_dist_right;
}
