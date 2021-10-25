use std::fmt;
use std::fmt::Formatter;

use crate::mining::types_def::Attribute;

#[derive(Debug, Clone)]
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
        if let Err(e) = writeln!(f, "{{  Attribute: {}", self.root){
            println!("Writing error: {}", e.to_string());
        };
        if self.is_leaf {
            if let Err(e) = writeln!(f, "  Error:  {}, Max Class:  {}", self.error.unwrap(), self.max_class){
                println!("Writing error: {}", e.to_string());
            };
        } else {
            for tree in &self.left {
                if let Err(e) =  writeln!(f, "Left:  {}", tree){
                    println!("Writing error: {}", e.to_string());
                };
            }
            for tree in &self.right {
                if let Err(e) = writeln!(f, "Right:  {}", tree){
                    println!("Writing error: {}", e.to_string());
                };
            }
        }
        if let Err(e) =write!(f, " }}"){
            println!("Writing error: {}", e.to_string());
        };
        Ok(())
    }
}
