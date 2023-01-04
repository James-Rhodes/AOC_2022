use std::collections::{HashMap,HashSet};
use std::ops::{Add,Sub};

#[derive(Copy,Clone,Debug,Default)]
struct Coordinate {
    x: f64,
    y: f64,
    z: f64
}

impl ToString for Coordinate {
    fn to_string(&self) -> String {
        return self.x.to_string() + "," +  &self.y.to_string() + "," + &self.z.to_string();
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

#[derive(Debug,Default)]
struct Cube {
    center: Coordinate,
    surface_centers: [Coordinate; 6]
}

impl Cube {
    fn new(center_x: f64, center_y: f64, center_z: f64) -> Self {
        
        return Cube {
            center: Coordinate { x: center_x, y: center_y, z: center_z },
            surface_centers: [
            Coordinate{ x: center_x + 0.5, y:center_y, z: center_z},
            Coordinate{ x: center_x - 0.5, y:center_y, z: center_z},
            Coordinate{ x: center_x, y:center_y + 0.5, z: center_z},
            Coordinate{ x: center_x, y:center_y - 0.5, z: center_z},
            Coordinate{ x: center_x, y:center_y, z: center_z + 0.5},
            Coordinate{ x: center_x, y:center_y, z: center_z - 0.5}]
        }
    }
}

struct ConnectedCubes{
    connection_point: Coordinate,
    connected_cubes_indices: [usize;2], // The indices in the cube array that is contains this connection point (mid point of the faces)
    frequency: u32
}

struct BoundingCube {
    x_bounds: (f64,f64),
    y_bounds: (f64,f64),
    z_bounds: (f64,f64),
}

impl BoundingCube {
    fn new(cubes: &Vec<Cube>) -> Self {
        
        let mut coords = cubes.iter().map(|cube| {
            cube.surface_centers.iter().map(|coord| coord.x).collect::<Vec<f64>>()
        }).flatten().collect::<Vec<f64>>();

        coords.sort_by(|a,b| a.partial_cmp(b).unwrap());

        let x_min = coords.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let x_max = coords.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        let mut coords = cubes.iter().map(|cube| {
            cube.surface_centers.iter().map(|coord| coord.y).collect::<Vec<f64>>()
        }).flatten().collect::<Vec<f64>>();

        coords.sort_by(|a,b| a.partial_cmp(b).unwrap());

        let y_min = coords.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let y_max = coords.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));


        let mut coords = cubes.iter().map(|cube| {
            cube.surface_centers.iter().map(|coord| coord.z).collect::<Vec<f64>>()
        }).flatten().collect::<Vec<f64>>();

        coords.sort_by(|a,b| a.partial_cmp(b).unwrap());

        let z_min = coords.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let z_max = coords.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));


        let x_bounds = (x_min,x_max);
        let y_bounds = (y_min,y_max);
        let z_bounds = (z_min, z_max);
        return Self {
            x_bounds,
            y_bounds,
            z_bounds
        };
        
    }
    
    fn contains_point(&self, point: &Coordinate) -> bool {
        return !(point.x < self.x_bounds.0 || point.x > self.x_bounds.1 || point.y < self.y_bounds.0 || point.y > self.y_bounds.1 || point.z < self.z_bounds.0 || point.z > self.z_bounds.1);
    }

    #[allow(dead_code)]
    fn get_coordinate_boundaries(&self) -> ((f64,f64),(f64,f64),(f64,f64)){
        return (self.x_bounds, self.y_bounds, self.z_bounds);
    }
}

fn main() {
    println!("Hello World");

    let input_text = std::fs::read_to_string("./inputs/input_day_18.txt").unwrap();

    println!("Part One: {}", part_one(&input_text));
    println!("Part Two: {}", part_two(&input_text));
}


fn parse_coordinates(input:&str) -> Vec<Cube> {
    return input.lines().map(|coord| {
        let coord = coord.split(",").map(|num| num.parse::<f64>().unwrap()).collect::<Vec<f64>>();
        let result = Cube::new(coord[0], coord[1], coord[2]);
        return result;
    }).collect::<Vec<Cube>>()
}

fn get_side_frequency(cubes: &Vec<Cube>) -> HashMap<String, ConnectedCubes>{
    
    let mut side_frequency: HashMap<String, ConnectedCubes> = HashMap::new();

    
    cubes.iter().enumerate().for_each(|(index,cube)| {
        for coord in &cube.surface_centers {
            side_frequency.entry(coord.to_string()).and_modify(|connected_cube| {
                connected_cube.frequency += 1;
                connected_cube.connected_cubes_indices[1] = index;
            }).or_insert(ConnectedCubes { connection_point: coord.clone(), connected_cubes_indices: [index,0], frequency: 1 });
        }
    });

    return side_frequency; 
}

fn part_one(input_text: &str) -> u32 {
    let cubes = parse_coordinates(input_text);

    return get_side_frequency(&cubes).iter().filter(|(_, connected_cube)| connected_cube.frequency == 1).count() as u32;
}

fn part_two(input_text: &str) -> u32 {
    // Plan: Start with the result from part one. Then just iterate through the cubes if there are any cubes that
    // check all 8 directions to detemine if it is an internal cube or not? Then subtract the edges that are shared with the non-cube
    let cubes = parse_coordinates(input_text);
    let side_frequency = get_side_frequency(&cubes);

    
    let bounding_cube = BoundingCube::new(&cubes);
    
    let mut external_faces = 0; // count the number of faces that you can iterate til max boundf that doesn't collide with the side_frequency map.

    side_frequency.iter().filter(|(_,cube)| cube.frequency == 1).for_each(|(_,cube)| { 
        let cube_center = cubes[cube.connected_cubes_indices[0]].center;
        let cube_center = Coordinate { x: cube_center.x as f64, y: cube_center.y as f64, z: cube_center.z as f64 }; 

        let direction_to_check = cube.connection_point - cube_center; // The direction to iterate through
        let direction_to_check = Coordinate {x: direction_to_check.x * 2.0, y:direction_to_check.y * 2.0,z:direction_to_check.z * 2.0};

        let current_air_pocket_coord = cube_center + direction_to_check;

        if air_pocket_is_external(current_air_pocket_coord, &side_frequency, &bounding_cube){
            external_faces += 1;
        }
    });



    return external_faces; 
}


fn air_pocket_is_external(air_pocket_coord: Coordinate, cube_sides: &HashMap<String, ConnectedCubes>, bounding_cube: &BoundingCube) -> bool {
    let mut visited_air_pockets:HashSet<String> = HashSet::new();

    return dfs_air_pocket_is_external(air_pocket_coord, cube_sides, &mut visited_air_pockets, bounding_cube);
}

fn dfs_air_pocket_is_external(air_pocket_coord: Coordinate, cube_sides: &HashMap<String, ConnectedCubes>, visited_air_pockets: &mut HashSet<String>, bounding_cube: &BoundingCube) -> bool {

    // outside the bounding cube so must be external
    if !bounding_cube.contains_point(&air_pocket_coord) {
        return true;
    }

    let mut is_external = false;    
    visited_air_pockets.insert(air_pocket_coord.to_string());

    let wall_up = air_pocket_coord + Coordinate{x:0.0,y:0.5,z:0.0};
    let wall_down = air_pocket_coord + Coordinate{x:0.0,y:-0.5,z:0.0};
    let wall_left = air_pocket_coord + Coordinate{x:-0.5,y:0.0,z:0.0};
    let wall_right = air_pocket_coord + Coordinate{x:0.5,y:0.0,z:0.0};
    let wall_forward = air_pocket_coord + Coordinate{x:0.0,y:0.0,z:0.5};
    let wall_backward = air_pocket_coord + Coordinate{x:0.0,y:0.0,z:-0.5};


    let up = air_pocket_coord + Coordinate{x:0.0,y:1.0,z:0.0};
    let down = air_pocket_coord + Coordinate{x:0.0,y:-1.0,z:0.0};
    let left = air_pocket_coord + Coordinate{x:-1.0,y:0.0,z:0.0};
    let right = air_pocket_coord + Coordinate{x:1.0,y:0.0,z:0.0};
    let forward = air_pocket_coord + Coordinate{x:0.0,y:0.0,z:1.0};
    let backward = air_pocket_coord + Coordinate{x:0.0,y:0.0,z:-1.0};

    if !visited_air_pockets.contains(&up.to_string()) && !cube_sides.contains_key(&wall_up.to_string()){
        is_external = is_external || dfs_air_pocket_is_external(up, cube_sides, visited_air_pockets, bounding_cube);
    }
    if !visited_air_pockets.contains(&down.to_string()) && !cube_sides.contains_key(&wall_down.to_string()){
        is_external = is_external || dfs_air_pocket_is_external(down, cube_sides, visited_air_pockets, bounding_cube);
    }
    if !visited_air_pockets.contains(&left.to_string()) && !cube_sides.contains_key(&wall_left.to_string()){
        is_external = is_external || dfs_air_pocket_is_external(left, cube_sides, visited_air_pockets, bounding_cube);
    }
    if !visited_air_pockets.contains(&right.to_string()) && !cube_sides.contains_key(&wall_right.to_string()){
        is_external = is_external || dfs_air_pocket_is_external(right, cube_sides, visited_air_pockets, bounding_cube);
    }
    if !visited_air_pockets.contains(&forward.to_string()) && !cube_sides.contains_key(&wall_forward.to_string()){
        is_external = is_external || dfs_air_pocket_is_external(forward, cube_sides, visited_air_pockets, bounding_cube);
    }
    if !visited_air_pockets.contains(&backward.to_string()) && !cube_sides.contains_key(&wall_backward.to_string()){
        is_external = is_external || dfs_air_pocket_is_external(backward, cube_sides, visited_air_pockets, bounding_cube);
    }

    return is_external;
}

#[test]
fn test_part_one() {
    let input_text = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    assert_eq!(part_one(input_text), 64);
}

#[test]
fn test_part_two() {
    let input_text = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    assert_eq!(part_two(input_text), 58);
}


#[test]
fn test_bounding_box(){

    let input_text = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
    let cubes = parse_coordinates(input_text);
    let bounding_cube = BoundingCube::new(&cubes);
    let ((x_min,x_max), (y_min,y_max), (z_min, z_max)) = bounding_cube. get_coordinate_boundaries();


    assert_eq!(x_min, 0.5);
    assert_eq!(x_max, 3.5);
    assert_eq!(y_min, 0.5);
    assert_eq!(y_max, 3.5);
    assert_eq!(z_min, 0.5);
    assert_eq!(z_max, 6.5);
}