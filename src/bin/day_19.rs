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

#[derive(Clone)]
struct State{
    resources: HashMap<&'static str, u32>,
    robots: HashMap<&'static str, u32>,
}
impl State {
    fn new() -> Self {
        let mut resources:HashMap<&'static str, u32> = HashMap::new();
        for resource in RESOURCE_TYPES.iter() {
            resources.insert(resource, 0);
        }
        let mut robots:HashMap<&'static str, u32> = HashMap::new();
        for robot_type in ROBOT_TYPES.iter() {
            robots.insert(robot_type, 0);
        }
        robots.insert("ore_robot", 1); // always start with one ore robot
                                       //
        return State{ resources, robots}; 

    }

    fn get_num_geodes(&self) -> u32 {
        return *self.resources.get("geode").unwrap();
    }

    fn increase_resources(&mut self) {
        for (resource, robot) in RESOURCE_TYPES.iter().zip(ROBOT_TYPES.iter()){
            let count = self.resources.entry(resource).or_insert(0);
            let num_robots = self.robots.get(robot).expect("All robots should exist inside the inventory.robots hashmap"); // We know that the value exists
            *count += num_robots;
            //println!("{} {} collecting robot collected {} {}. You now have {} {}", num_robots, resource, num_robots, resource, *count, resource);
        }
    }

    fn build_robot(&mut self, robot_type: Option<&'static str>, robot_costs:&HashMap<&'static str, Vec<ResourceRequirements>>) {
       match robot_type {
            Some(robot_type) => {

                for requirement in robot_costs.get(robot_type).expect("The robot type must exist") {
                    let num_resource = self.resources.entry(requirement.resource).or_insert(0);
                    *num_resource -= requirement.cost;
                }

                let robot_count = self.robots.entry(robot_type).or_insert(0);
                *robot_count += 1;
                //println!("Purchased: {}, with {:?}", robot_type, robot_costs.get(robot_type).expect("The robot type must exist"));
            },
            None => ()
       } 
    }
}

#[derive(Clone)]
struct Simulation<'a> {
    blueprint: &'a Blueprint,
    max_resource_requirements: HashMap<&'static str, u32>
}

impl<'a> Simulation<'a>{
    fn new(blueprint: &'a Blueprint) -> Self {

        // create the hashmap containing the max amount of each resource that is required to build
        // a robot
        let mut max_resource_requirements:HashMap<&'static str, u32> = HashMap::new();
        for (_, costs) in &blueprint.robot_costs {
            for resource_requirement in costs {
                let resource_cost = max_resource_requirements.entry(&resource_requirement.resource).or_insert(1); 
                if *resource_cost < resource_requirement.cost {
                    *resource_cost = resource_requirement.cost;
                }
            } 
        }
        max_resource_requirements.insert("geode",u32::MAX);//geodes don't have a max as we always
                                                           //want to buy them

        return Simulation {blueprint:&blueprint, max_resource_requirements};
    }

    fn run(&mut self,state: &State, robot_built_last_round: Option<&'static str>, time_remaining: u32, most_num_geodes:&mut u32) {

        let mut state = state.clone();
        
        if time_remaining == 0 {
            // simulation for this branch is done
            
            let current_branch_num_geodes = state.get_num_geodes();
            if *most_num_geodes < current_branch_num_geodes {
                *most_num_geodes = current_branch_num_geodes;
            }

            return;

        }

        state.build_robot(robot_built_last_round, &self.blueprint.robot_costs);
        let branches = self.try_build_robots(&state);

        state.increase_resources();

        for possible_robot in branches {
            // Only run if the maximum possible geodes that can be built from the current state is
            // greater than the current most_num_geodes (branch and bound)

            if self.get_max_bound(&state, possible_robot, time_remaining -1) > *most_num_geodes {
                self.run(&state,possible_robot, time_remaining - 1, most_num_geodes);
            }
    }

    }

    fn get_max_bound(&self, state:&State,robot_decision_branch: Option<&'static str>, time_remaining:u32) -> u32 {
        // Assume that we either build a geode robot (have enough obsidian) or we build obsidian
        let mut state = state.clone();
        state.build_robot(robot_decision_branch, &self.blueprint.robot_costs);
        let geode_obsidian_cost = self.blueprint.robot_costs.get("geode_robot").unwrap().iter().find(|el| el.resource=="obsidian").unwrap().cost;


        for _ in (0..time_remaining).rev() {
            state.increase_resources();

            if state.resources.get("obsidian").unwrap() >= &geode_obsidian_cost {
                let num_resource = state.resources.entry("obsidian").or_insert(0);
                *num_resource -= geode_obsidian_cost;
                

                let robot_count = state.robots.entry("geode_robot").or_insert(0);
                *robot_count += 1;
            }
            else {
                let robot_count = state.robots.entry("obsidian_robot").or_insert(0);
                *robot_count += 1;
            }

        }

        return state.get_num_geodes();
    }

    fn try_build_robots(&self, state:&State) -> Vec<Option<&'static str>> {
        let mut possible_robot_decisions:Vec<Option<&'static str>> = vec![];
        // Reversed so that geode gets checked first. This is because we always want to get a geode robot if we can afford it
        for (robot,resource) in ROBOT_TYPES.iter().zip(RESOURCE_TYPES).rev() {
            if  self.can_afford_robot(robot,state) && self.should_buy_robot(&state.robots, robot, resource) {

                possible_robot_decisions.push(Some(*robot));
                if *robot == "geode_robot" {
                    return possible_robot_decisions;
                }
            }
        }

        possible_robot_decisions.push(None); // the case where we just don't buy any robots
        return possible_robot_decisions;
    }

    fn can_afford_robot(&self, robot_type: &str, state:&State) -> bool {

        for requirement in self.blueprint.robot_costs.get(robot_type).expect("The robot type must exist") {
            let num_resource = state.resources.get(requirement.resource).expect("The resource must exist");
            
            if *num_resource < requirement.cost {
                return false;
            }
        }

        return true;
    }

    fn should_buy_robot(&self, robots:&HashMap<&'static str, u32>,robot_type: &str, resource: &str) -> bool {

            let num_robots = *robots.get(robot_type).unwrap(); 
            let at_capacity = num_robots >= *self.max_resource_requirements.get(resource).unwrap();
            if !at_capacity {
                return true; 
             }
            return false;
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
    println!("Part Two: {}", part_two(&input_text,32));

}

fn part_one(input_text: &str, num_minutes: u32) -> u32 {

    let blueprints: Vec<Blueprint> = input_text.lines().map(|line| parse_blueprint(line).expect("Failed to parse blueprint").1).collect();

    let mut total_quality_levels = 0;
    blueprints.iter().for_each(|blueprint| total_quality_levels += blueprint.id * process_blueprint(&blueprint, num_minutes));

    return total_quality_levels;
}

fn part_two(input_text: &str, num_minutes: u32) -> u32 {

    let blueprints: Vec<Blueprint> = input_text.lines().map(|line| parse_blueprint(line).expect("Failed to parse blueprint").1).collect();

    let mut product_num_geodes = 1;
    blueprints.iter().take(3).for_each(|blueprint| product_num_geodes *= process_blueprint(&blueprint, num_minutes));

    return product_num_geodes;
}

fn process_blueprint(blueprint: &Blueprint, num_minutes: u32) -> u32 {

    let mut simulation = Simulation::new(blueprint);
    let state = State::new();
    let mut most_num_geodes = 0;

    simulation.run(&state,None,num_minutes,&mut most_num_geodes);
    return most_num_geodes;
}


#[test]
fn test_part_one(){
    let input_text = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";


    assert_eq!(part_one(input_text,24), 33);
}

#[test]
fn test_part_two(){
    let input_text = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";


    assert_eq!(part_two(input_text,32), 56*62);
}

#[test]
fn test_first_blueprint(){

    let input_text = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";

    assert_eq!(part_one(input_text,24), 9);
}

#[test]
fn test_second_blueprint(){

    dbg!("Hello this terst is starting");
    let input_text = "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    assert_eq!(part_one(input_text,24), 24);
}
