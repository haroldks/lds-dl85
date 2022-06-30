use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use bit_vec::BitVec;

use crate::cache::trie::Trie;
use crate::mining::types_def::Item;
use crate::solution::tree::Tree;

pub fn get_solution_tree(cache: Trie) -> (Tree, Trie, u64) {
    if !cache.is_done || cache.root.data.test == <usize>::MAX {
        (Tree::new(None), cache, 0)
    } else {
        let best_attribute = cache.root.data.test;
        let mut tree = Tree::new(Some(best_attribute));
        tree.current_depth = 0;
        tree.error = Option::from(cache.root.data.node_error);
        let branches = vec![vec![(best_attribute, false)], vec![(best_attribute, true)]];

        let data = get_sub_tree(branches[0].clone(), cache, 0);
        tree.left = Some(Box::new(data.1));

        let data = get_sub_tree(branches[1].clone(), data.0, 0);
        tree.right = Some(Box::new(data.1));

        (tree, data.0, data.2 + 1)
    }
}

fn get_sub_tree(parent: Vec<Item>, mut cache: Trie, depth: u64) -> (Trie, Tree, u64) {
    let parent_node = cache.get(&parent).unwrap();

    let len = parent.len();

    let parent_attribute = parent[len - 1].0;
    let parent_node_data = parent_node.data;
    let mut final_tree = Tree::new(Some(parent_node_data.test));

    final_tree.leaf_error = parent_node_data.leaf_error;
    final_tree.max_class = parent_node_data.max_class;
    final_tree.error = Some(parent_node_data.node_error);
    final_tree.current_depth = parent_node_data.current_depth + 1;

    if parent_attribute == parent_node_data.test || parent_node_data.is_leaf {
        final_tree.root = None;
        final_tree.is_leaf = true;
        final_tree.error = Some(parent_node_data.node_error);
        (cache, final_tree, depth)
    } else {
        let mut item_set_vec = parent.clone();
        let checker = item_set_vec
            .iter()
            .filter(|it| it.0 == parent_node_data.test)
            .collect::<Vec<&Item>>();
        if checker.len() > 0 {
            final_tree.root = None;
            final_tree.is_leaf = true;
            final_tree.error = Some(parent_node_data.node_error);
            (cache, final_tree, depth)
        } else {
            item_set_vec.push((parent_node_data.test, false)); //left
            item_set_vec.sort_unstable();

            let data = get_sub_tree(item_set_vec, cache, depth);
            let left_tree_infos = (data.1.is_leaf, data.1.max_class);
            final_tree.left = Some(Box::new(data.1));

            let curr_depth = data.2 + 1;

            let mut item_set_vec = parent.clone();
            item_set_vec.push((parent_node_data.test, true)); //right
            item_set_vec.sort_unstable();

            let data = get_sub_tree(item_set_vec, data.0, depth);
            let right_tree_infos = (data.1.is_leaf, data.1.max_class);
            if left_tree_infos.0 && right_tree_infos.0 && left_tree_infos.1 == right_tree_infos.1 {
                final_tree.left = None;
                final_tree.root = None;
                final_tree.is_leaf = true;
            } else {
                final_tree.right = Some(Box::new(data.1));
            }

            (data.0, final_tree, max(data.2 + 1, curr_depth))
        }
    }
}

pub fn predict(transactions: Vec<BitVec>, tree: Tree) -> Vec<usize> {
    let mut y_pred: Vec<usize> = Vec::new();
    for transaction in transactions {
        let mut clone_tree = tree.clone();
        while !clone_tree.is_leaf {
            let root = clone_tree.root.unwrap();
            if !transaction[root] {
                clone_tree = *clone_tree.left.unwrap();
            } else {
                clone_tree = *clone_tree.right.unwrap();
            }
        }
        y_pred.push(clone_tree.max_class);
    }
    return y_pred;
}

pub fn get_data_as_transactions_and_target(
    filename: String,
) -> Result<(Vec<BitVec>, Vec<usize>), Error> {
    let input = File::open(&filename)?; //Error Handling for missing filename
    let buffered = BufReader::new(input); // Buffer for the file
    let data_lines: Vec<String> = buffered.lines().map(|x| x.unwrap()).collect();
    let nattributes = data_lines[0]
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .len()
        - 1;
    let ntransactions = data_lines.len();
    let mut inputs = vec![BitVec::from_elem(nattributes, false); ntransactions];
    let mut target = vec![];
    for (i, line) in data_lines.iter().enumerate() {
        let line = line.split_ascii_whitespace().collect::<Vec<&str>>();
        for (j, l) in line.iter().enumerate() {
            match j {
                0 => target.push(l.parse::<usize>().unwrap()),
                _ => inputs[i].set(j - 1, l == &"1"),
            }
        }
    }
    return Ok((inputs, target));
}

pub fn confusion_matrix(y_test: Vec<usize>, y_pred: Vec<usize>, nclasses: usize) -> Vec<Vec<i32>> {
    let mut matrix = vec![vec![0; nclasses]; nclasses];
    let len = y_test.len();
    for i in 0..len {
        matrix[y_test[i]][y_pred[i]] += 1;
    }
    return matrix;
}

pub fn accuracy(y_test: Vec<usize>, y_pred: Vec<usize>) -> f64 {
    assert_eq!(y_test.len(), y_pred.len(), "Vectors length mismatch");

    let mut equal = vec![];
    for i in 0..y_pred.len() {
        equal.push(y_pred[i] == y_test[i]);
    }
    equal.iter().filter(|x| **x).count() as f64 / y_pred.len() as f64
}
