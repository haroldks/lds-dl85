use crate::cache::trie::*;
use crate::data::dt_chuncked::*;
use crate::dl85::basic_dl85::DL85;
use crate::mining::its_ops_chunked::ItemsetOpsChunked;
use crate::mining::itemset_bitvector_trait::ItemsetBitvector;
use crate::solution::solution::{accuracy, confusion_matrix, get_data_as_transactions_and_target, get_solution_tree, predict};
use std::{env, process};
use std::time::Instant;
use crate::config::Config;
use crate::data::dt::Data;
use crate::data::dt_longed::DataLong;
use crate::data::dl_test::DataLongTest;
use crate::mining::its_ops::ItemsetOps;

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

    let a: u64 = 8;
    let mask = 1u64 << 3u64;
    println!("{:b}", a ^ mask );
    println!("{:b}", a ^ mask ^ mask);

    println!("Everything is ok..\n");
    let filename = config.filename;

    let time = Instant::now();
    let data = DataChunked::new(filename.clone()).unwrap();
    println!("Chunked {:?} milliseconds", time.elapsed().as_millis());

    let time = Instant::now();
    let datac = DataLong::new(filename.clone()).unwrap();
    println!("Data {:?} ", datac.target);
    println!("Long {:?} milliseconds", time.elapsed().as_millis());

    let time = Instant::now();
    let d = Data::new(filename.clone()).unwrap();
    println!("Base {:?} milliseconds", time.elapsed().as_millis());



    let itemset_bitset_operations = ItemsetOpsChunked::new(&data);
    let its = ItemsetOps::new(&d);
    let cache = Trie::new();

    // Algorithms parameters
    let min_support = config.min_support;
    let max_depth = config.max_depth;
    let max_error = config.max_error;
    let time_limit = config.time_limit;
    let error_save_time = config.error_save_time;


    let mut algo = DL85::new(its.get_infos());

    print!("We start the run.. \n");
    let output = algo.run(min_support, max_depth, max_error, time_limit, error_save_time, its, cache);
    let data = get_solution_tree(output.0);
    let dd = get_data_as_transactions_and_target(filename.clone()).unwrap();
    println!("Tree: {:?}", data.0);
    let y_pred = predict(dd.0.clone(), data.0.clone());

    println!("Accuracy: {:?}", accuracy(dd.1.clone(), y_pred.clone()));
    println!("Confusion Matrix: {:?}", confusion_matrix(dd.1, y_pred, 2));
}


