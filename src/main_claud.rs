// Import required crates
use std::collections::BTreeMap;
// use std::cmp::Ordering;

// Node structure for Huffman tree
#[derive(Clone)]
struct Node {
    freq: u32,
    symbol: char,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// Implementation of Node
impl Node {
    // Create new node
    fn new(freq: u32, symbol: char) -> Self {
        Node {
            freq,
            symbol,
            left: None,
            right: None,
        }
    }
}

// Function to build Huffman tree
fn build_tree(freqs: &BTreeMap<char, u32>) -> Node {
    // Create leaf nodes
    let mut nodes = freqs
        .iter()
        .map(|(symbol, freq)| Node::new(*freq, *symbol))
        .collect::<Vec<_>>();

    // Iterate until there is only one node left
    while nodes.len() > 1 {
        // Sort nodes by frequency
        nodes.sort_by(|a, b| a.freq.cmp(&b.freq));

        // Take two least frequent nodes
        let left = nodes.remove(0);
        let right = nodes.remove(0);

        // Create internal node with sum of frequencies
        let internal_node = Node {
            freq: left.freq + right.freq,
            symbol: '\0',
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };

        // Add internal node back to list
        nodes.push(internal_node.clone());
    }

    // Return root node
    nodes[0].clone()
}

// Function to traverse Huffman tree
// and assign codes
fn assign_codes(node: &Node, code: &str, codes: &mut BTreeMap<char, String>) {
    // If leaf node, set code
    if node.left.is_none() && node.right.is_none() {
        codes.insert(node.symbol, code.to_string());
        return;
    }

    // Traverse left sub-tree
    if let Some(left) = &node.left {
        assign_codes(left, &(code.to_owned() + "0"), codes);
    }

    // Traverse right sub-tree
    if let Some(right) = &node.right {
        assign_codes(right, &(code.to_owned() + "1"), codes);
    }
}

// Function to build Huffman codes
fn build_codes(tree: &Node) -> BTreeMap<char, String> {
    let mut codes = BTreeMap::new();
    assign_codes(tree, "", &mut codes);
    codes
}

fn main() -> () {
    // Example
    let freqs = BTreeMap::from([
        ('a', 5),
        ('b', 9),
        ('c', 12),
        ('d', 13),
        ('e', 16),
        ('f', 45),
    ]);

    let tree = build_tree(&freqs);
    let codes = build_codes(&tree);

    println!("Huffman codes: {:#?}", codes);
}
