use crate::cache::trie::*;
use crate::data::dt_chuncked::*;
use crate::dl85::basic_dl85::DL85;
use crate::mining::its_ops_chunked::ItemsetOpsChunked;
use crate::solution::solution::{compute_metrics, get_data_as_transactions_and_target, get_solution_tree, predict};
use crate::tree::Tree;

mod mining;
mod data;
mod cache;
mod node;
mod dl85;
mod tree;
mod solution;

fn main() { // TODO: Unit tests

    let filename = "datasets/tic-tac-toe.txt".to_string();

    let datac = DataChuncked::new(filename.clone()).unwrap();
    let its_opsd = ItemsetOpsChunked::new(&datac, None, None, datac.ntransactions, false, datac.data[0].len());
    let mut algo = DL85::new(30, 4, 1000., 0., Trie::new(), its_opsd);
    let output = algo.run();
    let data = get_solution_tree(output.0);
    let dd = get_data_as_transactions_and_target(filename.clone()).unwrap();
    println!("Tree: {:?}", data.0);
    let y_pred = predict(dd.0.clone(), data.0.clone());
    let mut a = vec![];
    for i in 0..y_pred.len() {
        a.push((y_pred[i] == dd.1[i]));
    }
    println!("Accuracy: {:?}", a.iter().filter(|x| **x).count() as f32 / dd.1.len() as f32);
    println!("{:?}", y_pred);
    println!("Confusion Matrix: {:?}", compute_metrics(dd.1, y_pred, 2));
}


