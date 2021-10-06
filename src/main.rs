use crate::cache::trie::*;
use crate::data::dt_chuncked::*;
use crate::dl85::basic_dl85::DL85;
use crate::mining::its_ops_chunked::ItemsetOpsChunked;
use crate::solution::solution::get_solution_tree;


mod mining;
mod data;
mod cache;
mod node;
mod dl85;
mod tree;
mod solution;

fn main() { // TODO: Unit tests

    let datac = DataChuncked::new("datasets/tic-tac-toe.txt".to_string()).unwrap();
    let its_opsd = ItemsetOpsChunked::new(&datac, None, None, datac.ntransactions, false, datac.data[0].len());
    let mut algo = DL85::new(100, 5, 500., 2., Trie::new(), its_opsd);
    let output = algo.run();
    let data = get_solution_tree(output.0);
    println!("{:?}", data.0);
}
