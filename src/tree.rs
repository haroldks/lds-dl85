use std::fmt;
use std::fmt::Formatter;

use crate::mining::types_def::Attribute;

#[derive(Debug)]
pub struct Tree {
    pub root: Attribute,
    pub left: Vec<Tree>,
    pub right: Vec<Tree>,
    pub is_leaf: bool,
    pub max_class: usize,
    pub error: Option<f64>,
}


impl Tree {
    pub fn new(root: Attribute) -> Tree {
        Tree {
            root,
            left: vec![],
            right: vec![],
            is_leaf: false,
            max_class: 0,
            error: None,
        }
    }
}


impl fmt::Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{  Attribute: {}", self.root);
        if self.is_leaf {
            writeln!(f, "  Error:  {}, Max Class:  {}", self.error.unwrap(), self.max_class);
        } else {
            for tree in &self.left {
                writeln!(f, "Left:  {}", tree);
            }
            for tree in &self.right {
                writeln!(f, "Right:  {}", tree);
            }
        }
        write!(f, " }}");
        Ok(())
    }
}
