use crate::cache::trie::*;
use crate::data::dt_chuncked::*;
use crate::dl85::basic_dl85::DL85;
use crate::mining::its_ops_chunked::ItemsetOpsChunked;
use crate::solution::solution::{accuracy, confusion_matrix, get_data_as_transactions_and_target, get_solution_tree, predict};
use std::{env, process};
use crate::config::Config;

mod mining;
mod data;
mod cache;
mod node;
mod dl85;
mod solution;
mod config;

fn main() { // TODO: Unit tests

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Everything is ok..\n");

    let filename = config.filename;
    let data = DataChuncked::new(filename.clone()).unwrap();
    let itemset_biset_operations = ItemsetOpsChunked::new(&data, None, None, data.ntransactions, false, data.data[0].len());
    let cache = Trie::new();

    // Algorithms parameters
    let min_support = config.min_support;
    let max_depth = config.max_depth;
    let max_error = config.max_error;
    let time_limit = config.time_limit;
    let error_save_time = config.error_save_time;


    let mut algo = DL85::new(itemset_biset_operations.get_infos());

    print!("We start the run.. \n");
    let output = algo.run(min_support, max_depth, max_error, time_limit, error_save_time, itemset_biset_operations, cache);
    let data = get_solution_tree(output.0);
    let dd = get_data_as_transactions_and_target(filename.clone()).unwrap();
    println!("Tree: {:?}", data.0);
    let y_pred = predict(dd.0.clone(), data.0.clone());

    println!("Accuracy: {:?}", accuracy(dd.1.clone(), y_pred.clone()));
    println!("Confusion Matrix: {:?}", confusion_matrix(dd.1, y_pred, 2));
}


