use crate::cache::trie::*;
use crate::data::dt_chuncked::*;
use crate::dl85::basic_dl85::DL85;
use crate::mining::its_ops_chunked::ItemsetOpsChunked;
use crate::solution::solution::{accuracy, confusion_matrix, get_data_as_transactions_and_target, get_solution_tree, predict};

mod mining;
mod data;
mod cache;
mod node;
mod dl85;
mod solution;

fn main() { // TODO: Unit tests

    let filename = "datasets/anneal.txt".to_string();

    let data = DataChuncked::new(filename.clone()).unwrap();
    let itemset_biset_operations = ItemsetOpsChunked::new(&data, None, None, data.ntransactions, false, data.data[0].len());
    let cache = Trie::new();

    // Algorithms parameters
    let min_support = 100;
    let max_depth = 4;
    let max_error = <f64>::MAX;
    let time_limit = 10.;
    let error_save_time = 2;


    let mut algo = DL85::new(itemset_biset_operations.get_infos());
    let output = algo.run(min_support, max_depth, max_error, time_limit, error_save_time, itemset_biset_operations, cache);
    let data = get_solution_tree(output.0);
    let dd = get_data_as_transactions_and_target(filename.clone()).unwrap();
    println!("Tree: {:?}", data.0);
    let y_pred = predict(dd.0.clone(), data.0.clone());

    println!("Accuracy: {:?}", accuracy(dd.1.clone(), y_pred.clone()));
    println!("{:?}", y_pred);
    println!("Confusion Matrix: {:?}", confusion_matrix(dd.1, y_pred, 2));
}


