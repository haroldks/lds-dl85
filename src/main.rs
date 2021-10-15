use crate::cache::trie::*;
use crate::data::dt_chuncked::*;
use crate::dl85::basic_dl85::DL85;
use crate::mining::its_ops_chunked::ItemsetOpsChunked;
use crate::solution::solution::{accuracy, confusion_matrix, get_data_as_transactions_and_target, get_solution_tree, predict};
use crate::tree::Tree;

mod mining;
mod data;
mod cache;
mod node;
mod dl85;
mod tree;
mod solution;

fn main() { // TODO: Unit tests

    let filename = "datasets/anneal.txt".to_string();

    let datac = DataChuncked::new(filename.clone()).unwrap();
    let its_opsd = ItemsetOpsChunked::new(&datac, None, None, datac.ntransactions, false, datac.data[0].len());
    let mut algo = DL85::new(1, 4, <f64>::MAX, 0., Trie::new(), its_opsd);
    let output = algo.run();
    let data = get_solution_tree(output.0);
    let dd = get_data_as_transactions_and_target(filename.clone()).unwrap();
    println!("Tree: {:?}", data.0);
    let y_pred = predict(dd.0.clone(), data.0.clone());

    println!("Accuracy: {:?}", accuracy(dd.1.clone(), y_pred.clone()));
    println!("{:?}", y_pred);
    println!("Confusion Matrix: {:?}", confusion_matrix(dd.1, y_pred, 2));
}


