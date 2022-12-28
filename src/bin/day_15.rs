use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt, recognize},
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
struct Sensor {
    pos: (i32, i32),
    closest_beacon: (i32, i32),
    manhattan_distance: i32,
}

impl Sensor {
    fn new(sensor_pos: (i32, i32), beacon_pos: (i32, i32)) -> Self {
        let manhattan_distance =
            i32::abs(sensor_pos.0 - beacon_pos.0) + i32::abs(sensor_pos.1 - beacon_pos.1);
        let curr_sensor = Sensor {
            pos: sensor_pos,
            closest_beacon: beacon_pos,
            manhattan_distance: manhattan_distance,
        };

        return curr_sensor;
    }

    pub fn add_impossible_beacon_locations(
        &self,
        set: &mut HashSet<(i32, i32)>,
        target_y_row: i32,
    ) {
        let y_delta = target_y_row - self.pos.1;
        if y_delta > self.manhattan_distance {
            return;
        }
        let max_x_dist = self.manhattan_distance - i32::abs(y_delta);
        for x_delta in -max_x_dist..=max_x_dist {
            let x_coord = self.pos.0 + x_delta;
            let y_coord = self.pos.1 + y_delta;
            if !(x_coord == self.closest_beacon.0 && y_coord == self.closest_beacon.1)
                && !(x_coord == self.pos.0 && y_coord == self.pos.1)
            {
                set.insert((x_coord, y_coord));
            }
        }
    }
    fn point_within_coverage_area(&self, point: (i32, i32)) -> bool {
        let manhattan_dist_from_sensor =
            i32::abs(point.0 - self.pos.0) + i32::abs(point.1 - self.pos.1);

        return manhattan_dist_from_sensor <= self.manhattan_distance;
    }
}

fn parse_sensor<'a>(input: &'a str) -> IResult<&'a str, Sensor> {
    let (input, _) = tag("Sensor at ")(input)?;
    let (input, sensor_coord) = parse_point(input)?;
    let (input, _) = tag(": closest beacon is at ")(input)?;
    let (input, beacon_coord) = parse_point(input)?;
    return Ok((input, Sensor::new(sensor_coord, beacon_coord)));
}

fn parse_point(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, _) = tag("x=")(input)?;

    let (input, x_coord) = map_res(recognize(preceded(opt(tag("-")), digit1)), |num| {
        i32::from_str_radix(num, 10)
    })(input)?;

    let (input, _) = tag(", y=")(input)?;

    let (input, y_coord) = map_res(recognize(preceded(opt(tag("-")), digit1)), |num| {
        i32::from_str_radix(num, 10)
    })(input)?;

    return Ok((input, (x_coord, y_coord)));
}

fn main() {
    let text_input = std::fs::read_to_string("./inputs/input_day_15.txt").unwrap();

    println!("Part One: {}", part_one(&text_input, 2000000));
    println!("Part Two: {}", part_two(&text_input, 0, 4000000));
}

fn part_one(text_input: &str, row_num: i32) -> u32 {
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    text_input.lines().for_each(|line| {
        let sensor = parse_sensor(line).unwrap().1;
        sensor.add_impossible_beacon_locations(&mut set, row_num);
    });

    let mut num_impossible_locations = 0;

    set.iter().for_each(|coord| {
        if coord.1 == row_num {
            num_impossible_locations += 1;
        }
    });
    return num_impossible_locations;
}
fn part_two(text_input: &str, lower_bound: i32, upper_bound: i32) -> i64 {
    let mut sensors = vec![];
    text_input.lines().for_each(|line| {
        sensors.push(parse_sensor(line).unwrap().1);
    });

    for y in lower_bound..=upper_bound {
        let mut x = lower_bound;
        while x <= upper_bound {
            let mut point_within_sensors = false;
            for sensor in &sensors {
                point_within_sensors = sensor.point_within_coverage_area((x, y));

                if point_within_sensors {
                    // Jump x ahead until we are no longer within its range and then continue
                    let max_x_reach = sensor.pos.0
                        + i32::abs(sensor.manhattan_distance - i32::abs(sensor.pos.1 - y));

                    x = max_x_reach;
                    break;
                }
            }

            if !point_within_sensors {
                return x as i64 * 4000000 + y as i64;
            }

            x += 1;
        }
    }
    return -1;
}
#[test]
fn part_one_test() {
    let text_input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    assert_eq!(part_one(text_input, 10), 26);
}

#[test]
fn part_two_test() {
    let text_input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    assert_eq!(part_two(text_input, 0, 20), 56000011);
}

#[test]
fn test_point_within_coverage() {
    let sensor = parse_sensor("Sensor at x=8, y=7: closest beacon is at x=2, y=10")
        .unwrap()
        .1;

    println!("{:?}", sensor);

    assert!(sensor.point_within_coverage_area((8, -2)));
    assert!(sensor.point_within_coverage_area((17, 7)));
    assert!(sensor.point_within_coverage_area((8, 16)));
    assert!(sensor.point_within_coverage_area((-1, 7)));
    assert!(!sensor.point_within_coverage_area((20, -2)));
    assert!(!sensor.point_within_coverage_area((14, 1)));
}
