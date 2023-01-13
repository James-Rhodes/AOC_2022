use nom::{IResult, bytes::complete::tag, sequence::delimited, character::complete};
use std::collections::HashMap;

const RESOURCE_TYPES:[&str;4] = ["ore", "clay", "obsidian", "geode"];
const ROBOT_TYPES:[&str;4] = ["ore_robot", "clay_robot", "obsidian_robot", "geode_robot"];

#[derive(Debug, Clone)]
struct ResourceRequirements {
    resource: &'static str,
    cost: u32
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: u32,
    robot_costs: HashMap<&'static str, Vec<ResourceRequirements>>
}

// #[derive(Debug)]
struct Inventory {
    resources: HashMap<&'static str, u32>,
    robots: HashMap<&'static str, u32>,
    strategies: HashMap<&'static str, fn(&HashMap<&'static str, u32>,&HashMap<&'static str, u32>)->bool>,
    blueprint: Blueprint
}

impl Inventory{
    fn new(blueprint: &Blueprint) -> Self {

        let mut resources:HashMap<&'static str, u32> = HashMap::new();
        for resource in RESOURCE_TYPES.iter() {
            resources.insert(resource, 0);
        }
        let mut robots:HashMap<&'static str, u32> = HashMap::new();
        for robot_type in ROBOT_TYPES.iter() {
            robots.insert(robot_type, 0);
        }

        let mut strategies: HashMap<&'static str, fn(&HashMap<&'static str, u32>,&HashMap<&'static str, u32>)->bool> = HashMap::new();
        strategies.insert("geode_robot", |robghts,resources| {
            println!("In geode_robot");
            return true; // Always buy geode robots if we can afford them
        });

        strategies.insert("obsidian_robot", |robots,resources| {
            println!("In obsidian_robot");
            return true; // Always buy geode robots if we can afford them
        });

        strategies.insert("clay_robot", |robots,resources| {
            println!("In clay_robot");
            return true; // Always buy geode robots if we can afford them
        });
        strategies.insert("ore_robot", |robots,resources| {
            println!("In ore_robot");
            return true; // Always buy geode robots if we can afford them
        });

        robots.insert("ore_robot", 1); // always start with one ore robot

        return Inventory {resources, robots, blueprint:blueprint.clone(), strategies:strategies };
    }

    fn update(&mut self) {

        self.try_build_robots();
        self.increase_resources();
    }

    fn get_quality(&self, id:u32) -> u32 {
        return self.resources.get("geode").unwrap() * id;
    }

    fn increase_resources(&mut self) {
        for (resource, robot) in RESOURCE_TYPES.iter().zip(ROBOT_TYPES.iter()){
            let count = self.resources.entry(resource).or_insert(0);
            *count += self.robots.get(robot).expect("All robots should exist inside the inventory.robots hashmap"); // We know that the value exists
        }
    }

    fn try_build_robots(&mut self){

        // Reversed so that geode gets checked first. This is because we always want to get a geode robot if we can afford it
        for robot in ROBOT_TYPES.iter().rev() {
            if  self.can_afford_robot(robot) && self.strategies.get(robot).unwrap()(&self.robots, &self.resources){
                self.build_robot(robot);
                return;
            }

        }
    }

    fn can_afford_robot(&self, robot_type: &str) -> bool {

        for requirement in self.blueprint.robot_costs.get(robot_type).expect("The robot type must exist") {
            let num_resource = self.resources.get(requirement.resource).expect("The resource must exist");
            
            if *num_resource < requirement.cost {
                return false;
            }
        }

        return true;
    }

    fn build_robot(&mut self, robot_type: &'static str) {
        
        for requirement in self.blueprint.robot_costs.get(robot_type).expect("The robot type must exist") {
            let num_resource = self.resources.entry(requirement.resource).or_insert(0);
            
            *num_resource -= requirement.cost;

            println!("Resource: {}, current amount: {}", requirement.resource, *num_resource);

        }
        
        let robot_count = self.robots.entry(robot_type).or_insert(0);
        *robot_count += 1;
        println!("Purchased: {}, with {:?}", robot_type, self.blueprint.robot_costs.get(robot_type).expect("The robot type must exist"));
    }

}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {

    let mut robot_costs: HashMap<&'static str, Vec<ResourceRequirements>> = HashMap::new();

    let (input, index) = delimited(tag("Blueprint "), complete::u32, tag(":"))(input)?;

    let (input, ore_robot_cost) = delimited(tag(" Each ore robot costs "), complete::u32, tag(" ore."))(input)?; 
    let ore_robot_requirements = vec![ResourceRequirements{ resource: "ore", cost:ore_robot_cost}];
    robot_costs.insert("ore_robot", ore_robot_requirements);

    let (input, clay_robot_cost) = delimited(tag(" Each clay robot costs "), complete::u32, tag(" ore."))(input)?; 
    let clay_robot_requirements = vec![ResourceRequirements{ resource: "ore", cost:clay_robot_cost}];
    robot_costs.insert("clay_robot", clay_robot_requirements);

    let (input, obsidian_robot_cost_ore) = delimited(tag(" Each obsidian robot costs "), complete::u32, tag(" ore"))(input)?; 
    let (input, obsidian_robot_cost_clay) = delimited(tag(" and "), complete::u32, tag(" clay."))(input)?; 
    let obsidian_robot_requirements = vec![ResourceRequirements{ resource: "ore", cost:obsidian_robot_cost_ore},ResourceRequirements{ resource: "clay", cost:obsidian_robot_cost_clay}];
    robot_costs.insert("obsidian_robot", obsidian_robot_requirements);


    let (input, geode_robot_cost_ore) = delimited(tag(" Each geode robot costs "), complete::u32, tag(" ore"))(input)?; 
    let (input, geode_robot_cost_obsidian) = delimited(tag(" and "), complete::u32, tag(" obsidian."))(input)?; 
    let geode_robot_requirements = vec![ResourceRequirements{ resource: "ore", cost:geode_robot_cost_ore},ResourceRequirements{ resource: "obsidian", cost:geode_robot_cost_obsidian}];
    robot_costs.insert("geode_robot", geode_robot_requirements);

    let blueprint = Blueprint{
        id: index,
        robot_costs
    };

    return Ok((input,blueprint));
}

fn main() {
    println!("Hello Day 19");    

    let input_text = std::fs::read_to_string("./inputs/input_day_19.txt").unwrap();

    println!("Part One: {}", part_one(&input_text,24));
}

fn part_one(input_text: &str, num_minutes: usize) -> u32 {

    let blueprints: Vec<Blueprint> = input_text.lines().map(|line| parse_blueprint(line).expect("Failed to parse blueprint").1).collect();

    let mut total_quality_levels = 0;
    blueprints.iter().for_each(|blueprint| total_quality_levels += process_blueprint(&blueprint, num_minutes));

    return total_quality_levels;
}

fn process_blueprint(blueprint: &Blueprint, num_minutes: usize) -> u32 {

    let mut inventory = Inventory::new(blueprint);
    
    for _ in 0..num_minutes {
        inventory.update();
        println!("{:?}", inventory.robots);
        println!("{:?}", inventory.resources);
    }
    return inventory.get_quality(blueprint.id);
}


#[test]
fn test_part_one(){
    let input_text = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";


    assert_eq!(part_one(input_text,24), 33);
}
