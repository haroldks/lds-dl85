use std::io::Error;
use std::fs::File;
use serde_json::to_writer;
use serde::{Deserialize, Serialize};
use crate::solution::tree::Tree;


#[derive(Serialize, Deserialize)]
pub struct Export {
    pub support: u64,
    pub max_depth: u64,
    pub timeout: f64,
    pub allow_discrepancy: bool,
    pub use_information_gain: bool,

    pub max_discrepancy : Option<usize>,
    pub discrepancy: Option<usize>,
    pub error: f64,
    pub accuracy: f64,
    pub cache_size: u64,

    pub tree_depth: u64,
    pub tree: Tree
}


impl Export {
    pub fn new() -> Export{
        Export {
            support: 0,
            error: 0.0,
            cache_size: 0,
            timeout: 0.0,
            discrepancy: None,
            max_discrepancy: None,
            tree_depth: 0,
            allow_discrepancy: false,
            use_information_gain: false,
            tree: Tree {
                root: None,
                left: None,
                right: None,
                is_leaf: false,
                max_class: 0,
                error: None,
                leaf_error: 0.0,
                current_depth: 0
            },
            accuracy: 0.0,
            max_depth: 0
        }
    }


    pub fn to_json(&self, filename: String) -> Result<(), Error> {
        if let Err(e) = to_writer(&File::create(filename)?, &self) {
            println!("File Creating error: {}", e.to_string());
        };
        Ok(())
    }

}
