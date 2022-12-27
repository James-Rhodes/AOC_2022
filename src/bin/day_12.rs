use std::collections::VecDeque;

#[derive(Debug)]
struct MapPosition {
    has_been_visited: bool,
    parent: Option<(usize, usize)>,
    neighbours: Vec<(usize, usize)>,
}
fn main() {
    let input_text = std::fs::read_to_string("./inputs/input_day_12.txt").unwrap();

    println!("Part One: {}", part_one(&input_text));
    println!("Part Two: {}", part_two(&input_text));
}

fn part_one(input: &str) -> u32 {
    let mut map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as u32).collect::<Vec<u32>>())
        .collect();
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 83 {
                start_pos = (j, i);
                map[i][j] = 'a' as u32;
            } else if map[i][j] == 69 {
                end_pos = (j, i);
                map[i][j] = 'z' as u32;
            }
        }
    }

    let mut map_info: Vec<Vec<MapPosition>> = vec![];
    for y in 0..map.len() {
        map_info.push(vec![]);
        for x in 0..map[y].len() {
            let has_been_visited = false;
            let parent: Option<(usize, usize)> = None;
            let mut neighbours: Vec<(usize, usize)> = vec![];

            // Left
            if x != 0 && map[y][x - 1] <= (map[y][x] + 1) {
                neighbours.push((x - 1, y));
            }
            // Right
            if x != map[y].len() - 1 && map[y][x + 1] <= (map[y][x] + 1) {
                neighbours.push((x + 1, y));
            }
            // Up
            if y != 0 && map[y - 1][x] <= (map[y][x] + 1) {
                neighbours.push((x, y - 1));
            }
            // Down
            if y != map.len() - 1 && map[y + 1][x] <= (map[y][x] + 1) {
                neighbours.push((x, y + 1));
            }

            map_info[y].push(MapPosition {
                has_been_visited,
                parent,
                neighbours,
            });
        }
    }

    return breadth_first_search(&mut map_info, &start_pos, &end_pos);
}

fn breadth_first_search(
    map: &mut Vec<Vec<MapPosition>>,
    start_pos: &(usize, usize),
    end_pos: &(usize, usize),
) -> u32 {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            map[y][x].has_been_visited = false;
            map[y][x].parent = None;
        }
    }
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    queue.push_back(*start_pos);

    while queue.len() > 0 {
        let (curr_x, curr_y) = queue.pop_front().unwrap();
        if !map[curr_y][curr_x].has_been_visited {
            map[curr_y][curr_x].has_been_visited = true;

            if curr_x == end_pos.0 && curr_y == end_pos.1 {
                // We found the End point
                break;
            }
            for i in 0..map[curr_y][curr_x].neighbours.len() {
                let (neighbour_x, neighbour_y) = map[curr_y][curr_x].neighbours[i];
                if !map[neighbour_y][neighbour_x].has_been_visited {
                    map[neighbour_y][neighbour_x].parent = Some((curr_x, curr_y));
                    queue.push_back((neighbour_x, neighbour_y));
                }
            }
        }
    }

    let mut num_steps = 0;

    let mut pos = *end_pos;
    loop {
        let next_pos = map[pos.1][pos.0].parent;
        if next_pos.is_none() {
            break;
        }
        num_steps += 1;
        pos = next_pos.unwrap();
    }
    return num_steps;
}

fn part_two(input: &str) -> u32 {
    let mut map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as u32).collect::<Vec<u32>>())
        .collect();
    let mut start_pos: Vec<(usize, usize)> = vec![];
    let mut end_pos = (0, 0);

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 83 || map[i][j] == ('a' as u32) {
                start_pos.push((j, i));
                map[i][j] = 'a' as u32;
            } else if map[i][j] == 69 {
                end_pos = (j, i);
                map[i][j] = 'z' as u32;
            }
        }
    }

    let mut map_info: Vec<Vec<MapPosition>> = vec![];
    for y in 0..map.len() {
        map_info.push(vec![]);
        for x in 0..map[y].len() {
            let has_been_visited = false;
            let parent: Option<(usize, usize)> = None;
            let mut neighbours: Vec<(usize, usize)> = vec![];

            // Left
            if x != 0 && map[y][x - 1] <= (map[y][x] + 1) {
                neighbours.push((x - 1, y));
            }
            // Right
            if x != map[y].len() - 1 && map[y][x + 1] <= (map[y][x] + 1) {
                neighbours.push((x + 1, y));
            }
            // Up
            if y != 0 && map[y - 1][x] <= (map[y][x] + 1) {
                neighbours.push((x, y - 1));
            }
            // Down
            if y != map.len() - 1 && map[y + 1][x] <= (map[y][x] + 1) {
                neighbours.push((x, y + 1));
            }

            map_info[y].push(MapPosition {
                has_been_visited,
                parent,
                neighbours,
            });
        }
    }
    let num_steps = start_pos
        .iter()
        .map(|sp| breadth_first_search(&mut map_info, &sp, &end_pos))
        .filter(|num| *num != 0)
        .min()
        .unwrap();

    return num_steps;
}
#[test]
fn part_one_test() {
    let input_text = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";

    let num_steps = part_one(input_text);
    assert_eq!(num_steps, 31);
}
#[test]
fn part_two_test() {
    let input_text = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";

    let num_steps = part_two(input_text);
    assert_eq!(num_steps, 29);
}
