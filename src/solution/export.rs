use crate::solution::tree::Tree;
use serde::{Deserialize, Serialize};
use serde_json::to_writer;
use std::fs::File;
use std::io::Error;

#[derive(Serialize, Deserialize)]
pub struct Export {
    pub dataset: String,
    pub support: u64,
    pub max_depth: u64,
    pub nb_features: usize,
    pub timeout: f64,
    pub allow_discrepancy: bool,
    pub use_information_gain: bool,

    pub max_discrepancy: Option<usize>,
    pub discrepancy: Option<usize>,
    pub error: f64,
    pub accuracy: f64,
    pub cache_size: u64,
    pub has_timeout: bool,
    pub recursion_count: usize,
    pub duration: u128,

    pub tree_depth: u64,
    pub tree: Tree,
}

impl Export {
    pub fn new() -> Export {
        Export {
            dataset: "".to_string(),
            support: 0,
            error: 0.0,
            cache_size: 0,
            timeout: 0.0,
            has_timeout: false,
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
                current_depth: 0,
            },
            accuracy: 0.0,
            max_depth: 0,
            nb_features: 0,
            recursion_count: 0,
            duration: 0
        }
    }

    pub fn to_json(&self, filename: String) -> Result<(), Error> {
        if let Err(e) = to_writer(&File::create(filename)?, &self) {
            println!("File Creating error: {}", e.to_string());
        };
        Ok(())
    }
}
