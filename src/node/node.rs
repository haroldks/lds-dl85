use std::fmt;
use std::fmt::Formatter;

use crate::mining::types_def::Attribute;

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub is_explored: bool,
    pub current_depth: u64,
    // FIXME: Useless I think
    pub test: Attribute,
    pub leaf_error: f64,
    pub current_discrepancy: Option<u64>,
    pub node_error: f64,
    pub lower_bound: f64,
    pub max_class: usize,
    pub is_leaf: bool,
    // left: NodeData,
    // right: NodeData,
}


impl Node {
    pub fn new(test: Attribute, current_depth: u64) -> Node {
        Node {
            is_explored: false,
            current_depth,
            test,
            current_discrepancy: None,
            leaf_error: <f64>::MAX,
            node_error: <f64>::MAX,
            lower_bound: 0f64,
            max_class: <usize>::MAX,
            is_leaf: false,
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "[current_depth : {}, is_leaf  :  {}, is_explored  :  {},   test:  {},  leaf_error:  {},  node_error:  {}, max class : {}, current_discrepancy  :  {:?}]", self.current_depth, self.is_leaf, self.is_explored, self.test, self.leaf_error, self.node_error, self.max_class, self.current_discrepancy)
    }
}
