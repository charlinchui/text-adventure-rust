use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Deserialize, Serialize, Debug)]
struct Node {
    id: usize,
    node_type: String,
    value: String,
    options: Vec<Option>
}

#[derive(Deserialize, Serialize, Debug)]
struct Option {
    value: String,
    next_node: usize,
}

fn main() {
    let json = std::fs::read_to_string("data/story.json").unwrap();
    let nodes = from_str::<Vec<Node>>(&json).unwrap();
    println!("{:#?}", nodes);
    let node = read_node(nodes, 1);
    println!("NODE :D : {:?}", node)
}

fn read_node(nodes: Vec<Node>,id: usize) -> Node {
    for node in nodes {
        if &node.id == &id {
            return node;
        }
    }
    return Node{
        id: 9999,
        node_type: "Does not exist".to_string(),
        value: "This node does not exist".to_string(),
        options: Vec::from([Option{value:"".to_string(), next_node:999999}])
    }
}
