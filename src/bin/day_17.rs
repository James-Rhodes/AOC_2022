use std::collections::{HashSet, VecDeque};
const ROCKS: &str = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##";

#[derive(Default)]
struct GameGrid {
    occupied_positions: HashSet<(i32, i32)>,
    current_highest: u64,
    current_moving_rock: Rock,
    largest_heights: [i32; 7],
}

impl GameGrid {
    fn new() -> Self {
        let mut game_grid: GameGrid = GameGrid::default();

        game_grid.largest_heights = [-1; 7];
        return game_grid;
    }
    fn move_current_rock(&mut self, movements: &Vec<char>, current_movement: &mut usize) {
        while !self.current_moving_rock.stopped {
            self.current_moving_rock
                .perform_movement(movements[*current_movement], &self.occupied_positions);

            *current_movement = (*current_movement + 1) % movements.len();
        }

        self.current_moving_rock
            .get_world_space_positions(self.current_moving_rock.pos)
            .iter()
            .for_each(|coord| {
                self.occupied_positions.insert(*coord);

                if coord.1 + 1 > self.current_highest as i32 {
                    self.current_highest = coord.1 as u64 + 1;
                }

                if self.largest_heights[coord.0 as usize] < coord.1 {
                    self.largest_heights[coord.0 as usize] = coord.1;
                }
            });

        let min_height = self
            .largest_heights
            .iter()
            // .filter(|num| **num >= 0)
            .min()
            .unwrap();

        self.largest_heights
            .iter()
            .enumerate()
            .for_each(|(i, val)| self.relative_heights[i] = val - *min_height);
    }

    fn print_grid(&self) {
        for current_layer in (0..self.current_highest).rev() {
            let mut grid_layer = ".......".to_string();
            for (x, y) in &self.occupied_positions {
                if *y == current_layer as i32 {
                    grid_layer = String::from(
                        grid_layer[..*x as usize].to_owned()
                            + "#"
                            + &grid_layer[(*x + 1) as usize..],
                    );
                }
            }

            println!("{current_layer:>4} - {}", grid_layer);
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Rock {
    rock_positions: Vec<(i32, i32)>, // Model space rock piece positions referenced to the bottom left
    pos: (i32, i32),
    width: u64,
    stopped: bool,
}

impl Rock {
    fn perform_movement(&mut self, action: char, game_grid: &HashSet<(i32, i32)>) {
        let movement: i32 = match action {
            '<' => -1,
            '>' => 1,
            _ => 0,
        };

        let new_x_pos = self.pos.0 as i32 + movement;
        if new_x_pos >= 0
            && new_x_pos + (self.width - 1) as i32 <= 6
            && !self.collides_with_grid((new_x_pos, self.pos.1), game_grid)
        {
            self.pos.0 = new_x_pos as i32;
        }

        let new_y_pos: i32 = self.pos.1 - 1;

        if new_y_pos != -1 && !self.collides_with_grid((self.pos.0, new_y_pos), game_grid) {
            self.pos.1 = new_y_pos;
        } else {
            self.stopped = true;
        }
    }

    fn get_world_space_positions(&self, coord: (i32, i32)) -> Vec<(i32, i32)> {
        return self
            .rock_positions
            .iter()
            .map(|(x, y)| (coord.0 + x, coord.1 + y))
            .collect::<Vec<(i32, i32)>>();
    }

    fn collides_with_grid(&self, coord: (i32, i32), game_grid: &HashSet<(i32, i32)>) -> bool {
        for ws_coord in self.get_world_space_positions(coord) {
            if game_grid.contains(&ws_coord) {
                return true;
            }
        }
        return false;
    }
}

fn main() {
    let input_text = std::fs::read_to_string("./inputs/input_day_17.txt").unwrap();

    println!("Part One: {}", part_one(&input_text, 2022));
}

fn part_one(input_text: &str, num_iterations: u64) -> u64 {
    let base_rocks = parse_rocks(ROCKS);
    let movements = input_text.chars().collect::<Vec<char>>();

    let mut game_grid = GameGrid::new();

    let mut movement_index = 0;
    let mut rock_index = 0;

    let (num_rocks_when_cycle_occurs, height_when_cycle_occurs) = find_cycle(
        &mut game_grid,
        &base_rocks,
        &mut rock_index,
        &movements,
        &mut movement_index,
    );

    println!(
        "{}, {}",
        num_rocks_when_cycle_occurs, height_when_cycle_occurs
    );

    let cycles_in_num_iterations = num_iterations / num_rocks_when_cycle_occurs;
    let remaining_rocks_after_cycles = num_iterations % num_rocks_when_cycle_occurs;

    let current_height: u64 = cycles_in_num_iterations * height_when_cycle_occurs;

    println!("cycles in num_iterations: {}, remaining_rocks: {}, current_height: {}, current_height_grid: {}", cycles_in_num_iterations,remaining_rocks_after_cycles,current_height, game_grid.current_highest);
    for _ in 0..remaining_rocks_after_cycles {
        let mut current_rock = base_rocks[rock_index].clone();
        current_rock.pos.1 = game_grid.current_highest as i32 + 3;

        game_grid.current_moving_rock = current_rock;
        game_grid.move_current_rock(&movements, &mut movement_index);

        rock_index = (rock_index + 1) % base_rocks.len();
    }

    game_grid.print_grid();

    // println!("Hello?");
    // println!(
    //     "relative_heights: {:?}\nmax_heights: {:?}",
    //     game_grid.relative_heights, game_grid.largest_heights
    // );

    println!(
        "Current_Height: {}, current_highest_grid: {}, height_when_cycle: {}, diff: {}",
        current_height,
        game_grid.current_highest,
        height_when_cycle_occurs,
        game_grid.current_highest - height_when_cycle_occurs
    );
    return current_height + (game_grid.current_highest - height_when_cycle_occurs);
}

fn part_two(input_text: &str) -> u64 {
    return part_one(input_text, 1000000000000);
}

fn parse_rocks(input: &str) -> Vec<Rock> {
    let mut base_rocks = input
        .split("\n\n")
        .map(|rock_text| {
            Rock {
                rock_positions: rock_text
                    .lines()
                    .rev()
                    .enumerate()
                    .map(|(i, rock_row)| {
                        rock_row
                            .chars()
                            .enumerate()
                            .map(|(j, block)| {
                                return match block {
                                    '.' => (99, 99), // Some large number I know cannot exist sop I can filter next
                                    '#' => (j as i32, i as i32),
                                    _ => (99, 99),
                                };
                            })
                            .filter(|(x, y)| *x != 99 && *y != 99)
                            .collect::<Vec<(i32, i32)>>()
                    })
                    .flatten()
                    .collect::<Vec<(i32, i32)>>(),
                pos: (2, 0),
                width: 0,
                stopped: false,
            }
        })
        .collect::<Vec<Rock>>();

    for rock in &mut base_rocks {
        let mut largest_width: u64 = 0;

        for (x, _) in rock.rock_positions.iter() {
            if (x + 1) as u64 > largest_width {
                largest_width = (x + 1) as u64;
            }
        }

        rock.width = largest_width;
    }
    return base_rocks;
}

fn find_cycle(
    game_grid: &mut GameGrid,
    base_rocks: &Vec<Rock>,
    rock_index: &mut usize,
    movements: &Vec<char>,
    movement_index: &mut usize,
) -> (u64, u64) {
    let mut state_cache: HashSet<(VecDeque<[i32; 7]>, usize, usize)> = HashSet::new();

    let mut num_rocks = 0;
    const NUM_ROWS_TO_INDICATE_CYCLE: u64 = 20;
    let mut prev_relative_heights: VecDeque<[i32; 7]> = VecDeque::new();
    let mut prev_max_height: VecDeque<u64> = VecDeque::new();
    loop {
        let mut current_rock = base_rocks[*rock_index].clone();
        current_rock.pos.1 = game_grid.current_highest as i32 + 3;

        game_grid.current_moving_rock = current_rock;
        game_grid.move_current_rock(&movements, movement_index);

        prev_max_height.push_front(game_grid.current_highest);
        if num_rocks < NUM_ROWS_TO_INDICATE_CYCLE {
            prev_relative_heights.push_front(game_grid.relative_heights.clone());
        } else {
            prev_relative_heights.push_front(game_grid.relative_heights.clone());
            prev_relative_heights.pop_back();
            let current_state = (prev_relative_heights.clone(), *movement_index, *rock_index);

            if !state_cache.insert(current_state.clone()) {
                println!("Cache match on: {:?}", current_state);
                println!("Cache contents: {:?}", state_cache);
                break;
            }
        }
        *rock_index = (*rock_index + 1) % base_rocks.len();
        num_rocks += 1;
    }

    println!("{:?}", prev_max_height);
    return (
        num_rocks - NUM_ROWS_TO_INDICATE_CYCLE - 1,
        prev_max_height[NUM_ROWS_TO_INDICATE_CYCLE as usize + 1],
    );
}

#[test]
fn part_one_test() {
    let input_text = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    assert_eq!(3068, part_one(input_text, 2022));
    // assert_eq!(3068, part_one(input_text, 3));
}
#[ignore]
#[test]
fn part_two_test() {
    let input_text = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    assert_eq!(1514285714288, part_two(input_text));
}
