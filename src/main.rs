

use mining::types_def::*;

use crate::cache::trie::*;

use crate::data::dt_chuncked::*;

use crate::dl85::basic_dl85::DL85;

use crate::mining::its_ops_chunked::ItemsetOpsChunked;

use crate::tree::Tree;

mod mining;

mod data;
mod cache;
mod node;
mod dl85;
mod tree;


fn main() { // TODO: Unit tests

    let datac = DataChuncked::new("datasets/heart-cleveland.txt".to_string()).unwrap();
    let its_opsd = ItemsetOpsChunked::new(&datac, None, None, datac.ntransactions, false, datac.data[0].len());
    let mut algo = DL85::new(10, 3, 200., 1000., Trie::new(), its_opsd);
    let output = algo.run();
    let data = get_solution_tree(output.0);
    println!("{:?}", data.0);
}

fn get_solution_tree(cache: Trie) -> (Tree, Trie) {
    if !cache.is_done || cache.root.data.test == <usize>::MAX {
        (Tree::new(<usize>::MAX), cache)
    } else {
        let best_attribute = cache.root.data.test;
        let mut tree = Tree::new(best_attribute);
        let branches = vec![vec![(best_attribute, false)], vec![(best_attribute, true)]];

        let data = get_sub_tree(branches[0].clone(), cache);
        tree.left.push(data.1);

        let data = get_sub_tree(branches[1].clone(), data.0);
        tree.right.push(data.1);

        (tree, data.0)
    }
}

fn get_sub_tree(parent: Vec<Item>, mut cache: Trie) -> (Trie, Tree) {

    let parent_node = cache.get(&parent).unwrap();
    let len = parent.len();

    let parent_attribute = parent[len - 1].0;
    let parent_node_data = parent_node.data;
    let mut final_tree = Tree::new(parent_node_data.test);
    if parent_attribute == parent_node_data.test || parent_node_data.is_leaf {
        final_tree.is_leaf = true;
        final_tree.max_class = parent_node_data.max_class;
        final_tree.error = Option::from(parent_node_data.node_error);
        (cache, final_tree)
    } else {
        let mut item_set_vec = parent.clone();
        item_set_vec.push((parent_node_data.test, false)); //left
        item_set_vec.sort_unstable();

        let data = get_sub_tree(item_set_vec, cache);
        final_tree.left.push(data.1);

        let mut item_set_vec = parent.clone();
        item_set_vec.push((parent_node_data.test, true)); //right
        item_set_vec.sort_unstable();

        let data = get_sub_tree(item_set_vec, data.0);
        final_tree.right.push(data.1);
        (data.0, final_tree)
    }
}
