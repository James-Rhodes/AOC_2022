struct Node {
    value: i64,
    next_index: usize,
    prev_index: usize
}

struct LinkedList{
    nodes: Vec<Node>,
    head_index: usize,
    tail_index: usize,
    zero_index: usize,
    length:usize

}

impl LinkedList{
    fn new(input_text: &str, decryption_key: i64) -> Self {

        let mut zero_index = 0;

        let mut nodes: Vec<Node> = input_text.lines().enumerate().map(|(index, line)| {
            let value = line.parse::<i64>().unwrap() * decryption_key; 
            if value == 0 {
                zero_index = index;
            }
            
            return Node{
                value,
                next_index: index + 1,
                prev_index: 0.max(index as i64 - 1) as usize
            }
        }).collect();

        let length = nodes.len();
        nodes[0].prev_index = length - 1;
        nodes[length - 1].next_index = 0;

        return LinkedList{
            nodes,
            head_index: 0,
            tail_index: length - 1,
            zero_index,
            length
        }
    }


    fn shift_number(&mut self, base_index:usize){
        // base_index is the index we are shifting FROM THE ORIGINAL ORDER
        // not the current state of the linkedlist
        
        let current_value = self.nodes[base_index].value;
        let move_amount = self.get_move_amount(current_value);

        let node_to_move = base_index;

        if move_amount == 0 {
            return;
        }

        // Detach the current node
        let next_node = self.nodes[node_to_move].next_index;
        let prev_node = self.nodes[node_to_move].prev_index;

        self.nodes[prev_node].next_index = next_node;
        self.nodes[next_node].prev_index = prev_node;

        if base_index == self.head_index {
            self.head_index = next_node;
        }
        else if base_index == self.tail_index {
            self.tail_index = prev_node;
        }


        // Move to where we are reinserting the node
        let mut current_index = node_to_move;

        if move_amount >= 0 {
            for _ in 0..(move_amount.abs() as usize + 1) {
                current_index = self.nodes[current_index].next_index;
            }

        }else{
            for _ in 0..(move_amount.abs() as usize) {
                current_index = self.nodes[current_index].prev_index;
            }
        }

        // Re-Insert node in list
        let prev_node = self.nodes[current_index].prev_index;
        self.nodes[current_index].prev_index = node_to_move;
        self.nodes[prev_node].next_index = node_to_move;

        self.nodes[node_to_move].next_index = current_index;
        self.nodes[node_to_move].prev_index = prev_node;
    }

    fn get_move_amount(&self,current_value: i64) -> i64 {
        return current_value - (current_value/(self.nodes.len() as i64 - 1)) * (self.nodes.len() as i64 - 1);
    }

    fn get_value_at_position_from_zero(&self,pos: usize) -> i64 {

        let mut current_node = self.zero_index;
        for _ in 0..pos {
            current_node = self.nodes[current_node].next_index;
        }

        return self.nodes[current_node].value;
        
    }

    #[allow(dead_code)]
    fn print_list(&mut self) {
        let mut current_index = self.head_index;
        for _ in 0..self.length {
            print!("{}, ", self.nodes[current_index].value);
            current_index = self.nodes[current_index].next_index;
        }      
        println!("\n");
    }
}

fn part_one(input_text: &str) -> i64 {

    let mut linked_list = LinkedList::new(input_text,1);

    for i in 0..linked_list.length {
        linked_list.shift_number(i);
    }


    let pos_1000 =linked_list.get_value_at_position_from_zero(1000);  
    let pos_2000 =linked_list.get_value_at_position_from_zero(2000);  
    let pos_3000 =linked_list.get_value_at_position_from_zero(3000);  

    return pos_1000 + pos_2000 + pos_3000;
}

fn part_two(input_text: &str, num_mix_rounds: usize) -> i64 {

    let decryption_key = 811589153;

    let mut linked_list = LinkedList::new(input_text,decryption_key);

    for _ in 0..num_mix_rounds {
        for i in 0..linked_list.length {
            linked_list.shift_number(i);
        }
    }

    let pos_1000 =linked_list.get_value_at_position_from_zero(1000);  
    let pos_2000 =linked_list.get_value_at_position_from_zero(2000);  
    let pos_3000 =linked_list.get_value_at_position_from_zero(3000);  

    return pos_1000 + pos_2000 + pos_3000;
}


fn main() { 

    let input_text = std::fs::read_to_string("./inputs/input_day_20.txt").unwrap();

    println!("Part One: {}", part_one(&input_text));
    println!("Part Two: {}", part_two(&input_text,10));
}



#[test]
fn test_part_one(){

    let input_text = "1
2
-3
3
-2
0
4";
    assert_eq!(part_one(input_text), 3);
}

#[test]
fn test_part_two(){

    let input_text = "1
2
-3
3
-2
0
4";
    assert_eq!(part_two(input_text,10), 1623178306);
}

#[test]
fn test_cyclical_optimisation(){

    let input_text = "1
2
-3
3
-2
0
4";

    let linked_list = LinkedList::new(input_text,1);

    assert_eq!(linked_list.get_move_amount(1), 1);
    assert_eq!(linked_list.get_move_amount(-1), -1);
    assert_eq!(linked_list.get_move_amount(3), 3);
    assert_eq!(linked_list.get_move_amount(-3), -3);
    assert_eq!(linked_list.get_move_amount(10), 4);
    assert_eq!(linked_list.get_move_amount(-10), -4);
    assert_eq!(linked_list.get_move_amount(6_000_000), 0);




}
