use std::{env, fs, process};
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

fn main() { // TODO: Unit tests

    let do_test = false;

    if do_test {
        if let Err(e) = run_test() {
            println!("Error while Running test json : {}", e);
        };
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
    let output = algo.run(min_support, max_depth, max_error, time_limit, error_save_time, true, false, false, itemset_bitset_operations, cache);
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


#[derive(Serialize, Deserialize, Debug)]
struct TimeoutComp {
    timeout: Vec<f64>,
    normal_run: Vec<f64>,
    discrepancy_run: Vec<f64>,
}

impl TimeoutComp {
    pub fn new(timeout: Vec<f64>, normal_run: Vec<f64>, discrepancy_run: Vec<f64>) -> TimeoutComp {
        TimeoutComp {
            timeout,
            normal_run,
            discrepancy_run,
        }
    }


    pub fn to_json(&self, filename: String) -> Result<(), Error> {
        if let Err(e) = to_writer(&File::create(filename)?, &self) {
            println!("File Creating error: {}", e.to_string());
        };
        Ok(())
    }
}


fn run_test() -> Result<(), Error> {

    // Read File here and get data set as a list
    let min_support = 1;
    let max_depth = 9;
    //let use_info_gain = true;

    for info_gain in [true, false] {
        let files = fs::read_dir("datasets").unwrap();

        for file in files {
            let file = file?;
            let path = file.path().to_str().unwrap().to_string();
            let path_clone = path.clone();
            let filename: Vec<&str> = path_clone.split("/").collect();

            let right_split = &filename[1];
            let mut out = "results/".to_string();
            if !info_gain {
                out = "results_no_ig/".to_string();
            }
            out.push_str(right_split);
            let size = out.len();
            let out = &out[..size - 3];
            let mut out = out.to_string();
            out.push_str("json");


            let mut timeout_vec = vec![];
            let mut normal_run = vec![];
            let mut discrepancy_run = vec![];
            println!("Actual File: {:?}\n", path);
            for use_discrepancy in [false, true] {
                let mut timeout = 10.;
                while timeout <= 120. {
                    println!("Timeout\t:  {}", timeout);
                    println!("Using discrepancy\t:  {}\n", use_discrepancy);
                    let data = DataLong::new(path.clone()).unwrap();
                    let its_op = ItemsetOpsLong::new(&data);
                    let mut algo = DL85::new(its_op.get_infos());
                    let output = algo.run(min_support, max_depth, <f64>::MAX, timeout, -1, info_gain, use_discrepancy, false, its_op, Trie::new());
                    if use_discrepancy {
                        timeout_vec.push(timeout);
                        discrepancy_run.push(output.0.root.data.node_error);
                    } else {
                        normal_run.push(output.0.root.data.node_error);
                    }
                    timeout += 20.;
                }
            }
            let infos = TimeoutComp::new(timeout_vec, normal_run, discrepancy_run);
            println!("File : {}", out);
            if let Err(e) = infos.to_json(out.to_string()) {
                println!("Error while creating json : {}", e);
            };
        }
    }
    Ok(())
}








