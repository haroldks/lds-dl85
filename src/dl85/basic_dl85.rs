use std::cmp::max;
use crate::cache::trie::{Trie, TrieEdges, TrieNode};
use crate::data::dt_chuncked::DataChuncked;
use crate::mining::its_ops_chunked::ItemsetOpsChunked;
use crate::mining::types_def::{Attribute, Item};
use crate::node::node::Node;

pub struct DL85<'a> {
    ntransactions: usize,
    nclasses: usize,
    nattributes: usize,
    min_support: u64,
    max_depth: u64,
    max_error: f64,
    time_limit: f64,
    cache: Trie,
    its_op: ItemsetOpsChunked<'a>,

}

impl<'a> DL85<'a> {
    pub fn new(min_support: u64, max_depth: u64, max_error: f64, time_limit: f64, cache: Trie, its_op: ItemsetOpsChunked) -> DL85 {
        DL85 {
            ntransactions: its_op.data.ntransactions,
            nclasses: its_op.data.nclasses,
            nattributes: its_op.data.nattributes,
            min_support,
            max_depth,
            max_error,
            time_limit,
            cache,
            its_op,

        }
    }


    pub fn run(&mut self) {
        let mut candidates_list: Vec<Attribute> = Vec::new();
        if self.min_support == 1 {
            candidates_list = (0..self.nattributes).collect::<Vec<Attribute>>(); // TODO: Information Gain ???
        } else {
            for i in 0..self.nattributes {
                if self.its_op.temp_union(&(i, false)) >= self.min_support as usize && self.its_op.temp_union(&(i, true)) >= self.min_support as usize {
                    candidates_list.push(i);
                }
            }
        }

        println!("{:?}", candidates_list);

        let empty_itemset: Vec<Item> = vec![];
        self.recursive_dl85(&empty_itemset, usize::MAX, candidates_list, self.max_error, 0);
    }

    fn check_if_stop_condition_reached(node: &mut &mut TrieNode, upper_bond: f64,  min_support: u64,  current_support: u64, depth: u64, max_depth: u64) { // TODO: Here we check if the node already exists. If not we create new one and return his address

        if node.is_new {
            println!("This is a new node");
            // return node;
        }

        else if depth == max_depth || current_support < (2 * min_support)  as u64 {
            println!("We have reached the either the max depth or there is not enough elements to make a split with an upper_bound = {} and a leaf_error = {}", upper_bond, node.data.leaf_error);
            node.data.node_error = node.data.leaf_error;
            node.is_new = false;
            // return node;
        }
        if upper_bond <= node.data.lower_bound{
            println!("Infeasible solution because upper_bound = {} <= saved_lower_bound = {}", upper_bond, node.data.lower_bound);
            // return node;
        }

        if node.data.leaf_error == node.data.lower_bound {
            println!("We can not split anymore because we are at a leaf and a pure one = {} and a leaf_error = {}", upper_bond, node.data.leaf_error);
            node.data.node_error = node.data.leaf_error;
            node.is_new = false;
            // return node;
        }
        println!("Should not come here at any case")
        // return node;

    }


    fn create_cache_emplacement_for_current_its(cache_ref: &'a mut Trie, item : &Item, depth: u64, current_its: &Vec<Item>) -> &'a mut TrieNode { //TODO:  Here we do the creation of the new cache emplacement and compute the error
        let mut node  = cache_ref.insert(current_its);
        node.data = Node::new(*item, depth);
        node



    }

    fn get_cached_node(cache: &'a mut Trie, itemset: &Vec<Item>) -> Option<&'a mut TrieNode> {
        let node = cache.get(itemset);
        return node;
    }


    fn recursive_dl85(&mut self, current_itemset: &Vec<Item>, last_attribute: Attribute, next_candidates: Vec<Attribute>, upper_bound: f64, depth: u64) -> TrieNode {

        let  mut cache_ref = &mut self.cache;



        // let mut cached_node = DL85::get_cached_node(&mut cache_ref, current_itemset);
        let mut cached_node = cache_ref.get(current_itemset);

        if cached_node.is_some() {
            println!("The node exists in the cache.");
            let mut node_ref = cached_node.as_mut().unwrap();
            let current_support = self.its_op.support() as u64;
            DL85::check_if_stop_condition_reached(&mut node_ref, upper_bound, self.min_support, current_support, depth, self.max_depth);
           // return *node_ref;
        }

        let mut cached_node = cached_node.as_mut().unwrap();


        let new_candidates = DL85::get_next_sucessors(&next_candidates, last_attribute, &mut self.its_op, self.min_support);

        if new_candidates.len() == 0 {
            cached_node.data.node_error = cached_node.data.leaf_error;
            cached_node.is_new = false;
            println!("There are not candidates. Node error becomes leaf error. upper_bound = {}, depth = {}", upper_bound, depth);
            //return cached_node;
        }

        for attribute in new_candidates {

            let first_lb = -1f64;
            let second_lb = -1f64;

            let items: Vec<Item> = vec![(attribute, false), (attribute, true)];
            let first_item_sup = self.its_op.union_cover(&items[0]); // Here current is supposed to be updated


            let mut child_item_set = current_itemset.clone();
            child_item_set.push(items[0]);

            
            
            let mut first_node = DL85::create_cache_emplacement_for_current_its(cache_ref, &items[0], depth, &self.its_op.current); // Error computation // cache_ref, item_ref, depth
            first_node.data = Node::new(items[0], depth);

            first_node.data.lower_bound = match first_node.is_new {
                true => {
                    if first_lb > first_node.data.lower_bound{
                        first_node.data.lower_bound
                    }
                    else {
                        first_lb
                    }
                },
                _ => {first_lb}
            };

            *first_node = self.recursive_dl85(&child_item_set, attribute, new_candidates.clone(), 0.0, depth + 1);

            let first_split_error = first_node.data.node_error;
            self.its_op.backtrack();






        }


        return  TrieNode {
            item: (0, false),
            data: Node {
                current_depth: 0,
                test: (0, false),
                leaf_error: 0.0,
                node_error: 0.0,
                lower_bound: 0.0
            },
            sub_trie: TrieEdges { edges: vec![] },
            is_new: false
        };
    }



    fn get_next_sucessors(candidates: &Vec<Attribute>, last_attribute: Attribute, its_op: &mut ItemsetOpsChunked, min_support: u64) -> Vec<Attribute> {
        let mut next_candidates = vec![];
        let current_support = its_op.support();

        for candidate in candidates {
            if *candidate == last_attribute{
                continue
            }
            let left_sup = its_op.temp_union(&(*candidate, false));
            let right_sup = current_support - left_sup;
            if left_sup >= min_support as usize && right_sup >= min_support as usize {
                next_candidates.push(*candidate)
            }
        }
        next_candidates
    }


    fn get_candidates_support(&mut self, candidates: &Vec<usize>) -> Vec<(usize, usize)> {
        let mut all_supports = vec![];
        for candidate in candidates {
            println!("{:?}", candidate);
            let items = vec![(*candidate, true), (*candidate, false)];
            let mut c_supports = vec![];
            for it in items {
                c_supports.push(self.its_op.union_cover(&it));
                self.its_op.backtrack();
            }
            all_supports.push((c_supports[0], c_supports[1]));
        }
        all_supports
    }
}
