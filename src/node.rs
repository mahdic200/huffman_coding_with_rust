
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Node {
    pub freq: u32,
    pub symbol: char,
    pub left: Option<Rc<Node>>,
    pub right: Option<Rc<Node>>,
}

impl Node {
    pub fn new(freq: u32, symbol: char) -> Node {
        Node {
            freq,
            symbol,
            left: None,
            right: None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        return self.left.is_none() && self.right.is_none();
    }
}

pub struct Associated {
    codes: RefCell<HashMap<char, String>>
}

impl Associated {
    pub fn new() -> Associated {
        Associated {
            codes: RefCell::new(HashMap::new()),
        }
    }

    pub fn calculate(&self, node: &Node, code: &str) {
        if node.is_leaf() {
            self.codes.borrow_mut().insert(node.symbol, code.to_string());
            return;
        }


        if let Some(left) = &node.left {
            self.calculate(left, &(code.to_owned() + "0"));
        }

        if let Some(right) = &node.right {
            self.calculate(right, &(code.to_owned() + "1"));
        }
    }

    pub fn get(&self) -> &RefCell<HashMap<char, String>> {
        return &self.codes;
    }
}


pub fn init_nodes(nodes_map: &BTreeMap<u32, char>) -> Vec<Node> {
    nodes_map
        .iter()
        .map(|(freq, symbol)| Node::new(*freq, *symbol))
        .collect::<Vec<Node>>()
}


pub fn make_tree(nodes: &mut Vec<Node>) -> Node {

    while nodes.len() > 1 {

        let left = Rc::new(nodes.remove(0));
        let right = Rc::new(nodes.remove(0));

        let new_node: Node = Node {
            freq: left.freq + right.freq,
            symbol: '\0',
            left: Some(Rc::clone(&left)),
            right: Some(Rc::clone(&right)),
        };
    
        nodes.push(new_node);
        nodes.sort_by(|first_node, second_node| {
            if first_node.freq == second_node.freq {
                if !first_node.is_leaf() {
                    return Ordering::Greater;
                }
            }
            first_node.freq.cmp(&second_node.freq)
        });
    
    }

    return nodes[0].clone();
}
