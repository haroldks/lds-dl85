use std::{env, fs, process};
use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use serde_json::to_writer;

use crate::cache::trie::*;
use crate::config::Config;
use crate::data::dt::Data;
use crate::data::dt_chuncked::*;
use crate::data::dt_longed::DataLong;
use crate::dl85::basic_dl85::DL85;
use crate::experiments::experiments::{Test, TestConfig};
use crate::mining::itemset_bitvector_trait::ItemsetBitvector;
use crate::mining::its_ops::ItemsetOps;
use crate::mining::its_ops_chunked::ItemsetOpsChunked;
use crate::mining::its_ops_long::ItemsetOpsLong;
use crate::solution::solution::{accuracy, confusion_matrix, get_data_as_transactions_and_target, get_solution_tree, predict};

mod mining;
mod data;
mod cache;
mod node;
mod dl85;
mod solution;
mod config;
mod experiments;

fn main() { // TODO: Unit tests

    let do_test = true;

    if do_test {
        let mut test = Test::new();

        if let Err(e) = test.run(TestConfig {
            min_support: 1,
            max_depth: 9,
            max_error: <f64>::MAX,
            timeouts: None,
            output_folders: [(true, "tests/ig_results/".to_string()), (false, "tests/no_ig_results/".to_string())],
        }) {
            println!("Error while Running test json : {}", e);
        }


        process::exit(0);
    }


    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    println!("Everything is ok..\n");
    let filename = config.filename;

    let time = Instant::now();
    let datac = DataLong::new(filename.clone()).unwrap();
    println!("Long {:?} milliseconds", time.elapsed().as_millis());

    let itemset_bitset_operations = ItemsetOpsLong::new(&datac);

    let cache = Trie::new();

    // Algorithms parameters
    let min_support = config.min_support;
    let max_depth = config.max_depth;
    let max_error = config.max_error;
    let time_limit = config.time_limit;
    let error_save_time = config.error_save_time;


    let mut algo = DL85::new(itemset_bitset_operations.get_infos());


    print!("We start the run.. \n");
    let output = algo.run(min_support, max_depth, max_error, time_limit, error_save_time, true, true, false, itemset_bitset_operations, cache);
    println!("Cache Size : {:?} Nodes", output.0.cachesize);
    println!("Tree Error : {:?} ", output.0.root.data.node_error);

    let data = get_solution_tree(output.0);
    let dd = get_data_as_transactions_and_target(filename.clone()).unwrap();
    println!("Tree: {:?}", data.0);
    if let Err(e) = data.0.to_json("tree.json".to_string()) {
        println!("Error while creating json : {}", e);
    };
    println!("Depth: {:?}", data.2);
    let y_pred = predict(dd.0.clone(), data.0.clone());

    println!("Accuracy: {:?}", accuracy(dd.1.clone(), y_pred.clone()));
    println!("Confusion Matrix: {:?}", confusion_matrix(dd.1, y_pred, 2));
}








