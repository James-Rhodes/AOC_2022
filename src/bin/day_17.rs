use std::collections::{HashSet,HashMap, VecDeque};
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

#[derive(PartialEq,Eq,Clone, Copy, Hash, Debug)]
enum Block {
    Air,
    Rock
}

#[derive(PartialEq,Eq,Clone, Copy, Hash, Debug)]
struct Row {
    blocks: [Block; 7]
}

impl Row {
    fn new() -> Self{
        return Row {
            blocks: [Block::Air;7]
        }
    }
}

const NUM_CACHED_ROWS: usize = 20;
struct GameGrid {
    occupied_positions: HashSet<(i32, i32)>,
    current_highest: u64,
    current_moving_rock: Rock,
    prev_row_states: VecDeque<Row>
}

impl GameGrid {
    fn new() -> Self {

        let mut window:VecDeque<Row>= VecDeque::new();
        window.push_front(Row::new());

        return GameGrid{
            occupied_positions: HashSet::new(),
            current_highest: 0,
            current_moving_rock: Rock::default(),
            prev_row_states: window
        };

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

                self.update_prev_row_states(&coord);
                if coord.1 + 1 > self.current_highest as i32 {
                    self.current_highest = coord.1 as u64 + 1;
                }

            });
    }

    fn update_prev_row_states(&mut self, coord: &(i32,i32)) {

        let (x,y) = *coord;
        let mut relative_height = i32::max(self.current_highest as i32 - 1, 0) - y; // If largest = 20 and y is 17 this will give the index of 3 which is the row the row should be updated on

        if relative_height < 0 {
            // Push on to the vecdeque/stack

            while relative_height < 0 {
                self.prev_row_states.push_front(Row::new());

                if self.prev_row_states.len() > NUM_CACHED_ROWS {
                    // The stack has too many items so remove one
                    self.prev_row_states.pop_back();
                }

                relative_height += 1;
            }
            self.prev_row_states[0].blocks[x as usize] = Block::Rock;
        }
        else if relative_height < NUM_CACHED_ROWS as i32 {
            self.prev_row_states[relative_height as usize].blocks[x as usize] = Block::Rock;
        } 

    }

    #[allow(dead_code)]
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

            let display_layer = current_layer + 1;
            println!("{display_layer:>4} - {}", grid_layer);
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
    println!("Part Two: {}", part_two(&input_text));
}

fn part_one(input_text: &str, num_iterations: u64) -> u64 {

    let base_rocks = parse_rocks(ROCKS);
    let movements = input_text.chars().collect::<Vec<char>>();


    let (rocks_when_cycle_starts, rocks_when_cycle_ends) = find_cycle(
        &base_rocks,
        &movements,
    );

    let mut game_grid = GameGrid::new();

    let mut movement_index = 0;
    let mut rock_index = 0;

    
    let num_rocks_per_cycle = rocks_when_cycle_ends - rocks_when_cycle_starts;
    let mut height_before_cycle = 0;
    let mut height_after_cycle = 0;
    let mut cycle_height = 0;

    let cycles_that_fit_in_num_iterations = (num_iterations - rocks_when_cycle_starts) / num_rocks_per_cycle;
    let total_rocks_to_simulate = ((num_iterations - rocks_when_cycle_starts) % num_rocks_per_cycle) + rocks_when_cycle_ends;
    

    for rock_num in 1..= total_rocks_to_simulate {

        let mut current_rock = base_rocks[rock_index].clone();
        current_rock.pos.1 = game_grid.current_highest as i32 + 3;

        game_grid.current_moving_rock = current_rock;
        game_grid.move_current_rock(&movements, &mut movement_index);

        if rock_num == rocks_when_cycle_starts {
            height_before_cycle = game_grid.current_highest;
        }
        else if rock_num == rocks_when_cycle_ends {
            height_after_cycle = game_grid.current_highest;
            cycle_height = height_after_cycle - height_before_cycle;
        }

        rock_index = (rock_index + 1) % base_rocks.len();
    }


    
    // Height is going to be:
    // Height_when_cycle_starts + (height diff of cycle) * (num_cycles that fit in (num_iterations - rocks_when_cycle_starts)) + height_of_remaining_rocks

    let height_of_all_cycles =  cycle_height  * cycles_that_fit_in_num_iterations;
    let height_of_remaining_rocks = game_grid.current_highest - height_after_cycle;
    let total_height = height_before_cycle + height_of_all_cycles + height_of_remaining_rocks;

    return total_height;
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
                                    '.' => (99, 99), // Some large number I know cannot exist so I can filter next
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
    base_rocks: &Vec<Rock>,
    movements: &Vec<char>,
) -> (u64, u64) {

    let mut game_grid = GameGrid::new();
    let mut rock_index = 0;
    let mut movement_index = 0;
    let mut state_cache: HashMap<(VecDeque<Row>, usize, usize), u64> = HashMap::new();
    let mut prev_height = 0;
    let mut num_rocks = 0;
    loop {
        let mut current_rock = base_rocks[rock_index].clone();
        current_rock.pos.1 = game_grid.current_highest as i32 + 3;

        game_grid.current_moving_rock = current_rock;
        game_grid.move_current_rock(&movements, &mut movement_index);

        // Do the actual caching in here

        let current_state = (game_grid.prev_row_states.clone(), movement_index, rock_index);
        if state_cache.contains_key(&current_state) {
            let cycle_start = *state_cache.get(&current_state).unwrap();
            let cycle_end = num_rocks;

            return (
                cycle_start,
                cycle_end
            );
        } else if game_grid.current_highest > prev_height && num_rocks > NUM_CACHED_ROWS as u64{
            state_cache.insert(current_state, num_rocks);
        }
        
        rock_index = (rock_index + 1) % base_rocks.len();
        num_rocks += 1;
        prev_height = game_grid.current_highest;

    }
} 
#[test]
fn part_one_test() {
    let input_text = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    assert_eq!(3068, part_one(input_text, 2022));
    // assert_eq!(3068, part_one(input_text, 3));
}
#[test]
fn part_two_test() {
    let input_text = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    assert_eq!(1514285714288, part_two(input_text));
}