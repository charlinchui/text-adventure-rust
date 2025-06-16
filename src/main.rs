use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::io::stdin;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Node {
    id: isize,
    node_type: String,
    value: String,
    options: Vec<Option>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Option {
    value: String,
    next_node: isize,
}

fn main() {
    let json = std::fs::read_to_string("data/story.json").unwrap();
    let nodes = from_str::<Vec<Node>>(&json).unwrap();
    let mut node = read_node(nodes.clone(), 0);

    loop {
        let mut buffer = String::new();
        let current_node = node.clone();
        println!("{}", current_node.value);
        if node.node_type == "END" {
            break;
        }
        for option in &node.options {
            println!("{}", option.value);
        }
        println!("Select Option:");
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let selection = parse_string(&buffer, current_node.options.len());
                let next_node = read_node(nodes.clone(), current_node.options[selection-1].next_node);
                mutate_node(&mut node, &next_node); 
            },
            Err(err) => {
                println!("Wrong input: {}", err);
            }
        }
    };
}

fn mutate_node(base_node: &mut Node, new_node: &Node) {
    base_node.id = new_node.id.clone();
    base_node.node_type = new_node.node_type.clone();
    base_node.value = new_node.value.clone();
    base_node.options = new_node.options.clone();
}

fn read_node(nodes: Vec<Node>,id: isize) -> Node {
    for node in nodes {
        if node.id == id {
            return node;
        }
    }
    return Node{
        id: -1,
        node_type: String::from("END"),
        value: "This node does not exist".to_string(),
        options: Vec::from([Option{value:"".to_string(), next_node: -1}])
    }
}

fn parse_string(s: &str, max: usize) -> usize {
    if let Ok(num) = s.trim().parse::<usize>() {
        if num <= max && num > 0 {
            return num
        }
    }
    println!("Invalid input. Please enter a valid number.");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Failed to read line");
    parse_string(&buffer, max)
}
