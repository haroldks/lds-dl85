use crate::mining::types_def::{Attribute, Item};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Node {
    pub current_depth: u64, // FIXME: Useless I think
    pub test: Item,
    pub leaf_error : f64,
    pub node_error : f64,
    pub lower_bound : f64,
    // left: NodeData,
    // right: NodeData,

}


impl Node {

    pub fn new(test: Item, current_depth: u64) -> Node {
        Node {
            current_depth,
            test,
            leaf_error: <f64>::MAX,
            node_error: <f64>::MAX,
            lower_bound: 0f64
        }
    }

}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "[current_depth : {},  test:  {},  leaf_error:  ({}, {}),  node_error:  {}]", self.current_depth, self.test.0, self.test.1, self.leaf_error, self.node_error)
    }
}
