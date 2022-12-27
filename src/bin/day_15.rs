use nom::{
    bytes::complete::tag, character::streaming::alphanumeric0, multi::separated_list0, IResult,
};

struct Sensor {
    pos: (i32, i32),
    closest_beacon: (i32, i32),
    manhattan_distance: i32,
}

impl Sensor {
    fn new(sensor_pos: (i32, i32), beacon_pos: (i32, i32)) -> Self {
        let manhattan_distance =
            i32::abs(sensor_pos.0 - beacon_pos.0) + i32::abs(sensor_pos.1 - beacon_pos.1);
        return Sensor {
            pos: sensor_pos,
            closest_beacon: beacon_pos,
            manhattan_distance: manhattan_distance,
        };
    }
}

fn parse_sensor(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, split_string) = separated_list0(tag(":"), alphanumeric0)(input)?;

    return Ok((input, split_string));
}

// fn parse_sensor_location(input: &str) -> IResult<&str, (i32, i32)> {}
// fn parse_beacon_location(input: &str) -> IResult<&str, (i32, i32)> {}
fn main() {
    println!("Hello my dopod");

    println!("{:?}", parse_sensor("Hello : dfvjkndfv"))
}

fn part_one(text_input: &str, row_num: u32) -> u32 {
    return 1;
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
