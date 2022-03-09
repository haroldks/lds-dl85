use crate::cli::Cli;
use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::{process};

use crate::cache::trie::*;
use crate::data::dt_longed::DataLong;
use crate::dl85::basic_dl85::DL85;
use crate::mining::itemset_bitvector_trait::ItemsetBitvector;
use crate::mining::its_ops_long::ItemsetOpsLong;
use crate::solution::solution::{
    accuracy, confusion_matrix, get_data_as_transactions_and_target, get_solution_tree, predict,
};

mod cache;
mod cli;
mod data;
mod dl85;
mod experiments;
mod mining;
mod node;
mod solution;

fn run_from_conf(cli: Cli) -> Result<(), Box<dyn Error>> {
    let filename = cli.input.unwrap().clone();
    let dataset = DataLong::new(filename.clone())?;
    let operator = ItemsetOpsLong::new(&dataset);
    let cache = Trie::new();
    let mut model = DL85::new(operator.get_infos());
    println!("--------------------- Run start ---------------------  \n");
    let output = model.run(
        cli.support.unwrap(),
        cli.depth.unwrap(),
        cli.error,
        cli.timeout,
        cli.log_error_time,
        cli.use_information_gain,
        cli.allow_discrepancy,
        false,
        operator,
        cache,
    );
    println!("--------------------- Run over. --------------------- \n");

    println!("\n--------------------- Metrics ---------------------");
    println!("Cache Size : {:?} Nodes", output.0.cachesize);
    println!("Tree Error : {:?} ", output.0.root.data.node_error);
    let solution_tuple = get_solution_tree(output.0);
    let metrics = get_data_as_transactions_and_target(filename.clone()).unwrap();

    let y_pred = predict(metrics.0.clone(), solution_tuple.0.clone());
    println!(
        "Accuracy: {:?}",
        accuracy(metrics.1.clone(), y_pred.clone())
    );
    println!(
        "Confusion Matrix: {:?}",
        confusion_matrix(metrics.1, y_pred, 2)
    );

    println!("\n--------------------- Tree ---------------------");
    println!("Depth: {:?}", solution_tuple.2);
    println!("Tree: {:?}", solution_tuple.0);

    if let Some(output) = cli.output {
        println!("Output path was given. Saving the tree to: {}.", output);
        if let Err(e) = solution_tuple.0.to_json(output) {
            println!("Error while creating the tree json file : {}", e);
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cli = Cli::parse();

    if let Some(config_path) = cli.config.as_deref() {
        let file = File::open(config_path)?;
        let reader = BufReader::new(file);
        cli = serde_json::from_reader(reader)?;
    }

    if cli.input.is_none() || cli.support.is_none() || cli.depth.is_none() {
        println!("Missing parameters");
        process::exit(1);
    }

    if let Err(e) = run_from_conf(cli){
        println!("Error {}, while running from the configuration", e);
    };
    Ok(())
}
