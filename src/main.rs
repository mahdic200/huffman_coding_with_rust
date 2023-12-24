pub mod node;

use std::collections::BTreeMap;
use node::{init_nodes, make_tree, Associated};

fn main() {

    let freqs: BTreeMap<u32, char> = BTreeMap::from([
        (16, 'g'),
        (15, 'f'),
        (5, 'e'),
        (6, 'd'),
        (1, 'c'),
        (3, 'b'),
        (4, 'a'),
    ]);
    
    let tree = make_tree(&mut init_nodes(&freqs));

    let codes = Associated::new();


    codes.calculate(&tree, "");
    println!("{:#?}", freqs);
    println!("{:#?}", codes.get());

}
