use crate::mining::types_def::Attribute;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    pub root: Option<Attribute>,
    pub left: Option<Box<Tree>>,
    pub right: Option<Box<Tree>>,
    pub is_leaf: bool,
    pub max_class: usize,
    pub error: Option<f64>,
    pub leaf_error: f64,
    pub current_depth: u64,
}

impl Tree {
    pub fn new(root: Option<Attribute>) -> Tree {
        Tree {
            root,
            left: None,
            right: None,
            is_leaf: false,
            max_class: 0,
            error: None,
            leaf_error: <f64>::INFINITY,
            current_depth: <u64>::MAX,
        }
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Err(e) = writeln!(f, "{{  Attribute: {:?}", self.root) {
            println!("Writing error: {}", e.to_string());
        };
        if self.is_leaf {
            if let Err(e) = writeln!(
                f,
                "  Error:  {}, Max Class:  {}",
                self.error.unwrap(),
                self.max_class
            ) {
                println!("Writing error: {}", e.to_string());
            };
        } else {
            for tree in &self.left {
                if let Err(e) = writeln!(f, "Left:  {}", tree) {
                    println!("Writing error: {}", e.to_string());
                };
            }
            for tree in &self.right {
                if let Err(e) = writeln!(f, "Right:  {}", tree) {
                    println!("Writing error: {}", e.to_string());
                };
            }
        }
        if let Err(e) = write!(f, " }}") {
            println!("Writing error: {}", e.to_string());
        };
        Ok(())
    }
}
