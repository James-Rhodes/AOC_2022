use std::collections::HashSet;

#[derive(Debug)]
struct Movement {
    pub direction: String,
    pub num_steps: u32,
}

impl Movement {
    pub fn new(dir: String, n_steps: u32) -> Movement {
        return Movement {
            direction: dir,
            num_steps: n_steps,
        };
    }
}

#[derive(Clone, Copy, Debug)]
struct Pos {
    pub x: i32,
    pub y: i32,
}
impl std::ops::Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, rhs: Pos) -> Pos {
        return Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl Pos {
    fn distance(&self, other: &Pos) -> f64 {
        return f64::sqrt(
            f64::powi((self.x - other.x) as f64, 2) + f64::powi((self.y - other.y) as f64, 2),
        );
    }
}
struct Head {
    pos: Pos,
}
impl Head {
    pub fn new(start_x: i32, start_y: i32) -> Head {
        return Head {
            pos: Pos {
                x: start_x,
                y: start_y,
            },
        };
    }

    pub fn perform_movement(&mut self, tail: &mut Tail, movement: &Movement) {
        for _ in 0..movement.num_steps {
            self.pos = self.pos
                + match &movement.direction[..] {
                    "R" => Pos { x: 1, y: 0 },
                    "L" => Pos { x: -1, y: 0 },
                    "U" => Pos { x: 0, y: 1 },
                    "D" => Pos { x: 0, y: -1 },
                    _ => Pos { x: 0, y: 0 },
                };
            tail.perform_movement(&self.pos);
        }
    }
    pub fn perform_movement_multiple(&mut self, tails: &mut Vec<Tail>, movement: &Movement) {
        for _ in 0..movement.num_steps {
            self.pos = self.pos
                + match &movement.direction[..] {
                    "R" => Pos { x: 1, y: 0 },
                    "L" => Pos { x: -1, y: 0 },
                    "U" => Pos { x: 0, y: 1 },
                    "D" => Pos { x: 0, y: -1 },
                    _ => Pos { x: 0, y: 0 },
                };
            let mut curr_head_pos = self.pos;
            for tail in &mut *tails {
                tail.perform_movement(&curr_head_pos);

                curr_head_pos = tail.pos;
            }
        }
    }
}

#[derive(Clone)]
struct Tail {
    pos: Pos,
    hs: HashSet<String>,
}

impl Tail {
    pub fn new(start_x: i32, start_y: i32) -> Tail {
        let mut temp = Tail {
            pos: Pos {
                x: start_x,
                y: start_y,
            },
            hs: HashSet::new(),
        };

        temp.hs.insert(String::from(
            start_x.to_string() + "," + &start_y.to_string(),
        ));

        return temp;
    }

    pub fn perform_movement(&mut self, curr_head_pos: &Pos) {
        // Do stuff
        if self.pos.distance(curr_head_pos) > 1.5 {
            // Is the distance larger than we allow?
            let delta = Pos {
                x: i32::signum(curr_head_pos.x - self.pos.x),
                y: i32::signum(curr_head_pos.y - self.pos.y),
            };

            self.pos = self.pos + delta;

            self.hs.insert(String::from(
                self.pos.x.to_string() + "," + &self.pos.y.to_string(),
            ));
        }
    }
}

fn main() {
    let movements: Vec<Movement> = std::fs::read_to_string("./inputs/input_day_9.txt")
        .expect("Could not find file...")
        .lines()
        .map(|line| {
            let (dir, n_steps) = line.split_once(" ").unwrap();
            return Movement::new(String::from(dir), n_steps.parse::<u32>().unwrap());
        })
        .collect();

    println!("Part One: {}", part_one(&movements));
    println!("Part Two: {}", part_two(&movements));
}

fn part_one(movements: &Vec<Movement>) -> u32 {
    let mut head = Head::new(0, 0);
    let mut tail = Tail::new(0, 0);

    for m in movements {
        head.perform_movement(&mut tail, m);
    }

    return tail.hs.len() as u32;
}

fn part_two(movements: &Vec<Movement>) -> u32 {
    let mut head = Head::new(0, 0);
    let mut tails: Vec<Tail> = vec![Tail::new(0, 0); 9];

    for m in movements {
        head.perform_movement_multiple(&mut tails, m);
    }

    return tails[tails.len() - 1].hs.len() as u32;
}
// If it is the first one that has a distance greater than 1.5 then move to head pos
// Otherwise use the same motion as the first one for all other tails if the distance gets greater than 1.5

// ..........................
// ..........................
// ..........................
// ..........................
// ..........................
// ..........................
// ..........................
// ..........H12345..........    Just because the one before it moved diagonally doesnt necessarily mean we should move by the same delta maybe?
// ................6.........
// ................7.........
// ................8.........
// ................9.........
// ..........................
// ..........................
// ..........................
// ...........s..............  (9 covers s)
// ..........................
// ..........................
// ..........................
// ..........................
// ..........................
